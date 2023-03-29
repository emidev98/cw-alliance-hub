# CWAllianceHub

This Smart contract uses [x/alliance](https://github.com/terra-money/alliance) and [cw-nfts](https://github.com/CosmWasm/cw-nfts) standard enabling the users to stake multiple tokens and generate rewards. When tokens are staked using this smart contract the wallet will receive an NFT representation of the staked tokens that allows the owner of the NFT to claim the rewards, redelegate or redeem back the tokens.

### Execute

- `MsgDelegate` 
    - User send [tokens](https://github.com/cosmos/cosmos-sdk/blob/main/types/coin.go#L173) to the smart contract,
    - the smart contract:
        - apply a find algorithm to the active validators and execute [MsgDelegate from x/alliance module](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L17),
        - send a newly minted NFT to the user populating the metadata with the delegation information and nft status `DELEGATED` and current block height.

- `MsgUndelegate`
    - User send the NFT address (minted in MsgDelegate) to the smart contract,
    - the smart contract:
        - check if the user is not the owner of the nft throw an error. 
        - check if NFT status is `UNDELEGATED` or `UNDELEGATING` throw an error.
        - check if NFT status is `REDELEGATING` and block height is greater than current block height throw an error.
        - if none of the previous statements is true, the smart contract executes the [MsgUndelegate from x/alliance](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L85) and set the NFT status to `UNDELEGATING` with block height in the future when the undelegation will be finalized.


- `MsgRedelegate`
    - User send the NFT address (minted in MsgDelegate) to the smart contract,
    - the smart contract: 
        - check if the user is not the owner of the nft throw an error. 
        - check if NFT status is `UNDELEGATED` or `UNDELEGATING` throw an error.
        - check if NFT status is `REDELEGATING` and block height is greater than current block height throw an error,
        - the smart contract apply a find algorithm to the active validators and execute the [MsgRedelegate from x/alliance](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L46), update the nft metadata with the new validators and set the NFT status to `REDELEGATING` with block height in the future when the redelegation will be finalized.

- `MsgClaimRewards`:
    - User send the NFT address (minted in MsgDelegate) to the smart contract,
    - the smart contract:
        - check if the user is not the owner of the nft throw an error. 
        - check if NFT status is `UNDELEGATED` or `UNDELEGATING` and throw an error.
        - check if NFT status is `REDELEGATING` and block height is greater than current block height throw an error.
        - will [ClaimDelegationRewards from x/alliance module on behaf of the user](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L114) transferring the delegation rewards to the user account.


- `MsgRedeemUndelegation`
    - User send the NFT address (minted in MsgDelegate) to the smart contract,
    - the smart contract:
        - check if the user is not the owner of thenft thow an error,
        - check if NFT status is different than `UNDELEGATING` and block height is greather than current block height throw an error,
        - the smart contract will send the tokens written in the NFT metadata to the NFT owner and will set the NFT status to `UNDELEGATED`.


> :warning: **Slashing is not handled by the smart contract**. The smart contract will take a fee each time the user claims rewards to assure there is always a positive balance in the smart contract in case any of the validators is slashed.

### Query 

- `ListNFTS` return the list of minted NFTS from `MsgDelegate`


