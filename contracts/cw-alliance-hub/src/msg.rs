use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::Cfg;

#[cw_serde]
pub struct InstantiateMsg {
    pub cw721_code_id: u64,
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
    MsgDelegate { },
    MsgUndelegate { token_id: String },
    MsgRedelegate { token_id: String },
    MsgClaimRewards { token_id: String },
    MsgRedeemUndelegation { token_id: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Cfg)]
    GetConfig { },
}

#[cw_serde]
pub enum MigrateMsg {
    Migrate { },
}
