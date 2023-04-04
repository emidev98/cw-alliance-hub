use crate::{entry_points::{
    execute::{execute, Cw721ExecuteMsg},
    reply::reply,
}, ContractError};
use crate::msg::ExecuteMsg;
use crate::tests::utils::chain_with_contract;
use cosmwasm_std::{testing::mock_info, Reply, SubMsgResponse, StdError, Response, Coin, to_binary, Attribute, Binary, CosmosMsg, SubMsg, WasmMsg};
use cw721_progressive_metadata::{
    state::{Metadata as CW721Metadata, Trait as CW721Trait},
};
use terra_proto_rs::{
    alliance::alliance::MsgDelegate,
    cosmos::base::v1beta1::Coin as CosmosNativeCoin,
    traits::Message,
};

#[test]
fn test_delegate() {
    // GIVEN
    let (mut deps, env, info) = chain_with_contract();
    let msg = ExecuteMsg::MsgDelegate {};

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // THEN
    assert_eq!(2, res.messages.len());

    let instantiate_sub_msg = SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: String::from("terra..."),
            msg: to_binary(&Cw721ExecuteMsg::Mint {
                token_id: String::from("0"),
                owner: String::from("creator"),
                token_uri: None,
                extension: Some(CW721Metadata {
                    name: Some(String::from("Alliance NFT #0")),
                    attributes: Some(vec![CW721Trait {
                        display_type: String::from("Delegated"),
                        trait_type: String::from("validator1"),
                        timestamp: env.block.time,
                        value: String::from("100@token"),
                    }]),
                    ..Default::default()
                }),
            })
            .unwrap(),
            funds: vec![],
        },
        2,
    );
    assert_eq!(res.messages[0], instantiate_sub_msg);

    let delegate_sub_msg = SubMsg::new(CosmosMsg::Stargate {
        type_url: String::from("/alliance.alliance.MsgDelegate"),
        value: Binary::from(
            MsgDelegate {
                delegator_address: String::from("cosmos2contract"),
                validator_address: String::from("validator1"),
                amount: Some(CosmosNativeCoin {
                    denom: String::from("token"),
                    amount: String::from("100"),
                }),
            }
            .encode_to_vec(),
        ),
    });
    assert_eq!(res.messages[1], delegate_sub_msg);

    assert_eq!(
        vec![
            Attribute::new("action", "delegate"),
            Attribute::new("sender", "creator")
        ],
        res.attributes
    );
    
    // REPLY
    let reply_msg = Reply {
        id : 2,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        })
    };
    let reply_res = reply(deps.as_mut(), env, reply_msg).unwrap();
    assert_eq!(reply_res, 
        Response::new()
            .add_attribute("action", "mint_nft_reply")
            .add_attribute("minted_nfts", "1")
    );
}

#[test]
fn test_delegate_multiple_tokens() {
    // GIVEN
    let (mut deps, env, _) = chain_with_contract();
    let info = mock_info("creator", &vec![Coin::new(100,"token"),Coin::new(100,"stoken")]);
    let msg = ExecuteMsg::MsgDelegate {};

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // THEN
    assert_eq!(3, res.messages.len());

    let instantiate_sub_msg = SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: String::from("terra..."),
            msg: to_binary(&Cw721ExecuteMsg::Mint {
                token_id: String::from("0"),
                owner: String::from("creator"),
                token_uri: None,
                extension: Some(CW721Metadata {
                    name: Some(String::from("Alliance NFT #0")),
                    attributes: Some(vec![CW721Trait {
                        display_type: String::from("Delegated"),
                        trait_type: String::from("validator1"),
                        timestamp: env.block.time,
                        value: String::from("100@token"),
                    },
                    CW721Trait {
                        display_type: String::from("Delegated"),
                        trait_type: String::from("validator"),
                        timestamp: env.block.time,
                        value: String::from("100@stoken"),
                    }]),
                    ..Default::default()
                }),
            })
            .unwrap(),
            funds: vec![],
        },
        2,
    );
    assert_eq!(res.messages[0], instantiate_sub_msg);

    let delegate_sub_msg = SubMsg::new(CosmosMsg::Stargate {
        type_url: String::from("/alliance.alliance.MsgDelegate"),
        value: Binary::from(
            MsgDelegate {
                delegator_address: String::from("cosmos2contract"),
                validator_address: String::from("validator1"),
                amount: Some(CosmosNativeCoin {
                    denom: String::from("token"),
                    amount: String::from("100"),
                }),
            }
            .encode_to_vec(),
        ),
    });
    assert_eq!(res.messages[1], delegate_sub_msg);

    let delegate_sub_msg_2 = SubMsg::new(CosmosMsg::Stargate {
        type_url: String::from("/alliance.alliance.MsgDelegate"),
        value: Binary::from(
            MsgDelegate {
                delegator_address: String::from("cosmos2contract"),
                validator_address: String::from("validator"),
                amount: Some(CosmosNativeCoin {
                    denom: String::from("stoken"),
                    amount: String::from("100"),
                }),
            }
            .encode_to_vec(),
        ),
    });
    assert_eq!(res.messages[2], delegate_sub_msg_2);

    assert_eq!(
        vec![
            Attribute::new("action", "delegate"),
            Attribute::new("sender", "creator")
        ],
        res.attributes
    );
    
    // REPLY
    let reply_msg = Reply {
        id : 2,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            data: None,
        })
    };
    let reply_res = reply(deps.as_mut(), env, reply_msg).unwrap();
    assert_eq!(reply_res, 
        Response::new()
            .add_attribute("action", "mint_nft_reply")
            .add_attribute("minted_nfts", "1")
    );
}

#[test]
fn test_delegate_reply_error() {
    // GIVEN
    let (mut deps, env, info) = chain_with_contract();
    let msg = ExecuteMsg::MsgDelegate {};

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // THEN
    assert_eq!(2, res.messages.len());

    let instantiate_sub_msg = SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: String::from("terra..."),
            msg: to_binary(&Cw721ExecuteMsg::Mint {
                token_id: String::from("0"),
                owner: String::from("creator"),
                token_uri: None,
                extension: Some(CW721Metadata {
                    name: Some(String::from("Alliance NFT #0")),
                    attributes: Some(vec![CW721Trait {
                        display_type: String::from("Delegated"),
                        trait_type: String::from("validator1"),
                        timestamp: env.block.time,
                        value: String::from("100@token"),
                    }]),
                    ..Default::default()
                }),
            })
            .unwrap(),
            funds: vec![],
        },
        2,
    );
    assert_eq!(res.messages[0], instantiate_sub_msg);

    let delegate_sub_msg = SubMsg::new(CosmosMsg::Stargate {
        type_url: String::from("/alliance.alliance.MsgDelegate"),
        value: Binary::from(
            MsgDelegate {
                delegator_address: String::from("cosmos2contract"),
                validator_address: String::from("validator1"),
                amount: Some(CosmosNativeCoin {
                    denom: String::from("token"),
                    amount: String::from("100"),
                }),
            }
            .encode_to_vec(),
        ),
    });
    assert_eq!(res.messages[1], delegate_sub_msg);

    assert_eq!(
        vec![
            Attribute::new("action", "delegate"),
            Attribute::new("sender", "creator")
        ],
        res.attributes
    );
    
    // REPLY
    let reply_msg = Reply {
        id : 2,
        result: cosmwasm_std::SubMsgResult::Err(String::from("Something went wrong")),
    };
    let reply_res = reply(deps.as_mut(), env, reply_msg).unwrap_err();
    assert_eq!(reply_res, StdError::generic_err(String::from("Error minting nft: Something went wrong")));
}

#[test]
fn test_delegate_no_funds() {
    // GIVEN
    let (mut deps, env, _info) = chain_with_contract();
    let info = mock_info("creator", &vec![]);
    let info2 = mock_info("creator", &vec![Coin::new(0,"token")]);

    let msg = ExecuteMsg::MsgDelegate {};

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap_err();
    let res2 = execute(deps.as_mut(), env, info2, msg).unwrap_err();

    // THEN
    assert_eq!(res, ContractError::NoFundsReceived {  });
    assert_eq!(res2, ContractError::NoFundsReceived {  });
}
