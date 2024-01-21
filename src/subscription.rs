use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Subscription struct.
/// Can be bought
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// date
    pub date: NaiveDate,
    /// type of subscription
    pub sub_type: SubscriptionType,
}

impl Default for Subscription {
    fn default() -> Self {
        Self {
            date: Utc::now().date_naive(),
            sub_type: SubscriptionType::default(),
        }
    }
}

impl Subscription {
    pub fn new(sub_type: SubscriptionType) -> Self {
        Self {
            date: Utc::now().date_naive(),
            sub_type,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum SubscriptionType {
    #[default]
    Exercise8,
    Exercise12,
    Exercise16,
    Exercise24,
    Exercise32,
}
impl SubscriptionType {
    pub fn value(&self) -> SubscriptionTypeValues {
        match self {
            SubscriptionType::Exercise8 => SubscriptionTypeValues::new(8, dec!(8000.0)),
            SubscriptionType::Exercise12 => SubscriptionTypeValues::new(12, dec!(11520.0)),
            SubscriptionType::Exercise16 => SubscriptionTypeValues::new(16, dec!(14400.0)),
            SubscriptionType::Exercise24 => SubscriptionTypeValues::new(24, dec!(19200.0)),
            SubscriptionType::Exercise32 => SubscriptionTypeValues::new(32, dec!(24000.0)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionTypeValues {
    pub visits: u8,
    pub money: Decimal,
}

impl SubscriptionTypeValues {
    fn new(visits: u8, money: Decimal) -> Self {
        Self { visits, money }
    }
}
