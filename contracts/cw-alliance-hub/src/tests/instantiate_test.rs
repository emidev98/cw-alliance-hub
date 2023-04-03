use crate::entry_points::instantiate::instantiate;
use crate::entry_points::reply::reply;
use crate::state::CFG;
use crate::tests::utils::{inst_msg, default_chain};
use cosmwasm_std::{from_binary, CosmosMsg, WasmMsg, Reply, Event, SubMsgResponse, Addr, StdError};
use cw2::{get_contract_version, ContractVersion};
use cw721_progressive_metadata::{
    InstantiateMsg as Cw721InstantiateMsg,
};

#[test]
fn test_instantiate() {
    // GIVEN 
    let (mut deps, env, info) = default_chain();

    // WHEN 
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();

    // THEN
    assert_eq!(1, res.messages.len());

    match &res.messages[0].msg {
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id,
            msg,
            funds,
            label,
            admin,
        }) => {
            assert_eq!(*code_id, 12345);
            assert_eq!(
                from_binary::<Cw721InstantiateMsg>(msg).unwrap(),
                Cw721InstantiateMsg {
                    name: String::from("Test Collection"),
                    symbol: String::from("TST"),
                    minter: env.contract.address.to_string(),
                }
            );
            assert!(funds.is_empty());
            assert_eq!(label, "Test Collection");
            assert_eq!(admin, &Some(env.contract.address.to_string()));
        }
        _ => panic!("Unexpected message type"),
    }

    assert_eq!(2, res.attributes.len());
    assert_eq!(res.attributes[0], ("action", "instantiate_alliance_hub"));
    assert_eq!(res.attributes[1], ("sender", "creator"));

    let cfg = CFG.load(deps.as_ref().storage).unwrap();
    assert_eq!(cfg.unbonding_seconds, 100);
    assert_eq!(cfg.minted_nfts, 0);
    assert_eq!(cfg.nft_contract_addr, None);

    assert_eq!(get_contract_version(deps.as_ref().storage).unwrap(), 
        ContractVersion {
            contract: String::from("crates.io:cw-alliance-hub"),
            version: String::from("0.1.0")
        }
);
}

#[test]
fn test_instantiate_reply() {
    // GIVEN 
    let (mut deps, env, info) = default_chain();
    instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();
    let reply_msg = Reply {
        id : 1,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![
                Event::new("instantiate").add_attribute("_contract_address", "terra...") ],
        })
    };

    // WHEN 
    let res = reply(deps.as_mut(), env.clone(), reply_msg).unwrap();

    // THEN
    assert_eq!(0, res.messages.len());
    assert_eq!(2, res.attributes.len());
    assert_eq!(res.attributes[0], ("action", "instantiate_nft_reply"));
    assert_eq!(res.attributes[1], ("nft_contract_address", "terra..."));

    let cfg = CFG.load(deps.as_ref().storage).unwrap();
    assert_eq!(cfg.unbonding_seconds, 100);
    assert_eq!(cfg.minted_nfts, 0);
    assert_eq!(cfg.nft_contract_addr, Some(Addr::unchecked("terra...")));
}

#[test]
fn test_instantiate_reply_fail() {
    // GIVEN 
    let (mut deps, env, info) = default_chain();
    instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();
    let msg = Reply {
        id : 1,
        result: cosmwasm_std::SubMsgResult::Err(String::from("Something went wrong"))
    };

    // WHEN 
    let res = reply(deps.as_mut(), env.clone(), msg).unwrap_err();

    // THEN
    assert_eq!(res, StdError::generic_err(String::from("Error instantiating nft: Something went wrong")));
}

#[test]
fn test_instantiate_reply_without_instantiate_event() {
    // GIVEN 
    let (mut deps, env, info) = default_chain();
    instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();
    let msg = Reply {
        id : 1,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![],
        })
    };

    // WHEN 
    let res = reply(deps.as_mut(), env.clone(), msg).unwrap_err();

    // THEN
    assert_eq!(res, StdError::generic_err(String::from("No instantiate event found")));
}

#[test]
fn test_instantiate_reply_without_contract_addr_attribute() {
    // GIVEN 
    let (mut deps, env, info) = default_chain();
    instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();
    let msg = Reply {
        id : 1,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![
                Event::new("instantiate")
            ],
        })
    };

    // WHEN 
    let res = reply(deps.as_mut(), env.clone(), msg).unwrap_err();

    // THEN
    assert_eq!(res, StdError::generic_err(String::from("No '_contract_address' attribute found")));
}