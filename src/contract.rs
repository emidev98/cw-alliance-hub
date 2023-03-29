use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Cfg, DisplayStatus, DisplayType, CFG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdResult,
};
use cosmwasm_std::{
    AllValidatorsResponse, Coin, CosmosMsg, Reply, 
    StdError, SubMsg, WasmMsg, StakingQuery,
};

use cw2::set_contract_version;
use cw721_metadata_onchain::{
    InstantiateMsg as Cw721InstantiateMsg,
    Metadata as CW721Metadata, 
    MintMsg as Cw721MintMsg, 
    Trait as Cw721Trait,
};
use terra_proto_rs::traits::Message;
use terra_proto_rs::{
    alliance::alliance::MsgDelegate,
    cosmos::base::v1beta1::Coin as CosmosNativeCoin,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-alliance-hub";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_REPLY_ID: u64 = 1;
const MINT_NFT: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cw721_instantiate_msg = Cw721InstantiateMsg {
        name: msg.cw721_collection.name.clone(),
        symbol: msg.cw721_collection.symbol.clone(),
        minter: info.sender.to_string(),
    };

    // Instantiate CW721 contract
    let cw721_instantiate_submsg = SubMsg::reply_always(
        WasmMsg::Instantiate {
            code_id: msg.cw721_code_id,
            msg: to_binary(&cw721_instantiate_msg)?,
            funds: info.funds.clone(),
            label: msg.cw721_collection.name.clone(),
            admin: Some(info.sender.to_string()),
        },
        INSTANTIATE_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(cw721_instantiate_submsg)
        .add_attribute("action", "instantiate_alliance_hub")
        .add_attribute("sender", info.sender)
        .add_attribute("cw721_label", msg.cw721_collection.name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        MINT_NFT => handle_mint_nft_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Unwrap the resoults of the instantiate submessage
    let result = msg
        .result
        .into_result()
        .map_err(|op| StdError::generic_err(op))?;

    let mut evt = String::new();
    /* Find the event type _instantiate_contract which contains the contract_address*/
    let event = result
        .events
        .iter()
        .find(|event| {
            evt.push_str(&String::from(" ").add(&event.ty.clone()));
            return event.ty == "instantiate";
        })
        .ok_or_else(|| StdError::generic_err(evt))?;

    let mut evts = String::new();
    /* Find the contract_address from _instantiate_contract event*/
    let contract_address = &event
        .attributes
        .iter()
        .find(|attr| {
            evts.push_str(&String::from(" ").add(&attr.key.clone()));
            return attr.key == "_contract_address";
        })
        .ok_or_else(|| StdError::generic_err(evts))?
        .value;

    /* Update the state of the contract adding the new generated nft_contract_addr */
    CFG.save(
        deps.storage,
        &Cfg {
            nft_contract_addr: Addr::unchecked(contract_address.clone()),
            minted_nfts: 0,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate_reply")
        .add_attribute("nft_contract_address", contract_address))
}

fn handle_mint_nft_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {

    // TODO: Whatever is necessary to handle the minting of the NFT
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::MsgDelegate {} => try_delegate(env, info, deps),
        ExecuteMsg::MsgUndelegate { nft_address } => try_undelegate(env, info, deps, nft_address),
        ExecuteMsg::MsgRedelegate { nft_address } => try_redelegate(env, info, deps, nft_address),
        ExecuteMsg::MsgClaimRewards { nft_address } => {
            try_claim_rewards(env, info, deps, nft_address)
        }
        ExecuteMsg::MsgRedeemUndelegation { nft_address } => {
            try_redeem_undelegation(env, info, deps, nft_address)
        }
    }
}

fn try_delegate(env: Env, info: MessageInfo, deps: DepsMut) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let res: AllValidatorsResponse = deps
        .querier
        .query(&QueryRequest::Staking(StakingQuery::AllValidators {}))?;

    let msg_delegate = generate_delegate_msg(info.funds, env.clone(), res);
    let msg_mint = generate_mint_msg(
        info.sender.clone().into(),
        env.block.height,
        (cfg.minted_nfts + 1).to_string(),
        msg_delegate.clone(),
    );

    let messages: Vec<CosmosMsg> = msg_delegate
        .iter()
        .map( |msg| {
            CosmosMsg::Stargate {
                type_url: "/alliance.alliance.MsgDelegate".to_string(),
                value: Binary::from(msg.encode_to_vec()),
            }
        })
        .collect();

    Ok(Response::new()
        .add_attribute("action", "delegate")
        .add_attribute("sender", info.sender.to_string())
        .add_submessage(SubMsg::reply_always(WasmMsg::Execute {
            contract_addr: cfg.nft_contract_addr.to_string(),
            msg: to_binary(&msg_mint)?,
            funds: vec![],
        }, MINT_NFT))
        .add_messages(messages))
}

fn generate_delegate_msg(
    funds: Vec<Coin>,
    env: Env,
    res: AllValidatorsResponse,
) -> Vec<MsgDelegate> {
    let mut vals_len = res.validators.len() as u64;

    funds.iter()
        .map(|coin| {
            let pseudorandom_index = get_pseudorandom(env.block.height, vals_len) - 1;
            let val = &res.validators[pseudorandom_index as usize];

            let msg_delegate = MsgDelegate {
                delegator_address: env.contract.address.to_string(),
                validator_address: val.address.to_string(),
                amount: Some(CosmosNativeCoin {
                    denom: coin.denom.to_string(),
                    amount: coin.amount.to_string(),
                }),
            };

            // Remove one of the indices to generate a new
            // pseudorandom index in the next iteration
            if vals_len > 1 {
                vals_len = vals_len - 1
            }

            msg_delegate
        })
        .collect::<Vec<MsgDelegate>>()
}

fn generate_mint_msg(
    sender: String,
    block_height: u64,
    nft_id: String,
    msg_delegate: Vec<MsgDelegate>,
) -> Cw721MintMsg<CW721Metadata> {
    let attributes = msg_delegate
        .iter()
        .map(|msg| {
            let unwrapped_coin = msg.amount.as_ref().unwrap();
            let value = unwrapped_coin.amount.clone().add(&unwrapped_coin.denom);
            let display_type = DisplayType {
                display_status: DisplayStatus::Delegated {},
                height: block_height,
            };

            let nft_trait = Cw721Trait {
                display_type: Some(display_type.to_string()),
                trait_type: msg.validator_address.to_string(),
                value: value,
            };

            nft_trait
        })
        .collect::<Vec<Cw721Trait>>();

    Cw721MintMsg::<CW721Metadata> {
        token_id: nft_id.clone(),
        owner: sender,
        token_uri: None,
        extension: CW721Metadata {
            name: Some(String::from("Alliance NFT #").add(&nft_id)),
            attributes: Some(attributes),
            description: Some(block_height.to_string()),
            image: None,
            image_data: None,
            external_url: None,
            background_color: None,
            animation_url: None,
            youtube_url: None,
        },
    }
}

fn get_pseudorandom(block_height: u64, max: u64) -> u64 {
    let min = 0;

    let range: u64 = max - min + 1;
    let seed: u64 = block_height % (range + 1);

    min + (seed % range)
}

fn try_undelegate(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _nft_address: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}

fn try_redelegate(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _nft_address: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}

fn try_claim_rewards(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _nft_address: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}

fn try_redeem_undelegation(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _nft_address: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(match msg {
        QueryMsg::GetConfig {} => to_binary(&CFG.load(deps.storage)?)?,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {}
