use cosmwasm_std::{
    coins,
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Empty, Env, MessageInfo, OwnedDeps, Validator, Decimal, Reply, SubMsgResponse, Event
};

use crate::{msg::{CW721Collection, InstantiateMsg}, entry_points::{instantiate::instantiate, reply::reply}};

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

pub fn chain_with_mocked_data() -> (
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
        id : 1,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            data: None,
            events: vec![
                Event::new("instantiate").add_attribute("_contract_address", "terra...") ],
        })
    };
    reply(deps.as_mut(), env.clone(), msg).unwrap();

    (deps, env, info)
}

fn mock_querier() -> MockQuerier {
    let mut querier = MockQuerier::new(&[]);
    querier.update_staking("token", &[Validator {
        address: String::from("validator"),
        commission: Decimal::percent(0),
        max_commission: Decimal::percent(20),
        max_change_rate: Decimal::percent(1),
    },Validator {
        address: String::from("validator1"),
        commission: Decimal::percent(10),
        max_commission: Decimal::percent(20),
        max_change_rate: Decimal::percent(1),
    },Validator {
        address: String::from("validator2"),
        commission: Decimal::percent(20),
        max_commission: Decimal::percent(20),
        max_change_rate: Decimal::percent(1),
    }], &[]);

    querier
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
