use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};

// Contain the list of nfts minted by the contract
// Where the key is the address of the nft and the
// boolean value represents if the user has executed
// the MsgUndelegate to avoid double execution
pub const MINTED_NFTS: Map<Addr, bool> = Map::new("nfts");
pub const CFG: Item<Cfg> = Item::new("config");

#[cw_serde]
pub struct Cfg {
    pub nft_contract_addr: Addr,
}