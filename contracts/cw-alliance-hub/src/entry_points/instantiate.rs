use crate::msg::InstantiateMsg;
use crate::state::Cfg;
use crate::{error::ContractError, state::CFG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_binary, DepsMut, Env, MessageInfo, Response};
use cosmwasm_std::{SubMsg, WasmMsg};

use super::constants::{CONTRACT_NAME, CONTRACT_VERSION, INSTANTIATE_REPLY_ID};
use cw2::set_contract_version;
use cw721_progressive_metadata::InstantiateMsg as Cw721InstantiateMsg;

// version info for migration info
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cw721_instantiate_msg = Cw721InstantiateMsg {
        name: msg.cw721_collection.name.clone(),
        symbol: msg.cw721_collection.symbol,
        minter: env.contract.address.to_string(),
    };

    // Instantiate CW721 contract
    let cw721_instantiate_submsg = SubMsg::reply_always(
        WasmMsg::Instantiate {
            code_id: msg.cw721_code_id,
            msg: to_binary(&cw721_instantiate_msg)?,
            funds: vec![],
            label: msg.cw721_collection.name.clone(),
            admin: Some(env.contract.address.to_string()),
        },
        INSTANTIATE_REPLY_ID,
    );

    CFG.save(deps.storage, &Cfg::new(msg.cw721_unbonding_seconds))?;

    Ok(Response::new()
        .add_submessage(cw721_instantiate_submsg)
        .add_attribute("action", "instantiate_alliance_hub")
        .add_attribute("sender", info.sender))
}
