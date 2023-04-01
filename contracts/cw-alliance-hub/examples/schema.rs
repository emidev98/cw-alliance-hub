use cosmwasm_schema::write_api;

use cw_alliance_hub::msg::{
    ExecuteMsg, 
    InstantiateMsg, 
    QueryMsg
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
