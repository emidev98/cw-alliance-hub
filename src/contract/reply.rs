use std::ops::Add;

use crate::state::{Cfg, CFG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, Addr, DepsMut, Env, Response, StdResult,
};
use cosmwasm_std::{Reply, StdError};

use crate::contract::constants::{
    MINT_NFT_REPLY,
    INSTANTIATE_REPLY_ID,
};
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        MINT_NFT_REPLY => handle_mint_nft_reply(deps, msg),
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
            return event.ty == "instantiate";
        })
        .ok_or_else(|| StdError::generic_err(evt))?;

    let mut evts = String::new();
    /* Find the contract_address from _instantiate_contract event*/
    let contract_address = &event
        .attributes
        .iter()
        .find(|attr| {
            evts.push_str(&String::from(" ").add(&attr.key.clone()));
            return attr.key == "_contract_address";
        })
        .ok_or_else(|| StdError::generic_err(evts))?
        .value;

    /* Update the state of the contract adding the new generated nft_contract_addr */
    CFG.save(
        deps.storage,
        &Cfg {
            nft_contract_addr: Addr::unchecked(contract_address.clone()),
            minted_nfts: 0,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate_nft_reply")
        .add_attribute("nft_contract_address", contract_address))
}

fn handle_mint_nft_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Unwrap the result, if it is an error, respond with the error
    if msg.result.is_err() {
        return Err(StdError::generic_err("Error minting nft"));
    }

    // Update the state of the contract increasing the minted nfts by 1 
    CFG.update(_deps.storage, |mut cfg| -> StdResult<_> {
        cfg.minted_nfts += 1;
        Ok(cfg)
    })?;

    Ok(Response::new())
}
