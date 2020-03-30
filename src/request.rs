use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ShareType {
    Email,
    Facebook,
    Twitter,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum MostPopularPeriod {
    #[serde(rename = "1")]
    OneDay,
    #[serde(rename = "7")]
    SevenDays,
    #[serde(rename = "30")]
    ThirtyDays,
}
