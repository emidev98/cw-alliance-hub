use std::ops::Add;
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{DisplayType, CFG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg,
    Validator, WasmMsg,
};
use cosmwasm_std::{BankMsg, Empty, Timestamp, Uint128};
use terra_proto_rs::alliance::alliance::MsgRedelegate;

use super::{
    constants::{
        DEFAULT_DELIMITER,
        MINT_NFT_REPLY_ID,
        UNBONDING_NFT_REPLY_ID,
        REDELEGATE_REPLY_ID,
        REDEEM_BOND_REPLY_ID
    },
    query,
};
use cw721_progressive_metadata::state::Extension;
use cw721_progressive_metadata::{
    state::{Metadata as CW721Metadata, Trait as CW721Trait},
    ExecuteMsg as Cw721ExecuteDefaultMsg,
};
use terra_proto_rs::{
    alliance::alliance::{MsgClaimDelegationRewards, MsgDelegate, MsgUndelegate},
    cosmos::base::v1beta1::Coin as CosmosNativeCoin,
    traits::Message,
};

type Cw721ExecuteMsg = Cw721ExecuteDefaultMsg<Extension, Empty>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // TODO send claim_rewards to the user if he has anys
    match msg {
        ExecuteMsg::MsgDelegate {} => try_delegate(env, info, deps),
        ExecuteMsg::MsgStartUnbonding { token_id } => {
            try_start_unbonding(env, info, deps, token_id)
        }
        ExecuteMsg::MsgRedelegate { token_id } => try_redelegate(env, info, deps, token_id),
        ExecuteMsg::MsgClaimRewards { token_id } => try_claim_rewards(env, info, deps, token_id),
        ExecuteMsg::MsgRedeemBond { token_id } => try_redeem_bond(env, info, deps, token_id),
    }
}

fn try_delegate(env: Env, info: MessageInfo, deps: DepsMut) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let validators = query::all_validators(deps.querier);
    if validators.len() == 0 {
        return Err(ContractError::NoValidatorsFound {});
    }

    let msg_delegate = generate_delegate_msg(info.funds, env.clone(), validators);
    let msg_mint = generate_mint_msg(
        info.sender.clone().into(),
        env.block.time,
        cfg.minted_nfts.to_string(),
        msg_delegate.clone(),
    );

    let delegate_msgs: Vec<CosmosMsg> = msg_delegate
        .iter()
        .map(|msg| CosmosMsg::Stargate {
            type_url: "/alliance.alliance.MsgDelegate".to_string(),
            value: Binary::from(msg.encode_to_vec()),
        })
        .collect();

    Ok(Response::new()
        .add_attribute("action", "delegate")
        .add_attribute("sender", info.sender.to_string())
        .add_submessage(SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: cfg.nft_contract_addr.unwrap().to_string(),
                msg: to_binary(&msg_mint)?,
                funds: vec![],
            },
            MINT_NFT_REPLY_ID,
        ))
        .add_messages(delegate_msgs))
}

fn generate_delegate_msg(
    funds: Vec<Coin>,
    env: Env,
    validators: Vec<Validator>,
) -> Vec<MsgDelegate> {
    let mut vals_len = validators.len() as u64;
    funds
        .iter()
        .map(|coin| {
            let pseudorandom_index = get_pseudorandom(env.block.height, vals_len);
            let val = &validators[pseudorandom_index as usize];

            let msg_delegate = MsgDelegate {
                delegator_address: env.contract.address.to_string(),
                validator_address: val.address.to_string(),
                amount: Some(CosmosNativeCoin {
                    denom: coin.denom.to_string(),
                    amount: coin.amount.to_string(),
                }),
            };

            // Remove 1 of the index to generate a new
            // pseudorandom index in the next iteration
            if vals_len > 1 {
                vals_len = vals_len - 1
            }

            msg_delegate
        })
        .collect::<Vec<MsgDelegate>>()
}

fn generate_mint_msg(
    sender: String,
    block_time: Timestamp,
    token_id: String,
    msg_delegate: Vec<MsgDelegate>,
) -> Cw721ExecuteMsg {
    let attributes = msg_delegate
        .iter()
        .map(|msg| {
            let unwrapped_coin = msg.amount.as_ref().unwrap();
            let value = unwrapped_coin
                .amount
                .clone()
                .add(DEFAULT_DELIMITER)
                .add(&unwrapped_coin.denom);

            CW721Trait {
                display_type: DisplayType::Delegated.to_string(),
                trait_type: msg.validator_address.to_string(),
                timestamp: block_time,
                value: value,
            }
        })
        .collect::<Vec<CW721Trait>>();

    let msg = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: sender,
        token_uri: None,
        extension: Some(CW721Metadata {
            name: Some(String::from("Alliance NFT #").add(&token_id)),
            attributes: Some(attributes),
            ..Default::default()
        }),
    };

    msg
}

fn get_pseudorandom(block_height: u64, max: u64) -> u64 {
    let seed: u64 = block_height % (max + 1);

    seed % max
}

fn try_start_unbonding(
    env: Env,
    info: MessageInfo,
    deps: DepsMut,
    token_id: String,
) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let query_res = query::all_nft_info(
        deps.querier,
        token_id.clone(),
        cfg.nft_contract_addr.clone().unwrap().to_string(),
    );
    if query_res.access.owner != info.sender.to_string() {
        return Err(ContractError::UnauthorizedNFTOwnere(
            query_res.access.owner,
            info.sender.to_string(),
        ));
    }
    let attrs = query_res
        .info
        .extension
        .attributes
        .clone()
        .unwrap_or(vec![]);
    if attrs.len() == 0 {
        return Err(ContractError::NoDelegationsFound(token_id));
    }

    let msgs = attrs
        .iter()
        .map(|attr| {
            let coin = attr.value.split(DEFAULT_DELIMITER).collect::<Vec<&str>>();
            let msg = MsgUndelegate {
                delegator_address: env.contract.address.to_string(),
                validator_address: attr.trait_type.clone(),
                amount: Some(CosmosNativeCoin {
                    amount: coin[0].to_string(),
                    denom: coin[1].to_string(),
                }),
            }
            .encode_to_vec();

            CosmosMsg::Stargate {
                type_url: "/alliance.alliance.MsgUndelegate".to_string(),
                value: Binary::from(msg),
            }
        })
        .collect::<Vec<CosmosMsg>>();
    let msg_update_nft = generate_unbonding_nft_msg(
        query_res.info.extension,
        cfg.unbonding_seconds,
        env.block.time,
        token_id,
    );

    Ok(Response::new()
        .add_attribute("action", "start_unbonding")
        .add_attribute("sender", info.sender.to_string())
        .add_messages(msgs)
        .add_submessage(SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: cfg.nft_contract_addr.unwrap().to_string(),
                msg: to_binary(&msg_update_nft.unwrap())?,
                funds: vec![],
            },
            UNBONDING_NFT_REPLY_ID,
        )))
}

fn generate_unbonding_nft_msg(
    query_res: CW721Metadata,
    unbonding_seconds: u64,
    block_time: Timestamp,
    token_id: String,
) -> Result<Cw721ExecuteMsg, ContractError> {
    let unbonding_timestamp = block_time.plus_seconds(unbonding_seconds);
    let attrs = query_res
        .attributes
        .unwrap()
        .iter()
        .map(|attr| {
            if (attr.display_type == DisplayType::Redelegating.to_string()
                && attr.timestamp < block_time)
                || attr.display_type != DisplayType::Delegated.to_string()
            {
                return Err(ContractError::UnbondingImpossible(token_id.clone()));
            }

            Ok(CW721Trait {
                display_type: DisplayType::Unbonding.to_string(),
                timestamp: unbonding_timestamp,
                ..attr.clone()
            })
        })
        .collect::<Result<Vec<CW721Trait>, ContractError>>()
        .unwrap();

    let msg = Cw721ExecuteMsg::UpdateExtension {
        token_id,
        extension: Some(CW721Metadata {
            attributes: Some(attrs),
            ..query_res
        }),
    };

    Ok(msg)
}

fn try_redelegate(
    env: Env,
    info: MessageInfo,
    deps: DepsMut,
    token_id: String,
) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let validators = query::all_validators(deps.querier);
    if validators.len() == 0 {
        return Err(ContractError::NoValidatorsFound {});
    }
    let query_res = query::all_nft_info(
        deps.querier,
        token_id.clone(),
        cfg.nft_contract_addr.clone().unwrap().to_string(),
    );
    if query_res.access.owner != info.sender.to_string() {
        return Err(ContractError::UnauthorizedNFTOwnere(
            query_res.access.owner,
            info.sender.to_string(),
        ));
    }
    let attrs = query_res.info.extension.attributes.clone().unwrap_or(vec![]);
    if attrs.len() == 0 {
        return Err(ContractError::NoDelegationsFound(token_id.clone()));
    }
    let msgs = generate_redelegate_msg(validators, attrs, env.clone(), token_id.clone());
    let msg_update_nft = generate_redelegate_nft_msg(
        query_res.info.extension,
        cfg.unbonding_seconds,
        env.block.time,
        token_id,
    );

    Ok(Response::new()
        .add_attribute("action", "redelegate")
        .add_attribute("sender", info.sender.to_string())
        .add_messages(msgs)
        .add_submessage(SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: cfg.nft_contract_addr.unwrap().to_string(),
                msg: to_binary(&msg_update_nft.unwrap())?,
                funds: vec![],
            },
            REDELEGATE_REPLY_ID,
        )))
}

fn generate_redelegate_msg(validators: Vec<Validator>, attrs: Vec<CW721Trait>, env: Env, token_id: String) -> Vec<CosmosMsg> {
    let mut vals_len = validators.len() as u64;

    let msgs = attrs
        .iter()
        .map(|attr| {
            if (attr.display_type == DisplayType::Redelegating.to_string() && attr.timestamp < env.block.time)
                || attr.display_type != DisplayType::Delegated.to_string() {
                return Err(ContractError::UnbondingImpossible(token_id.clone()));
            }
            let coin = attr.value.split(DEFAULT_DELIMITER).collect::<Vec<&str>>();

            let pseudorandom_index = get_pseudorandom(env.block.height, vals_len);
            let val = &validators[pseudorandom_index as usize];

            let msg = MsgRedelegate {
                delegator_address: env.contract.address.to_string(),
                validator_src_address: attr.trait_type.clone(),
                validator_dst_address: val.address.to_string(),
                amount: Some(CosmosNativeCoin {
                    denom: coin[1].to_string(),
                    amount: coin[0].to_string(),
                }),
            }
            .encode_to_vec();

            // Remove 1 of the index to generate a new
            // pseudorandom index in the next iteration
            if vals_len > 1 {
                vals_len = vals_len - 1
            }

            Ok(CosmosMsg::Stargate {
                type_url: "/alliance.alliance.MsgRedelegate".to_string(),
                value: Binary::from(msg),
            })
        })
        .collect::<Result<Vec<CosmosMsg>,ContractError>>()
        .unwrap();
    msgs
}

fn generate_redelegate_nft_msg(
    query_res: CW721Metadata,
    unbonding_seconds: u64,
    block_time: Timestamp,
    token_id: String,
) -> Result<Cw721ExecuteMsg, ContractError> {
    let unbonding_timestamp = block_time.plus_seconds(unbonding_seconds);
    let attrs = query_res
        .attributes
        .unwrap()
        .iter()
        .map(|attr| {
            if (attr.display_type == DisplayType::Redelegating.to_string() && attr.timestamp < block_time)
                || attr.display_type != DisplayType::Delegated.to_string() {
                return Err(ContractError::UnbondingImpossible(token_id.clone()));
            }

            Ok(CW721Trait {
                display_type: DisplayType::Redelegating.to_string(),
                timestamp: unbonding_timestamp,
                ..attr.clone()
            })
        })
        .collect::<Result<Vec<CW721Trait>, ContractError>>()
        .unwrap();

    let msg = Cw721ExecuteMsg::UpdateExtension {
        token_id,
        extension: Some(CW721Metadata {
            attributes: Some(attrs),
            ..query_res
        }),
    };

    Ok(msg)
}

fn try_claim_rewards(
    env: Env,
    info: MessageInfo,
    deps: DepsMut,
    token_id: String,
) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let query_res = query::all_nft_info(
        deps.querier,
        token_id.clone(),
        cfg.nft_contract_addr.unwrap().to_string(),
    );
    if query_res.access.owner != info.sender.to_string() {
        return Err(ContractError::UnauthorizedNFTOwnere(
            query_res.access.owner,
            info.sender.to_string(),
        ));
    }
    let attrs = query_res.info.extension.attributes.unwrap_or(vec![]);
    if attrs.len() == 0 {
        return Err(ContractError::NoDelegationsFound(token_id));
    }

    let msgs = attrs
        .iter()
        .map(|attr| {
            let coin = attr.value.split(DEFAULT_DELIMITER).collect::<Vec<&str>>();
            let msg = MsgClaimDelegationRewards {
                delegator_address: env.contract.address.to_string(),
                validator_address: attr.trait_type.clone(),
                denom: coin[1].to_string(),
            }
            .encode_to_vec();

            CosmosMsg::Stargate {
                type_url: "/alliance.alliance.MsgClaimDelegationRewards".to_string(),
                value: Binary::from(msg),
            }
        })
        .collect::<Vec<CosmosMsg>>();

    Ok(Response::new()
        .add_attribute("action", "claim_rewards")
        .add_attribute("sender", info.sender.to_string())
        .add_messages(msgs))
}

fn try_redeem_bond(
    env: Env,
    info: MessageInfo,
    deps: DepsMut,
    token_id: String,
) -> Result<Response, ContractError> {
    let cfg = CFG.load(deps.storage)?;
    let query_res = query::all_nft_info(
        deps.querier,
        token_id.clone(),
        cfg.nft_contract_addr.clone().unwrap().to_string(),
    );
    if query_res.access.owner != info.sender.to_string() {
        return Err(ContractError::UnauthorizedNFTOwnere(
            query_res.access.owner,
            info.sender.to_string(),
        ));
    }
    let attrs = query_res
        .info
        .extension
        .attributes
        .clone()
        .unwrap_or(vec![]);
    if attrs.len() == 0 {
        return Err(ContractError::NoDelegationsFound(token_id));
    }

    let msgs = attrs
        .iter()
        .map(|attr| {
            let coin = attr.value.split(DEFAULT_DELIMITER).collect::<Vec<&str>>();
            // generate msg send to the user address
            BankMsg::Send {
                to_address: info.sender.to_string(),
                amount: vec![Coin {
                    amount: Uint128::from_str(coin[0]).unwrap(),
                    denom: coin[1].to_string(),
                }],
            }
        })
        .collect::<Vec<BankMsg>>();
    let msg_update_nft =
        generate_redeem_bond_nft_msg(query_res.info.extension, env.block.time, token_id);

    Ok(Response::new()
        .add_attribute("action", "redeem_bond")
        .add_attribute("sender", info.sender.to_string())
        .add_messages(msgs)
        .add_submessage(SubMsg::reply_always(
            WasmMsg::Execute {
                contract_addr: cfg.nft_contract_addr.unwrap().to_string(),
                msg: to_binary(&msg_update_nft.unwrap())?,
                funds: vec![],
            },
            REDEEM_BOND_REPLY_ID,
        )))
}

fn generate_redeem_bond_nft_msg(
    query_res: CW721Metadata,
    block_time: Timestamp,
    token_id: String,
) -> Result<Cw721ExecuteMsg, ContractError> {
    let attrs = query_res
        .attributes
        .unwrap()
        .iter()
        .map(|attr| {
            if attr.display_type != DisplayType::Unbonding.to_string()
                && attr.timestamp < block_time
            {
                return Err(ContractError::UnbondingImpossible(token_id.clone()));
            }

            Ok(CW721Trait {
                display_type: DisplayType::Unbonded.to_string(),
                timestamp: block_time,
                ..attr.clone()
            })
        })
        .collect::<Result<Vec<CW721Trait>, ContractError>>()
        .unwrap();

    let msg = Cw721ExecuteMsg::UpdateExtension {
        token_id,
        extension: Some(CW721Metadata {
            attributes: Some(attrs),
            ..query_res
        }),
    };

    Ok(msg)
}
