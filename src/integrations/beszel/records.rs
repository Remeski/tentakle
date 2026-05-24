use serde::Deserialize;

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct List<T> {
    pub page: isize,
    pub perPage: isize,
    pub totalPages: isize,
    pub totalItems: isize,
    pub items: Vec<T>

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
pub struct System {
    pub collectionId: String,
    pub collectionName: String,
    pub id: String,
    pub name: String,
    pub status: String,
    pub host: String,
    pub port: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct SystemStats {
    pub collectionId: String,
    pub collectionName: String,
    pub id: String,
    pub system: String,
    pub created: String,
    pub updated: String,
    pub stats: serde_json::Value
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Container {
    pub collectionId: String,
    pub collectionName: String,
    pub name: String,
    pub id: String,
    pub system: String,
    pub updated: isize,
    pub cpu: f32,
    pub memory: f32,
    pub net: f32,
    pub image: String,
    pub ports: String,
    pub status: String,
    pub health: isize
    
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize, Debug)]
pub struct Stats {
    pub la: [f64; 3]
}
