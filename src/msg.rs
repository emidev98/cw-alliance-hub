use cosmwasm_schema::cw_serde;

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
    MsgUndelegate { nft_address: String },
    MsgRedelegate { nft_address: String },
    MsgClaimRewards { nft_address: String },
    MsgRedeemUndelegation { nft_address: String },
}

#[cw_serde]
pub enum QueryMsg {
    GetConfig { },
}

#[cw_serde]
pub enum MigrateMsg {
    Migrate {}
}
