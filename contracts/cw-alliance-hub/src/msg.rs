use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
use crate::alliance_queries::AllianceInfo;
use crate::state::Cfg;

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
    MsgDelegate { },
    MsgStartUnbonding { token_id: String },
    MsgRedelegate { token_id: String },
    MsgClaimRewards { token_id: String },
    MsgRedeemBond { token_id: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Cfg)]
    GetConfig { },

    #[returns(AllianceInfo)]
    GetAllianceInfo { denom: String },

    #[returns(TokenMetadata)]
    GetMetadata { denom: String},
}

#[cw_serde]
pub enum MigrateMsg {
    Migrate { },
}

#[cw_serde]
pub struct TokenMetadata {
    metadata: Metadata
}

#[cw_serde]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
}
#[cw_serde]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u32,
    pub aliases: Option<Vec<String>>,
}