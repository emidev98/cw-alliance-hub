use crate::msg::QueryMsg;
use crate::state::CFG;
use cosmwasm_std::{QuerierWrapper, WasmQuery, StakingQuery, Validator};
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

type CW721Query = CW721QueryEmpty<CW721Metadata>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(match msg {
        QueryMsg::GetConfig {} => to_binary(&CFG.load(deps.storage)?)?,
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
