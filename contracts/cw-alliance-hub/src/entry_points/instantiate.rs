use crate::error::ContractError;
use crate::msg::InstantiateMsg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, to_binary, DepsMut, Env, MessageInfo,Response};
use cosmwasm_std::{SubMsg, WasmMsg};

use cw2::set_contract_version;
use cw721_progressive_metadata::{
    InstantiateMsg as Cw721InstantiateMsg,
};
use super::constants::{
    CONTRACT_NAME, 
    CONTRACT_VERSION,
    INSTANTIATE_REPLY_ID,
};


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
            funds: info.funds,
            label: msg.cw721_collection.name.clone(),
            admin: Some(env.contract.address.to_string()),
        },
        INSTANTIATE_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(cw721_instantiate_submsg)
        .add_attribute("action", "instantiate_alliance_hub")
        .add_attribute("sender", info.sender)
        .add_attribute("cw721_label", msg.cw721_collection.name))
}