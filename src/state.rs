use cosmwasm_std::Addr;
use cw_storage_plus::Map;

// Contain the list of nfts minted by the contract
// Where the key is the address of the nft and the
// boolean value represents if the user has executed
// the MsgUndelegate to avoid double execution
pub const MINTED_NFTS: Map<Addr, bool> = Map::new("nfts");
