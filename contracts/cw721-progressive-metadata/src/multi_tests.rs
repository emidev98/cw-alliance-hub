use cosmwasm_std::{Addr, Empty, QuerierWrapper};
use cw721::OwnerOfResponse;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use crate::InstantiateMsg;

fn cw721_progressive_metadata_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::entry::execute,
        crate::entry::instantiate,
        crate::entry::query,
    )
    .with_migrate(crate::entry::migrate);
    Box::new(contract)
}


fn query_owner(querier: QuerierWrapper, cw721: &Addr, token_id: String) -> Addr {
    let resp: OwnerOfResponse = querier
        .query_wasm_smart(
            cw721,
            &crate::QueryMsg::<Empty>::OwnerOf {
                token_id,
                include_expired: None,
            },
        )
        .unwrap();
    Addr::unchecked(resp.owner)
}


// Test update extension by the collection owner only
#[test]
fn test_update_extension() {
    let mut app = App::default();
    let colletion_owner = || Addr::unchecked("colletion_owner");
    let nft_owner = || Addr::unchecked("nft_owner");
    let code_id_017 = app.store_code(cw721_progressive_metadata_contract());
    let token_id = "1".to_string();
    let cw721 = app
        .instantiate_contract(
            code_id_017,
            colletion_owner(),
            &InstantiateMsg {
                name: "collection".to_string(),
                symbol: "symbol".to_string(),
                minter: colletion_owner().into_string(),
            },
            &[],
            "cw721-progressive-metadata",
            Some(colletion_owner().into_string()),
        )
        .unwrap();

    app.execute_contract(
        colletion_owner(),
        cw721.clone(),
        &crate::ExecuteMsg::<Empty, Empty>::Mint {
            token_id: token_id.clone(),
            owner:  colletion_owner().to_string(),
            token_uri: None,
            extension: Empty::default(),
        },
        &[],
    )
    .unwrap();

    let owner = query_owner(app.wrap(), &cw721, token_id.clone());
    assert_eq!(owner, colletion_owner().to_string());

    let res_ok = app.execute_contract(
        colletion_owner(),
        cw721.clone(),
        &crate::ExecuteMsg::<Empty, Empty>::UpdateExtension {
            token_id: token_id.clone(),
            extension: Empty::default(),
        },
        &[],
    );
    assert!(res_ok.is_ok());

    let res_err = app.execute_contract(
        nft_owner(),
        cw721.clone(),
        &crate::ExecuteMsg::<Empty, Empty>::UpdateExtension {
            token_id: token_id.clone(),
            extension: Empty::default(),
        },
        &[],
    );
    assert!(res_err.is_err());

}
