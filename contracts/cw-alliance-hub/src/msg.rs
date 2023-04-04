use crate::state::Cfg;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub cw721_code_id: u64,
    pub cw721_unbonding_seconds: u64,
    pub cw721_collection: CW721Collection,
}

#[cw_serde]
pub struct CW721Collection {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    MsgDelegate {},
    MsgStartUnbonding { token_id: String },
    MsgRedelegate { token_id: String },
    MsgClaimRewards { token_id: String },
    MsgRedeemBond { token_id: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Cfg)]
    GetConfig {},
}

#[cw_serde]
pub enum MigrateMsg {
    Migrate {},
}
