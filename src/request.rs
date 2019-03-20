use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ShareType {
    Email,
    Facebook,
    Twitter,
}

#[derive(Serialize, Debug)]
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
