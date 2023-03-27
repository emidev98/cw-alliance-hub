use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use terra_proto_build::alliance::alliance::{MsgUndelegate, MsgRedelegate, MsgClaimDelegationRewards};
use terra_proto_build::cosmos::base::v1beta1::Coin as CosmosNativeCoin;
use terra_proto_build::cosmos::staking::v1beta1::MsgDelegate;
use terra_proto_build::traits::Message;

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
        ExecuteMsg::Deposit(dep) => {
            try_delegate(dep.delegator_address, dep.validator_address, info.funds)
        }
        ExecuteMsg::UndelegateMsg {
            delegator_address,
            validator_address,
            amount,
        } => try_undelegate(delegator_address, validator_address, amount),
        ExecuteMsg::RedelegateMsg {
            delegator_address,
            validator_src_address,
            validator_dst_address,
            amount,
        } => try_redelegate(delegator_address, validator_src_address, validator_dst_address, amount),
        ExecuteMsg::ClaimRewardsMsg {
            delegator_address,
            validator_address,
            demom,
        } => try_claim_rewards(delegator_address, validator_address, demom),
    }
}

fn try_delegate(
    delegator_address: String,
    validator_address: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    if amount.len() != 1 {
        return Err(ContractError::InvalidFunds {});
    }

    let amount = amount.first().unwrap();
    let amount = Some(CosmosNativeCoin {
        denom: amount.denom.to_string(),
        amount: amount.amount.to_string(),
    });

    let bin = MsgDelegate {
        delegator_address,
        validator_address,
        amount,
    }
    .encode_to_vec();

    Ok(Response::default().add_message(CosmosMsg::Stargate {
        type_url: "/alliance.alliance.MsgDelegate".to_string(),
        value: Binary::from(bin),
    }))
}

fn try_undelegate(
    delegator_address: String,
    validator_address: String,
    amount: Coin,
) -> Result<Response, ContractError> {
    let amount = Some(CosmosNativeCoin {
        denom: amount.denom.to_string(),
        amount: amount.amount.to_string(),
    });

    let bin = MsgUndelegate {
        delegator_address,
        validator_address,
        amount,
    }
    .encode_to_vec();

    Ok(Response::default().add_message(CosmosMsg::Stargate {
        type_url: "/alliance.alliance.MsgUndelegate".to_string(),
        value: Binary::from(bin),
    }))
}

fn try_redelegate(
    delegator_address: String,
    validator_src_address: String,
    validator_dst_address: String,
    amount: Coin,
) -> Result<Response, ContractError> {
    let amount = Some(CosmosNativeCoin {
        denom: amount.denom.to_string(),
        amount: amount.amount.to_string(),
    });

    let bin = MsgRedelegate {
        delegator_address,
        validator_src_address,
        validator_dst_address,
        amount,
    }
    .encode_to_vec();

    Ok(Response::default().add_message(CosmosMsg::Stargate {
        type_url: "/alliance.alliance.MsgRedelegate".to_string(),
        value: Binary::from(bin),
    }))
}

fn try_claim_rewards(
    delegator_address: String,
    validator_address: String,
    denom: String,
) -> Result<Response, ContractError> {
    let bin = MsgClaimDelegationRewards {
        delegator_address,
        validator_address,
        denom,
    }
    .encode_to_vec();

    Ok(Response::default().add_message(CosmosMsg::Stargate {
        type_url: "/alliance.alliance.MsgClaimDelegationRewards".to_string(),
        value: Binary::from(bin),
    }))
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
