#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, Addr, CosmosMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-alliance";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit(dep) => try_delegate(dep.del_addr, dep.val_addr, info.funds),
        ExecuteMsg::UndelegateMsg {
            del_addr,
            val_addr,
            amount,
        } => try_undelegate(del_addr, val_addr, amount),
        ExecuteMsg::RedelegateMsg {
            del_addr,
            src_val_addr,
            dst_val_addr,
            amount,
        } => try_redelegate(del_addr, src_val_addr, dst_val_addr, amount),
        ExecuteMsg::ClaimRewardsMsg {
            del_addr,
            val_addr,
            demom,
        } => try_claim_rewards(del_addr, val_addr, demom),
    }
}

fn try_delegate(del_addr: Addr, val_addr: String, funds: Vec<Coin>)-> Result<Response, ContractError>{
    if funds.len() != 1 {
        return Err(ContractError::InvalidFunds{ });
    }
    let bin = MsgDelegate {
        delegator_address: del_addr,
        validator_address: val_addr,
        amount: funds[0]
    }
    .encode_to_vec();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(bin),
    };
    Ok(Response::default())
}

fn try_undelegate(del_addr: Addr, val_addr: String, amount: Coin)-> Result<Response, ContractError>{
    Ok(Response::default())
}

fn try_redelegate(del_addr: Addr, src_val_addr: String, dst_val_addr: String, amount: Coin)-> Result<Response, ContractError>{
    Ok(Response::default())
}

fn try_claim_rewards(del_addr: Addr, val_addr: String, demom: String)-> Result<Response, ContractError>{
    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {}
