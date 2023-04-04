use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized NFT owner, expected '{0}', received '{1}'")]
    UnauthorizedNFTOwnere(String, String),

    #[error("NFT '{0}' has no delegations")]
    NoDelegationsFound(String),
    
    #[error("Cannot redeem bond for NFT '{0}'")]
    RedeeemBondImpossibel(String),

    #[error("Cannot unbond the '{0}' NFT")]
    UnbondingImpossible(String),

    #[error("Cannot redelegate the '{0}' NFT")]
    RedelegatingImpossible(String),

    #[error("Something went wrong quering the validatos of the network")]
    NoValidatorsFound {},

    #[error("Nft contract adddress not setup in the hub contract config")]
    NoNftContractAddress {},

    #[error("Serialization errors")]
    Serialize {},

    #[error("Something went wrong minting the NFT")]
    NFTMintError {},

    #[error("Something went wrong trying to instantiate the NFT")]
    NFTContractInstantiateError {},

    #[error("Funds were not received")]
    NoFundsReceived {},
}
