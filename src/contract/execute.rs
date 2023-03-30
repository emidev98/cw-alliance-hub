use std::ops::Add;

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{DisplayStatus, DisplayType, CFG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, DepsMut, Env, MessageInfo,
    QueryRequest, Response, AllValidatorsResponse, Coin, CosmosMsg, 
    SubMsg, WasmMsg, StakingQuery, Validator, WasmQuery, QuerierWrapper,
};

use cw721_metadata_onchain::{
    Metadata as CW721Metadata, 
    MintMsg as Cw721MintMsg,  
    ExecuteMsg as Cw721ExecuteMsg, 
    Trait as CW721Trait,
    QueryMsg as CW721Query
};
use cw721::AllNftInfoResponse;
use terra_proto_rs::traits::Message;
use terra_proto_rs::{
    alliance::alliance::MsgDelegate,
    alliance::alliance::MsgClaimDelegationRewards,
    cosmos::base::v1beta1::Coin as CosmosNativeCoin,
};

use crate::contract::constants::MINT_NFT_REPLY;

use super::constants::DEFAULT_DELIMITER;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // TODO send claim_rewards to the user if he has anys
    match msg {
        ExecuteMsg::MsgDelegate {} => try_delegate(env, info, deps),
        ExecuteMsg::MsgUndelegate { token_id } => try_undelegate(env, info, deps, token_id),
        ExecuteMsg::MsgRedelegate { token_id } => try_redelegate(env, info, deps, token_id),
        ExecuteMsg::MsgClaimRewards { token_id } => try_claim_rewards(env, info, deps, token_id),
        ExecuteMsg::MsgRedeemUndelegation { token_id } => try_redeem_undelegation(env, info, deps, token_id),
    }
}

fn try_delegate(env: Env, info: MessageInfo, deps: DepsMut) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let res: AllValidatorsResponse = deps
        .querier
        .query(&QueryRequest::Staking(StakingQuery::AllValidators {}))?;

    let msg_delegate = generate_delegate_msg(info.funds, env.clone(), res.validators);
    let msg_mint = generate_mint_msg(
        info.sender.clone().into(),
        env.block.height,
        cfg.minted_nfts.to_string(),
        msg_delegate.clone(),
    );

    let delegate_msgs: Vec<CosmosMsg> = msg_delegate
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
            msg: to_binary(&Cw721ExecuteMsg::Mint(msg_mint))?,
            funds: vec![],
        }, MINT_NFT_REPLY))
        .add_messages(delegate_msgs))
}

fn generate_delegate_msg( funds: Vec<Coin>, env: Env, validators: Vec<Validator>) -> Vec<MsgDelegate> {
    let mut vals_len = validators.len() as u64;

    funds.iter()
        .map(|coin| {
            let pseudorandom_index = get_pseudorandom(env.block.height, vals_len);
            let val = &validators[pseudorandom_index as usize];

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
) -> Cw721MintMsg<Option<CW721Metadata>> {
    let attributes = msg_delegate
        .iter()
        .map(|msg| {
            let unwrapped_coin = msg.amount.as_ref().unwrap();
            let value = unwrapped_coin.amount.clone().add(DEFAULT_DELIMITER).add(&unwrapped_coin.denom);
            let display_type = DisplayType {
                display_status: DisplayStatus::Delegated {},
                height: block_height,
            };

            CW721Trait {
                display_type: Some(display_type.to_string()),
                trait_type: msg.validator_address.to_string(),
                value: value,
            }
        })
        .collect::<Vec<CW721Trait>>();

    Cw721MintMsg::<Option<CW721Metadata>> {
        token_id: nft_id.clone(),
        owner: sender,
        token_uri: None,
        extension: Some(CW721Metadata {
            name: Some(String::from("Alliance NFT #").add(&nft_id)),
            attributes: Some(attributes),
            description: Some(block_height.to_string()),
            image: None,
            image_data: None,
            external_url: None,
            background_color: None,
            animation_url: None,
            youtube_url: None,
        }),
    }
}

fn get_pseudorandom(block_height: u64, max: u64) -> u64 {
    let seed: u64 = block_height % (max + 1);

    seed % max
}

fn try_undelegate(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _token_id: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}

fn try_redelegate(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _token_id: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}

fn try_claim_rewards(
    env: Env,
    info: MessageInfo,
    deps: DepsMut,
    token_id: String,
) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let query_res = query_all_nft_info(
        deps.querier, 
        token_id.clone(), 
        cfg.nft_contract_addr.to_string()
    );
    if query_res.access.owner != info.sender.to_string() {
        return Err(ContractError::UnauthorizedNFTOwnere(
            query_res.access.owner, 
            info.sender.to_string()
        ));
    }
    let attrs = query_res.info.extension.attributes.unwrap_or(vec![]);
    if attrs.len() == 0 {
        return Err(ContractError::NoDelegationsFound(token_id))
    }

    let msgs = attrs.iter()
        .map(|attr| {
            let coin = attr.value.split(DEFAULT_DELIMITER).collect::<Vec<&str>>();
            let msg = MsgClaimDelegationRewards {
                delegator_address: env.contract.address.to_string(),
                validator_address: attr.trait_type.clone(),
                denom: coin[1].to_string(),
            }.encode_to_vec();

            CosmosMsg::Stargate {
                type_url: "/alliance.alliance.MsgClaimDelegationRewards".to_string(),
                value: Binary::from(msg),
            }
        })
        .collect::<Vec<CosmosMsg>>();

    Ok(Response::new()
        .add_attribute("action", "claim_rewards")
        .add_attribute("sender", info.sender.to_string())
        .add_messages(msgs))
}

fn query_all_nft_info(querier: QuerierWrapper, token_id: String, contract_addr: String) -> AllNftInfoResponse<CW721Metadata> {
    let msg = to_binary(&CW721Query::AllNftInfo {
        token_id,
        include_expired: None,
    }).unwrap();
    
    let res: AllNftInfoResponse<CW721Metadata> = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr,
        msg
    })).unwrap();

    res
}

fn try_redeem_undelegation(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _token_id: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}
