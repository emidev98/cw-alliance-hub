pub mod entry_points {
    pub mod constants;
    pub mod execute;
    pub mod instantiate;
    pub mod migrate;
    pub mod query;
    pub mod reply;
}
pub mod msg;
pub mod state;
mod error;
pub mod tests;
pub use crate::error::ContractError;
