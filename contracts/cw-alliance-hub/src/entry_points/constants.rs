
pub const CONTRACT_NAME: &str = "crates.io:cw-alliance-hub";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub const INSTANTIATE_REPLY_ID: u64 = 1;
pub const MINT_NFT_REPLY_ID: u64 = 2;
pub const UPDATE_NFT_REPLY_ID: u64 = 3;


// This is the default contract delimiter when 
// having to parse structs to strings for the
// NFT attributes metadata
pub const DEFAULT_DELIMITER: &str = "@";