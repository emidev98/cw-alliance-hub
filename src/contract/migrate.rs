use crate::msg::MigrateMsg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, DepsMut, Env,  Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}