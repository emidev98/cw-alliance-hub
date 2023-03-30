use std::{fmt, str::FromStr};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::contract::constants::DEFAULT_DELIMITER;

// Contain the list of nfts minted by the contract
// Where the key is the address of the nft and the
// boolean value represents if the user has executed
// the MsgUndelegate to avoid double execution
pub const CFG: Item<Cfg> = Item::new("config");

#[cw_serde]
pub struct Cfg {
    pub minted_nfts: u64,
    pub nft_contract_addr: Addr,
}


#[cw_serde]
pub struct DisplayType {
    pub display_status: DisplayStatus,
    pub height: u64,
}

// Implement the `Display` trait for `DisplayType`
impl fmt::Display for DisplayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "{:?}{}{}",
            self.display_status, DEFAULT_DELIMITER, self.height
        )
    }
}

impl Into<DisplayType> for String {
    fn into(self) -> DisplayType {
        let parts: Vec<&str> = self.split(',').collect();
        let display_status = DisplayStatus::from_str(parts[0]).unwrap_or(DisplayStatus::Unknown);
        let height = parts[1].parse::<u64>().unwrap_or(0);

        DisplayType {
            display_status,
            height,
        }
    }
}

#[cw_serde]
pub enum DisplayStatus {
    Delegated,
    Redelegating,
    Undelegating,
    Undelegated,
    Unknown,
}

impl FromStr for DisplayStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Delegated" => Ok(DisplayStatus::Delegated),
            "Redelegating" => Ok(DisplayStatus::Redelegating),
            "Undelegating" => Ok(DisplayStatus::Undelegating),
            "Undelegated" => Ok(DisplayStatus::Undelegated),
            "Unknown" => Ok(DisplayStatus::Unknown),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DisplayStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DisplayStatus::Delegated => write!(f,"Delegated"),
            DisplayStatus::Redelegating => write!(f,"Redelegating"),
            DisplayStatus::Undelegating => write!(f,"Undelegating"),
            DisplayStatus::Undelegated => write!(f,"Undelegated"),
            DisplayStatus::Unknown => write!(f,"Unknown"),
        }
    }
}