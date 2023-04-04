use crate::msg::ExecuteMsg;
use crate::{
    entry_points::execute::{execute, Cw721ExecuteMsg},
    tests::utils::chain_with_contract_delegation,
    ContractError,
};
use cosmwasm_std::{
    coins, testing::mock_info, to_binary, Attribute, Binary, CosmosMsg, SubMsg, WasmMsg,
};
use cw721_progressive_metadata::state::{Metadata as CW721Metadata, Trait as CW721Trait};
use terra_proto_rs::alliance::alliance::MsgUndelegate;
use terra_proto_rs::{cosmos::base::v1beta1::Coin as CosmosNativeCoin, traits::Message};

#[test]
fn test_start_unbonding() {
    // GIVEN
    let (mut deps, env, info) = chain_with_contract_delegation(String::from("terra..."));
    let msg = ExecuteMsg::MsgStartUnbonding {
        token_id: String::from("0"),
    };

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // THEN
    assert_eq!(res.messages.len(), 2);
    let update_metadata = SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: String::from("terra..."),
            msg: to_binary(&Cw721ExecuteMsg::UpdateExtension {
                token_id: String::from("0"),
                extension: Some(CW721Metadata {
                    name: Some(String::from("Alliance NFT #0")),
                    attributes: Some(vec![CW721Trait {
                        display_type: String::from("Unbonding"),
                        trait_type: String::from("validator1"),
                        timestamp: env.block.time.plus_seconds(100),
                        value: String::from("100@token"),
                    }]),
                    ..Default::default()
                }),
            })
            .unwrap(),
            funds: vec![],
        },
        4,
    );
    assert_eq!(res.messages[0], update_metadata);

    let redelegate_sub_msg = SubMsg::new(CosmosMsg::Stargate {
        type_url: String::from("/alliance.alliance.MsgUndelegate"),
        value: Binary::from(
            MsgUndelegate {
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
    assert_eq!(res.messages[1], redelegate_sub_msg);
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "start_unbonding"),
            Attribute::new("sender", "creator")
        ]
    );
}

#[test]
fn test_start_unbonding_with_no_access() {
    // GIVEN
    let (mut deps, env, _info) = chain_with_contract_delegation(String::from("terra..."));
    let info = mock_info("invalid_creator", &coins(100, "token"));
    let msg = ExecuteMsg::MsgStartUnbonding {
        token_id: String::from("0"),
    };

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    // THEN
    assert_eq!(
        res,
        ContractError::UnauthorizedNFTOwnere(
            String::from("creator"),
            String::from("invalid_creator")
        )
    );
}
