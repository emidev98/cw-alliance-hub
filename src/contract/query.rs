use crate::msg::QueryMsg;
use crate::state::CFG;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, Env, StdResult
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(match msg {
        QueryMsg::GetConfig {} => to_binary(&CFG.load(deps.storage)?)?,
    })
}