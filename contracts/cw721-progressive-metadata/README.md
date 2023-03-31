# CW721 Progressive Metadata

This smart contract follows the [CW721 standard](https://github.com/CosmWasm/cw-nfts/tree/main/packages/cw721). The implementation is based on the forks [CW721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base) and [cw721-metadata-onchain](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-metadata-onchain) extension.

The custom code for this implementation is `ExecuteMsg::UpdateExtension` which allows the collection owner to modify the Metadata for any of the existent NFTs.

#### Authors 

The base smart contracts were developed by:

- Ethan Frey <ethanfrey@users.noreply.github.com>,
- Orkun Külçe <orkun@deuslabs.fi>,

The modifications to enable update the extension was developed by:

- emidev98

> ¡WARN: THIS CODE IS NOT AUDITED!

> Check the [LICENSE](./LICENSE).