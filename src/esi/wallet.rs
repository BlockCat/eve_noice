use restson::RestPath;

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct EsiWallet(pub f32);

#[derive(Serialize, Deserialize, Debug)]
pub struct EsiWalletTransactions(pub Vec<EsiWalletTransaction>);

#[derive(Serialize, Deserialize, Debug)]
pub struct EsiWalletTransaction {
    pub client_id: i32,
    pub date: DateTime<Utc>,
    pub is_buy: bool,
    pub is_personal: bool,    
    pub journal_ref_id: i64,
    pub location_id: i64,
    pub quantity: i32,
    pub transaction_id: i64,
    pub type_id: i32,
    pub unit_price: f32
}

restpath!(EsiWallet, "/characters/{character_id}/wallet/", [character_id: i32]);
restpath!(EsiWalletTransactions, "/characters/{character_id}/wallet/transactions/", [character_id: i32]);