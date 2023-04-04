use crate::msg::ExecuteMsg;
use crate::{
    entry_points::execute::{execute, Cw721ExecuteMsg},
    tests::utils::chain_with_contract_delegation,
    ContractError,
};
use cosmwasm_std::{coins, testing::mock_info, to_binary, Attribute, SubMsg, WasmMsg};
use cosmwasm_std::{BankMsg, Coin, Uint128};
use cw721_progressive_metadata::state::{Metadata as CW721Metadata, Trait as CW721Trait};

#[test]
fn test_redeem_bond() {
    // GIVEN
    let (mut deps, mut env, info) =
        chain_with_contract_delegation(String::from("terra...unbonding"));
    let msg = ExecuteMsg::MsgRedeemBond {
        token_id: String::from("0"),
    };
    env.block.time = env.block.time.plus_seconds(101);

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // THEN
    assert_eq!(res.messages.len(), 2);
    let update_metadata = SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: String::from("terra...unbonding"),
            msg: to_binary(&Cw721ExecuteMsg::UpdateExtension {
                token_id: String::from("0"),
                extension: Some(CW721Metadata {
                    name: Some(String::from("Alliance NFT #0")),
                    attributes: Some(vec![CW721Trait {
                        display_type: String::from("Unbonded"),
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
        5,
    );
    assert_eq!(res.messages[0], update_metadata);

    let send_msg = SubMsg::new(BankMsg::Send {
        to_address: String::from("creator"),
        amount: vec![Coin::new(Uint128::new(100).into(), String::from("token"))],
    });

    assert_eq!(res.messages[1], send_msg);
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "redeem_bond"),
            Attribute::new("sender", "creator")
        ]
    );
}

#[test]
fn test_redeem_bond_before_waiting_time() {
    // GIVEN
    let (mut deps, env, info) = chain_with_contract_delegation(String::from("terra..."));
    let msg = ExecuteMsg::MsgRedeemBond {
        token_id: String::from("0"),
    };

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    // THEN
    assert_eq!(res, ContractError::RedeeemBondImpossibel(String::from("0")));
}

#[test]
fn test_redeem_bond_with_no_access() {
    // GIVEN
    let (mut deps, env, _info) = chain_with_contract_delegation(String::from("terra..."));
    let info = mock_info("invalid_creator", &coins(100, "token"));
    let msg = ExecuteMsg::MsgRedeemBond {
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
