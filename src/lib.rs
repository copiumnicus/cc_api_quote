use revm::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteIntent {
    pub from: Address,
    pub to: Address,
    pub input: u128,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteIntentRes {
    pub to_amount: u128,
    pub gas_fees_in_usd: f64,
    /// not inclusive of gas fees
    pub from_token_price_in_usd: f64,
    /// not inclusive of gas fees
    pub to_token_price_in_usd: f64,
    pub block: u64,
}
