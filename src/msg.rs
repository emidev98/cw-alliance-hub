use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit(DepositMsg),
    UndelegateMsg {
        delegator_address: String,
        validator_address: String,
        amount: Coin,
    },
    RedelegateMsg {
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    },
    ClaimRewardsMsg {
        delegator_address: String,
        validator_address: String,
        demom: String,
    },
}

#[cw_serde]
pub struct DepositMsg {
    pub delegator_address: String,
    pub validator_address: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum MigrateMsg {}
