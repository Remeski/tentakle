use chrono::{DateTime, Utc};
use serde::Deserialize;

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub page: isize,
    pub perPage: isize,
    pub total_pages: isize,
    pub total_items: isize,
    pub items: Vec<T>,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct AuthRecord {
    pub id: String,
    pub collectionId: String,
    pub collectionName: String,
    pub created: String,
    pub updated: String,
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub emailVisibility: bool,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Auth {
    pub token: String,
    pub record: AuthRecord,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub collection_id: String,
    pub collection_name: String,
    pub id: String,
    pub name: String,
    pub status: String,
    pub host: String,
    pub port: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    pub collection_id: String,
    pub collection_name: String,
    pub id: String,
    pub system: String,
    pub created: String,
    pub updated: String,
    pub stats: serde_json::Value,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub collection_id: String,
    pub collection_name: String,
    pub name: String,
    pub id: String,
    pub system: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated: DateTime<Utc>,
    pub cpu: f32,
    pub memory: f32,
    pub net: f32,
    pub image: String,
    pub ports: String,
    pub status: String,
    pub health: isize,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Stats {
    pub la: [f64; 3],
}
