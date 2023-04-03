use std::ops::Add;

use crate::state::CFG;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Addr, DepsMut, Env, Response, StdResult};
use cosmwasm_std::{Reply, StdError};

use super::constants::{
    INSTANTIATE_REPLY_ID, MINT_NFT_REPLY_ID, REDEEM_BOND_REPLY_ID, REDELEGATE_REPLY_ID,
    UNBONDING_NFT_REPLY_ID,
};
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        MINT_NFT_REPLY_ID => handle_mint_nft_reply_id(deps, msg),
        UNBONDING_NFT_REPLY_ID => handle_unbonding_reply_id(msg),
        REDELEGATE_REPLY_ID => handle_redelegate_reply_id(msg),
        REDEEM_BOND_REPLY_ID => handle_redeem_bond(msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Unwrap the result, if it is an error, respond with the error
    if msg.result.is_err() {
        let msg = "Error instantiating nft: "
            .to_string()
            .add(&msg.result.unwrap_err());
        return Err(StdError::generic_err(msg));
    }

    // Unwrap the resoults of the instantiate submessage
    let result = msg
        .result
        .into_result()
        .map_err(|op| StdError::generic_err(op))?;

    /* Find the event type instantiate which contains the contract_address*/
    let event = match result.events.iter().find(|event| event.ty == "instantiate") {
        Some(event) => event,
        None => return Err(StdError::generic_err("No instantiate event found")),
    };

    /* Find the contract_address from instantiate event*/
    let contract_address = match event.attributes.iter().find(|attr| attr.key == "_contract_address") {
        Some(attr) => attr.value.clone(),
        None => return Err(StdError::generic_err("No '_contract_address' attribute found")),
    };

    /* Update the state of the contract adding the new generated nft_contract_addr */
    CFG.update(deps.storage, |mut cfg| -> StdResult<_> {
        cfg.nft_contract_addr = Some(Addr::unchecked(contract_address.clone()));
        Ok(cfg)
    })?;

    Ok(Response::new()
        .add_attribute("action", "instantiate_nft_reply")
        .add_attribute("nft_contract_address", contract_address))
}

fn handle_mint_nft_reply_id(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Unwrap the result, if it is an error, respond with the error
    if msg.result.is_err() {
        let msg = "Error minting nft: "
            .to_string()
            .add(&msg.result.unwrap_err());
        return Err(StdError::generic_err(msg));
    }

    // Update the state of the contract increasing the minted nfts by 1
    let mut cfg = CFG.load(deps.storage)?;
    cfg.minted_nfts+=1;
    CFG.save(deps.storage, &cfg)?;

    Ok(Response::new()
        .add_attribute("action", "mint_nft_reply")
        .add_attribute("minted_nfts", cfg.minted_nfts.to_string()))
}

fn handle_unbonding_reply_id(msg: Reply) -> StdResult<Response> {
    // Unwrap the result, if it is an error, respond with the error
    if msg.result.is_err() {
        let msg = "Error update nft: "
            .to_string()
            .add(&msg.result.unwrap_err());
        return Err(StdError::generic_err(msg));
    }

    Ok(Response::new().add_attribute("method", "start_unbonding_reply"))
}

fn handle_redelegate_reply_id(msg: Reply) -> StdResult<Response> {
    // Unwrap the result, if it is an error, respond with the error
    if msg.result.is_err() {
        let msg = "Error update nft:"
            .to_string()
            .add(&msg.result.unwrap_err());
        return Err(StdError::generic_err(msg));
    }

    Ok(Response::new().add_attribute("method", "redelegate_reply"))
}

fn handle_redeem_bond(msg: Reply) -> StdResult<Response> {
    // Unwrap the result, if it is an error, respond with the error
    if msg.result.is_err() {
        let msg = "Error update nft:"
            .to_string()
            .add(&msg.result.unwrap_err());
        return Err(StdError::generic_err(msg));
    }

    Ok(Response::new().add_attribute("method", "redeem_bond_reply"))
}
