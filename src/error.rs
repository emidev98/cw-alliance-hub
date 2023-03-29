use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Something went wrong minting the NFT")]
    NFTMintError {},

    #[error("Something went wrong trying to instantiate the NFT")]
    NFTContractInstantiateError {},

    #[error("Something went wrong trying to pick a validator from range")]
    InvalidValidator {},
}
