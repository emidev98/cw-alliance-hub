use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit(DepositMsg),
    UndelegateMsg {
        del_addr: Addr,
        val_addr: String,
        amount: Coin,
    },
    RedelegateMsg {
        del_addr: Addr,
        src_val_addr: String,
        dst_val_addr: String,
        amount: Coin,
    },
    ClaimRewardsMsg {
        del_addr: Addr,
        val_addr: String,
        demom: String,
    },
}

#[cw_serde]
pub struct DepositMsg {
    pub del_addr: Addr,
    pub val_addr: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum MigrateMsg {}
