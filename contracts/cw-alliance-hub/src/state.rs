use std::{fmt, str::FromStr};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;


// Contain the list of nfts minted by the contract
// Where the key is the address of the nft and the
// boolean value represents if the user has executed
// the MsgUndelegate to avoid double execution
pub const CFG: Item<Cfg> = Item::new("config");

#[cw_serde]
pub struct Cfg {
    pub minted_nfts: u64,
    pub unbonding_seconds: u64,
    pub nft_contract_addr: Option<Addr>,
}

impl Cfg {
    pub fn new(unbonding_seconds: u64) -> Self {
        Cfg {
            minted_nfts: 0,
            nft_contract_addr: None,
            unbonding_seconds: unbonding_seconds,
        }
    }
}


#[cw_serde]
pub enum DisplayType {
    Unknown,
    Unbonded,
    Unbonding,
    Delegated,
    Redelegating,
}

impl FromStr for DisplayType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Delegated" => Ok(DisplayType::Delegated),
            "Unbonding" => Ok(DisplayType::Unbonding),
            "Redelegating" => Ok(DisplayType::Redelegating),
            "Unbonded" => Ok(DisplayType::Unbonded),
            "Unknown" => Ok(DisplayType::Unknown),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DisplayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisplayType::Delegated => write!(f,"Delegated"),
            DisplayType::Unbonding => write!(f,"Unbonding"),
            DisplayType::Redelegating => write!(f,"Redelegating"),
            DisplayType::Unbonded => write!(f,"Unbonded"),
            DisplayType::Unknown => write!(f,"Unknown"),
        }
    }
}

impl Into<DisplayType> for String {
    fn into(self) -> DisplayType {
        match self.as_str() {
            "Delegated" => DisplayType::Delegated,
            "Unbonding" => DisplayType::Unbonding,
            "Redelegating" => DisplayType::Redelegating,
            "Unbonded" => DisplayType::Unbonded,
            "Unknown" => DisplayType::Unknown,
            _ => DisplayType::Unknown,
        }
    }
}