use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    to_binary, Binary, ContractResult, Decimal, Empty, Env, Event, MessageInfo, OwnedDeps,
    QuerierResult, Reply, SubMsgResponse, SystemResult, Timestamp, Validator, WasmQuery,
};
use cw721::{AllNftInfoResponse, NftInfoResponse, OwnerOfResponse};

use crate::{
    entry_points::{execute::execute, instantiate::instantiate, reply::reply},
    msg::{CW721Collection, ExecuteMsg, InstantiateMsg},
};
use cw721_progressive_metadata::state::{Metadata as CW721Metadata, Trait as CW721Trait};

pub fn default_chain() -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    Env,
    MessageInfo,
) {
    let deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("creator", &coins(100, "token"));

    (deps, env, info)
}

pub fn chain_with_contract() -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    Env,
    MessageInfo,
) {
    let mut deps = mock_dependencies();
    deps.querier = mock_querier();
    let env = mock_env();
    let info = mock_info("creator", &coins(100, "token"));
    instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();
    let msg = Reply {
        id: 1,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![Event::new("instantiate").add_attribute("_contract_address", "terra...")],
        }),
    };
    reply(deps.as_mut(), env.clone(), msg).unwrap();

    (deps, env, info)
}

pub fn chain_with_contract_delegation() -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
    Env,
    MessageInfo,
) {
    // GIVEN the chain with data,
    let mut deps = mock_dependencies();
    deps.querier = mock_querier();
    let env = mock_env();
    let info = mock_info("creator", &coins(100, "token"));

    // WHEN the contract is will instantiated will also delegte
    instantiate(deps.as_mut(), env.clone(), info.clone(), inst_msg()).unwrap();
    let msg = Reply {
        id: 1,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![Event::new("instantiate").add_attribute("_contract_address", "terra...")],
        }),
    };
    reply(deps.as_mut(), env.clone(), msg).unwrap();

    execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        ExecuteMsg::MsgDelegate {},
    )
    .unwrap();
    let reply_msg = Reply {
        id: 2,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        }),
    };
    reply(deps.as_mut(), env.clone(), reply_msg).unwrap();

    // Then return the chain
    (deps, env, info)
}

fn mock_querier() -> MockQuerier {
    let mut querier = MockQuerier::new(&[]);

    querier.update_staking(
        "token",
        &[
            Validator {
                address: String::from("validator"),
                commission: Decimal::percent(0),
                max_commission: Decimal::percent(20),
                max_change_rate: Decimal::percent(1),
            },
            Validator {
                address: String::from("validator1"),
                commission: Decimal::percent(10),
                max_commission: Decimal::percent(20),
                max_change_rate: Decimal::percent(1),
            },
            Validator {
                address: String::from("validator2"),
                commission: Decimal::percent(20),
                max_commission: Decimal::percent(20),
                max_change_rate: Decimal::percent(1),
            },
            Validator {
                address: String::from("validator3"),
                commission: Decimal::percent(10),
                max_commission: Decimal::percent(20),
                max_change_rate: Decimal::percent(1),
            },
        ],
        &[],
    );

    querier.update_wasm(handle_wasm_query);

    querier
}

fn handle_wasm_query(wq: &WasmQuery) -> SystemResult<ContractResult<Binary>> {
    match wq {
        WasmQuery::Smart { contract_addr, .. } => {
            if *contract_addr == "terra..." {
                QuerierResult::Ok(ContractResult::Ok(
                    to_binary(&AllNftInfoResponse::<CW721Metadata> {
                        access: OwnerOfResponse {
                            owner: String::from("creator"),
                            approvals: vec![],
                        },
                        info: NftInfoResponse::<CW721Metadata> {
                            extension: CW721Metadata {
                                name: Some(String::from("Alliance NFT #0")),
                                attributes: Some(vec![CW721Trait {
                                    display_type: String::from("Delegated"),
                                    trait_type: String::from("validator1"),
                                    timestamp: Timestamp::from_seconds(100),
                                    value: String::from("100@token"),
                                }]),
                                ..Default::default()
                            },
                            token_uri: None,
                        },
                    })
                    .unwrap(),
                ))
            } else {
                unimplemented!()
            }
        }
        _ => unimplemented!(),
    }
}

pub fn inst_msg() -> InstantiateMsg {
    InstantiateMsg {
        cw721_code_id: 12345,
        cw721_unbonding_seconds: 100,
        cw721_collection: CW721Collection {
            name: String::from("Test Collection"),
            symbol: String::from("TST"),
        },
    }
}
