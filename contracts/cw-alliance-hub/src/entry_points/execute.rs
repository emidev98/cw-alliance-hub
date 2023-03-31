use std::ops::Add;

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{DisplayType, CFG};
use cosmwasm_std::{Empty, Timestamp};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, DepsMut, Env, MessageInfo,
    Response, Coin, CosmosMsg, SubMsg, WasmMsg, Validator,
};

use cw721_progressive_metadata::state::Extension;
use cw721_progressive_metadata::{
    ExecuteMsg as Cw721ExecuteDefaultMsg, 
    state:: {
        Trait as CW721Trait,
        Metadata as CW721Metadata, 
    }
};
use terra_proto_rs::{
    alliance::alliance::{
        MsgUndelegate,
        MsgDelegate,
        MsgClaimDelegationRewards
    },
    traits::Message,
    cosmos::base::v1beta1::Coin as CosmosNativeCoin,
};
use super::{
    query,
    constants::{
        MINT_NFT_REPLY_ID, 
        DEFAULT_DELIMITER,
        UPDATE_NFT_REPLY_ID
    }
};

type Cw721ExecuteMsg = Cw721ExecuteDefaultMsg<Extension, Empty>;

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
        ExecuteMsg::MsgStartUnbonding { token_id } => try_start_unbonding(env, info, deps, token_id),
        ExecuteMsg::MsgRedelegate { token_id } => try_redelegate(env, info, deps, token_id),
        ExecuteMsg::MsgClaimRewards { token_id } => try_claim_rewards(env, info, deps, token_id),
        ExecuteMsg::MsgRedeemBond { token_id } => try_redeem_bond(env, info, deps, token_id),
    }
}

fn try_delegate(env: Env, info: MessageInfo, deps: DepsMut) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let validators = query::all_validators(deps.querier);
    if validators.len() == 0 {
        return Err(ContractError::NoValidatorsFound {});
    }

    let msg_delegate = generate_delegate_msg(info.funds, env.clone(), validators);
    let msg_mint = generate_mint_msg(
        info.sender.clone().into(),
        env.block.time,
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
            contract_addr: cfg.nft_contract_addr.unwrap().to_string(),
            msg: to_binary(&msg_mint)?,
            funds: vec![],
        }, MINT_NFT_REPLY_ID))
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

            // Remove 1 of the index to generate a new
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
    block_height: Timestamp,
    nft_id: String,
    msg_delegate: Vec<MsgDelegate>,
) -> Cw721ExecuteMsg  {
    let attributes = msg_delegate
        .iter()
        .map(|msg| {
            let unwrapped_coin = msg.amount.as_ref().unwrap();
            let value = unwrapped_coin.amount.clone().add(DEFAULT_DELIMITER).add(&unwrapped_coin.denom);

            CW721Trait {
                display_type: DisplayType::Delegated.to_string(),
                trait_type: msg.validator_address.to_string(),
                timestamp: block_height,
                value: value,
            }
        })
        .collect::<Vec<CW721Trait>>();

    let msg = Cw721ExecuteMsg::Mint{
        token_id: nft_id.clone(),
        owner: sender,
        token_uri: None,
        extension: Some(CW721Metadata {
            name: Some(String::from("Alliance NFT #").add(&nft_id)),
            attributes: Some(attributes),
            description: None,
            image: None,
            image_data: None,
            external_url: None,
            background_color: None,
            animation_url: None,
            youtube_url: None,
        })};

    msg
}

fn get_pseudorandom(block_height: u64, max: u64) -> u64 {
    let seed: u64 = block_height % (max + 1);

    seed % max
}

fn try_start_unbonding(env: Env,info: MessageInfo,deps: DepsMut,token_id: String) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let query_res = query::all_nft_info(
        deps.querier, 
        token_id.clone(), 
        cfg.nft_contract_addr.unwrap().to_string()
    );
    if query_res.access.owner != info.sender.to_string() {
        return Err(ContractError::UnauthorizedNFTOwnere(
            query_res.access.owner, 
            info.sender.to_string()
        ));
    }
    let attrs = query_res.info.extension.attributes.clone().unwrap_or(vec![]);
    if attrs.len() == 0 {
        return Err(ContractError::NoDelegationsFound(token_id))
    }

    let msgs = attrs.iter()
        .map(|attr| {
            let coin = attr.value.split(DEFAULT_DELIMITER).collect::<Vec<&str>>();
            let msg = MsgUndelegate {
                delegator_address: env.contract.address.to_string(),
                validator_address: attr.trait_type.clone(),
                amount: Some(CosmosNativeCoin { 
                    amount: coin[0].to_string(),
                    denom: coin[1].to_string(), 
                }),
            }.encode_to_vec();

            CosmosMsg::Stargate {
                type_url: "/alliance.alliance.MsgUndelegate".to_string(),
                value: Binary::from(msg),
            }
        })
        .collect::<Vec<CosmosMsg>>();

    let unbonding_timestamp = env.block.time.plus_seconds(cfg.unbonding_seconds);
    let msg_update_nft = generate_update_nft_msg(
        query_res.info.extension,
        unbonding_timestamp,
        token_id,
    );

    Ok(Response::new()
        .add_attribute("action", "undelegate")
        .add_attribute("sender", info.sender.to_string())
        .add_messages(msgs)
        .add_submessage(SubMsg::reply_always(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&msg_update_nft)?,
            funds: vec![],
        }, UPDATE_NFT_REPLY_ID))
    )
}

fn generate_update_nft_msg(
    query_res: CW721Metadata,
    unbonding_timestamp: Timestamp,
    nft_id: String,
) -> Cw721ExecuteMsg  {
    let attrs = query_res
        .attributes
        .unwrap()
        .iter()
        .map(|attr| {

            CW721Trait {
                display_type: DisplayType::Unbonding.to_string(),
                timestamp: unbonding_timestamp,
                trait_type: attr.trait_type.clone(),
                value: attr.value.clone(),
            }
        })
        .collect::<Vec<CW721Trait>>();

    let msg = Cw721ExecuteMsg::UpdateExtension {
        token_id: nft_id.clone(),
        extension: Some(CW721Metadata {
            name: Some(String::from("Alliance NFT #").add(&nft_id)),
            attributes: Some(attrs),
            description: None,
            image: None,
            image_data: None,
            external_url: None,
            background_color: None,
            animation_url: None,
            youtube_url: None,
        })};

    msg
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
    let query_res = query::all_nft_info(
        deps.querier, 
        token_id.clone(), 
        cfg.nft_contract_addr.unwrap().to_string()
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


fn try_redeem_bond(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
    _token_id: String,
) -> Result<Response, ContractError> {
    // TODO Implement
    Ok(Response::default())
}
