use crate::msg::{QueryMsg, TokenMetadata};
use crate::state::CFG;
use cosmwasm_std::{QuerierWrapper, WasmQuery, StakingQuery, Validator, from_binary};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, Env, StdResult, QueryRequest,
    AllValidatorsResponse
};
use cw721::AllNftInfoResponse;

use cw721_progressive_metadata::{
    state::Metadata as CW721Metadata,
    QueryMsg as CW721QueryEmpty
};
use crate::alliance_queries::{AllianceQueryMsg, CosmosQueryMsg, TokenFactoryQuery, AllianceInfo, CustomQueryMsg};

type CW721Query = CW721QueryEmpty<CW721Metadata>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(match msg {
        QueryMsg::GetConfig {} => to_binary(&CFG.load(deps.storage)?)?,
        QueryMsg::GetAllianceInfo { denom } => to_binary(&get_alliance_info(deps.querier, denom))?,
        QueryMsg::GetMetadata {denom} => to_binary(&get_metadata(deps.querier, denom))?,
    })
}

pub fn all_nft_info(querier: QuerierWrapper, token_id: String, contract_addr: String) -> AllNftInfoResponse<CW721Metadata> {
    let msg = to_binary(&CW721Query::AllNftInfo {
        token_id,
        include_expired: None,
    }).unwrap();
    
    let res: AllNftInfoResponse<CW721Metadata> = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr,
        msg
    })).unwrap();
    res
}

pub fn all_validators(querier: QuerierWrapper) -> Vec<Validator> {
    let res = querier   
        .query(&QueryRequest::Staking(StakingQuery::AllValidators {}));

    match res {
        Ok(AllValidatorsResponse { validators }) => validators,
        Err(_) => vec![],
    }
}

/// This only works if the chain implements alliance bindings into the custom query
/// See: https://github.com/terra-money/core/tree/feat/alliance-wasm-queries
pub fn get_alliance_info(querier: QuerierWrapper, denom: String) -> AllianceInfo {
    let query = CosmosQueryMsg::Custom(CustomQueryMsg::Alliance(AllianceQueryMsg::Alliance {
        denom,
    }));
    let res = querier.raw_query(&to_binary(&query).unwrap()).unwrap();
    let alliance_info: AllianceInfo = from_binary(&res.unwrap()).unwrap();
    alliance_info
}

pub fn get_metadata(querier: QuerierWrapper, denom: String) -> TokenMetadata {
    let query = CosmosQueryMsg::Custom(CustomQueryMsg::Token(TokenFactoryQuery::Metadata {
        denom,
    }));
    let res = querier.raw_query(&to_binary(&query).unwrap()).unwrap();
    let metadata: TokenMetadata = from_binary(&res.unwrap()).unwrap();
    metadata
}