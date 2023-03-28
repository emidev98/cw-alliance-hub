use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, NFTResponse};
use crate::state::MINTED_NFTS;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Addr, Order::Ascending};

use cw2::set_contract_version;
// use terra_proto_rs::{
    // alliance::alliance::{MsgClaimDelegationRewards, MsgRedelegate, MsgUndelegate},
    // cosmos::base::v1beta1::Coin as CosmosNativeCoin,
    // cosmos::staking::v1beta1::MsgDelegate,
    // traits::Message,
// };
use cw721_metadata_onchain::{
    InstantiateMsg as Cw721InstantiateMsg,
    entry::{instantiate as cw721_instantiate}
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-alliance";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let res = cw721_instantiate(
        deps, 
        env.clone(), 
        info, 
        Cw721InstantiateMsg{
            name: "Alliance NFT".to_string(),
            symbol: "ALLIANCE".to_string(),
            minter: env.contract.address.to_string(),
        }
    ).or(Err(ContractError::NFTMintError {}))?;

    Ok(res)
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
