use crate::state::CFG;
use crate::{msg::QueryMsg, ContractError};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, AllValidatorsResponse, Binary, Deps, Env, QueryRequest, StdResult,
};
use cosmwasm_std::{QuerierWrapper, StakingQuery, Validator, WasmQuery};
use cw721::AllNftInfoResponse;

use cw721_progressive_metadata::{state::Metadata as CW721Metadata, QueryMsg as CW721QueryEmpty};

type CW721Query = CW721QueryEmpty<CW721Metadata>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(match msg {
        QueryMsg::GetConfig {} => to_binary(&CFG.load(deps.storage)?)?,
    })
}

pub fn all_nft_info(
    querier: QuerierWrapper,
    token_id: String,
    contract_addr: String,
) -> Result<AllNftInfoResponse<CW721Metadata>, ContractError> {
    let msg = to_binary(&CW721Query::AllNftInfo {
        token_id,
        include_expired: None,
    })?;

    let res: AllNftInfoResponse<CW721Metadata> =
        querier.query(&QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }))?;

    Ok(res)
}

pub fn all_validators(querier: QuerierWrapper) -> Result<Vec<Validator>, ContractError> {
    let res = querier.query(&QueryRequest::Staking(StakingQuery::AllValidators {}));

    match res {
        Ok(AllValidatorsResponse { validators }) => Ok(validators),
        Err(err) => Err(ContractError::Std(err)),
    }
}
