use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_progressive_metadata::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg<Empty, Empty>,
        query: QueryMsg<Empty>,
    }
}
