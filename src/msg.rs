use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

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
    ListNFTS { },
}

#[cw_serde]
pub struct ListNFTSResponse {
    pub nfts: Vec<NFTResponse>,
}

#[cw_serde]
pub struct NFTResponse {
    pub address: Addr,
    pub undelegated: bool,
}

impl NFTResponse {
    pub fn new(address: Addr, undelegated: bool) -> Self {
        Self {
            address,
            undelegated,
        }
    }
}


#[cw_serde]
pub enum MigrateMsg {
    Migrate {}
}
