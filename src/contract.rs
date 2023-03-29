use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, NFTResponse};
use crate::state::{MINTED_NFTS, CFG, Cfg};
use cosmwasm_std::{SubMsg, WasmMsg, Reply, StdError};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Addr, Order::Ascending};

use cw2::set_contract_version;
use cw721_metadata_onchain::{
    InstantiateMsg as Cw721InstantiateMsg,
};
// use terra_proto_rs::{
    // alliance::alliance::{MsgClaimDelegationRewards, MsgRedelegate, MsgUndelegate},
    // cosmos::base::v1beta1::Coin as CosmosNativeCoin,
    // cosmos::staking::v1beta1::MsgDelegate,
    // traits::Message,
// };

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-alliance-hub";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_REPLY_ID: u64 = 1;

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
    let cw721_instantiate_submsg = SubMsg::reply_always(WasmMsg::Instantiate {
        code_id: msg.cw721_code_id,
        msg: to_binary(&cw721_instantiate_msg)?,
        funds: info.funds.clone(),
        label: msg.cw721_collection.name.clone(),
        admin: Some(info.sender.to_string()),
    }, INSTANTIATE_REPLY_ID);

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
            return event.ty == "instantiate"
        })
        .ok_or_else(|| StdError::generic_err(evt))?;

    let mut evts = String::new();
    /* Find the contract_address from _instantiate_contract event*/
    let contract_address = &event
        .attributes
        .iter()
        .find(|attr| {
            evts.push_str(&String::from(" ").add(&attr.key.clone()));
            return attr.key == "_contract_address"}
        )
        .ok_or_else(|| StdError::generic_err(evts))?
        .value;

    /* Update the state of the contract adding the new generated nft_contract_addr */
    CFG.save(deps.storage, &Cfg {
        nft_contract_addr: Addr::unchecked(contract_address.clone()),
    })?;

    Ok(Response::new()
        .add_attribute("method", "instantiate_reply")
        .add_attribute("nft_contract_address", contract_address))
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
        ExecuteMsg::MsgClaimRewards { nft_address } => try_claim_rewards(env, info, deps, nft_address),
        ExecuteMsg::MsgRedeemUndelegation { nft_address } => try_redeem_undelegation(env, info, deps, nft_address),
    }
}

fn try_delegate(
    _env: Env,
    _info: MessageInfo,
    _deps: DepsMut,
) -> Result<Response, ContractError> {

    Ok(Response::default())
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
    match msg {
        QueryMsg::ListNFTS { } => try_query_list_nfts(deps),
    }
    
}

fn try_query_list_nfts(deps: Deps) -> StdResult<Binary> {
    let res: Vec<NFTResponse> = MINTED_NFTS
        .range(deps.storage, None, None, Ascending)
        .collect::<StdResult<Vec<(Addr, bool)>>>()?
        .into_iter()
        .map(|(addr, undelegated)| NFTResponse::new(addr, undelegated))
        .collect();
    
    to_binary(&res)
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {}
