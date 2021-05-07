use chrono::{DateTime, Utc};
use rust_decimal::prelude::*;
use serde::Deserialize;

pub type Id = u64;
pub type Coin = String;
pub type Symbol = String;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Response<T> {
    Result { success: bool, result: T },
    Error { success: bool, error: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

pub type Subaccounts = Vec<Subaccount>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create {
    pub nickname: String,
    pub deletable: bool,
    pub editable: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeName;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delete;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin: Coin,
    pub free: Decimal,
    pub total: Decimal,
    pub spot_borrow: Decimal,
    pub available_without_borrow: Decimal,
}

pub type Balances = Vec<Balance>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub id: Id,
    pub coin: Coin,
    pub size: Decimal,
    pub time: DateTime<Utc>,
    pub notes: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketType {
    Future,
    Spot,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    #[serde(rename = "type")]
    pub market_type: MarketType,
    pub name: Symbol,
    pub underlying: Option<Coin>,
    pub base_currency: Option<Coin>,
    pub quote_currency: Option<Coin>,
    pub enabled: bool,
    pub ask: Decimal,
    pub bid: Decimal,
    pub last: Decimal,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub restricted: bool,
    pub min_provide_size: Decimal,
    pub price: Decimal,
    pub high_leverage_fee_exempt: bool,
    pub change1h: Decimal,
    pub change24h: Decimal,
    pub change_bod: Decimal,
    pub quote_volume24h: Decimal,
    pub volume_usd24h: Decimal,
}

pub type Markets = Vec<Market>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub asks: Vec<(Decimal, Decimal)>,
    pub bids: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: Id,
    pub liquidation: bool,
    pub price: Decimal,
    pub side: Side,
    pub size: Decimal,
    pub time: DateTime<Utc>,
}

pub type Trades = Vec<Trade>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub volume: Decimal,
    pub start_time: DateTime<Utc>,
}

pub type Prices = Vec<Price>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FutureType {
    Future,
    Perpetual,
    Move,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub ask: Decimal,
    pub bid: Decimal,
    pub change1h: Decimal,
    pub change24h: Decimal,
    //pub change_bod: Decimal,
    //pub volume_usd24h: Decimal,
    //pub volume: Decimal,
    pub description: String,
    pub enabled: bool,
    pub expired: bool,
    pub expiry: DateTime<Utc>,
    pub index: Decimal,
    //pub imf_factor: Decimal,
    pub last: Decimal,
    pub lower_bound: Decimal,
    pub mark: Decimal,
    pub name: Symbol,
    pub perpetual: bool,
    //pub position_limit_weight: Decimal,
    pub post_only: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub underlying: Symbol,
    pub upper_bound: Decimal,
    #[serde(rename = "type")]
    pub market_type: FutureType,
}

pub type Futures = Vec<Future>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureStats {
    pub volume: Decimal,
    pub next_funding_rate: Decimal,
    pub next_funding_time: DateTime<Utc>,
    pub expiration_price: Decimal,
    pub predicted_expiration_price: Decimal,
    pub strike_price: Decimal,
    pub open_interest: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub future: Symbol,
    pub rate: Decimal,
    pub time: DateTime<Utc>,
}

pub type FundingRates = Vec<FundingRate>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    backstop_provider: bool,
    collateral: Decimal,
    free_collateral: Decimal,
    initial_margin_requirement: Decimal,
    leverage: Decimal,
    liquidating: bool,
    maintenance_margin_requirement: Decimal,
    maker_fee: Decimal,
    margin_fraction: Decimal,
    open_margin_fraction: Decimal,
    taker_fee: Decimal,
    total_account_value: Decimal,
    total_position_size: Decimal,
    username: String,
    positions: Positions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    cost: Decimal,
    entry_price: Decimal,
    estimated_liquidation_price: Decimal,
    future: Symbol,
    initial_margin_requirement: Decimal,
    long_order_size: Decimal,
    maintenance_margin_requirement: Decimal,
    net_size: Decimal,
    open_size: Decimal,
    realized_pnl: Decimal,
    short_order_size: Decimal,
    side: Side,
    size: Decimal,
    unrealized_pnl: Decimal,
    collateral_used: Decimal,
}

pub type Positions = Vec<Position>;