use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Coin, Decimal, Uint128};

#[cw_serde]
pub enum CosmosQueryMsg {
    Custom(CustomQueryMsg)
}

#[cw_serde]
pub enum CustomQueryMsg {
    Alliance(AllianceQueryMsg),
    Token(TokenFactoryQuery),
}

#[cw_serde]
pub enum AllianceQueryMsg {
    Alliance {
        denom: String,
    },
    Delegation {
        denom: String,
        delegator: String,
        validator: String,
    },
    DelegationRewards {
        denom: String,
        delegator: String,
        validator: String,
    },
}

#[cw_serde]
pub enum TokenFactoryQuery {
    Metadata {
        denom: String,
    }
}

#[cw_serde]
pub struct AllianceInfo {
    pub denom: String,
    pub reward_weight: Decimal,
    pub take_rate: Decimal,
    pub total_tokens: Uint128,
    pub total_validator_shares: Decimal,
    pub reward_start_time: u64,
    pub reward_change_rate: Decimal,
    pub last_reward_change_time: u64,
    pub reward_weight_range: RewardWeightRange,
    pub is_initialized: bool,
}

#[cw_serde]
pub struct RewardWeightRange {
    pub min: Decimal,
    pub max: Decimal,
}

#[cw_serde]
pub struct Delegation {
    pub denom: String,
    pub delegator: String,
    pub validator: String,
    pub amount: Coin,
}

#[cw_serde]
pub struct DelegationRewards {
    pub rewards: Vec<Coin>,
}