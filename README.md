# CWAllianceHub

This Smart contract uses [x/alliance](https://github.com/terra-money/alliance) and [cw-nfts](https://github.com/CosmWasm/cw-nfts) enabling the users to stake multiple tokens and generate rewards. When tokens are send to this smart contractt, the smart contract will redeem an NFT representation of the staked tokens that allows the owner of the NFT to claim the rewards, redelegate or redeem back the tokens.

## Development Information

This smart contract shouldn't be used in production as it is right now because there are few things that are not handled and the economics are not completed. It should be used as example to understand:

- How to use [x/alliance](https://github.com/terra-money/alliance) from a smart contract through `CosmosMsg::Stargate`,
- How to setup a cargo workspaces with multiple contracts, linting, tests, tests coverage,
- How to reply to messages in CosmWasm,
- How to query a smaart contract from another smart contract,
- How to query the staking module from a smart contract,
- How to generate scripts to interact with the smart contract and blockchain to have a better development experience (see `scripts` folder),
- ...

In conclusion this smart contract is more of a proof of concept than a production ready smart contract but it can help you to understand many things about CosmWasm.

### Contract executions

- `MsgDelegate` 
    - User send [tokens](https://github.com/cosmos/cosmos-sdk/blob/main/types/coin.go#L173) to the smart contract,
    - smart contract:
        - apply a find algorithm to chose a validator and execute [MsgDelegate from x/alliance module](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L17),
        - send a newly minted NFT to the user populating the metadata with the delegatoin information and nft status `Delegated` and current block height.

- `MsgStartUnbonding`
    - NFT owner execute this method with token_id (minted in MsgDelegate),
    - smart contract:
        - check if NFT status is `Redelegating` and it's redelegating time has completed otherwise throws an error,
        - check if NFT status is NOT `Delegated` to throw an error,
        - if none of the previous statements is true, the smart contract executes [MsgUndelegate from x/alliance](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L85) and set the NFT status to `Unbonding` with block height in the future when the undelegation will be finalized.


- `MsgRedelegate`
    - NFT owner execute this method with token_id (minted in MsgDelegate),
    - smart contract:
        - check if NFT status is `Redelegating` and it's redelegating time has completed otherwise throws an error,
        - check if NFT status is NOT `Delegated` to throw an error,
        - the smart contract apply a find algorithm to active validators set and execute [MsgRedelegate from x/alliance](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L46), update the nft metadata with new validators,status `Redelegating` andd block height in the future when the redelegation will be finalized.

- `MsgClaimRewards`:
    - NFT owner execute this method with token_id (minted in MsgDelegate),
    - smart contract:
        - check if NFT status is `Redelegating` and it's redelegating time has completed otherwise throws an error,
        - check if NFT status is NOT `Delegated` to throw an error,
        - will [ClaimDelegationRewards from x/alliance module on behaf of the user](https://github.com/terra-money/alliance/blob/main/x/alliance/keeper/msg_server.go#L114).


- `MsgRedeemBond`
    - NFT owner execute this method with token_id (minted in MsgDelegate),
    - smart contract:
        - check if NFT status is `Redelegating` and it's redelegating time has completed otherwise throws an error,
        - check if NFT status is NOT `Delegated` to throw an error,
        - smart contract will send tokens written in the NFT metadata to NFT owner and will set the NFT status to `Unbonded`.


> ⚠️ **Slashing is not handled by the smart contract**.

> ⚠️ **Rewards are stored in the smart contract**

### Contract queries

- `GetConfig` return smart contract configuration:
    - **minted_nfts**: counter of how many nfts have been minted used to assign the next nft id,
    - **unbonding_seconds**: number of seconds set in staking module,
    - **nft_contract_addr**: the address of the nft collection used to represent the alliance NFTS.