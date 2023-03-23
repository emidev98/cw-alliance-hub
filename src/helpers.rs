use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, arg_msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&arg_msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg: msg,
            funds: vec![],
        }
        .into())
    }
}
