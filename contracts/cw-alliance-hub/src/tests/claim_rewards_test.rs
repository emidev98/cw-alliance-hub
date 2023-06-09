use crate::msg::ExecuteMsg;
use crate::{
    entry_points::execute::execute, tests::utils::chain_with_contract_delegation, ContractError,
};
use cosmwasm_std::{coins, testing::mock_info, Attribute, Binary, CosmosMsg, SubMsg};
use terra_proto_rs::{alliance::alliance::MsgClaimDelegationRewards, traits::Message};

#[test]
fn test_claim_rewards() {
    // GIVEN
    let (mut deps, env, info) = chain_with_contract_delegation(String::from("terra..."));
    let msg = ExecuteMsg::MsgClaimRewards {
        token_id: String::from("0"),
    };

    // WHEN
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // THEN
    assert_eq!(res.messages.len(), 1);
    let claim_rewards_sub_msg = SubMsg::new(CosmosMsg::Stargate {
        type_url: String::from("/alliance.alliance.MsgClaimDelegationRewards"),
        value: Binary::from(
            MsgClaimDelegationRewards {
                delegator_address: String::from("cosmos2contract"),
                validator_address: String::from("validator1"),
                denom: String::from("token"),
            }
            .encode_to_vec(),
        ),
    });
    assert_eq!(res.messages[0], claim_rewards_sub_msg);
    assert_eq!(
        res.attributes,
        vec![
            Attribute::new("action", "claim_rewards"),
            Attribute::new("sender", "creator")
        ]
    );
}

#[test]
fn test_claim_rewards_with_no_access() {
    // GIVEN
    let (mut deps, env, _info) = chain_with_contract_delegation(String::from("terra..."));
    let info = mock_info("invalid_creator", &coins(100, "token"));
    let msg = ExecuteMsg::MsgClaimRewards {
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
