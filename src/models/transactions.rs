use chrono::NaiveDateTime;
use crate::schema::wallet_transactions;

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct WalletTransaction {
    pub transaction_id: i32,
    pub client_id: i32,
    pub date: NaiveDateTime,
    pub is_buy: bool,
    pub is_personal: bool,
    pub location_id: i64,
    pub quantity: i32,
    pub type_id: i32,
    pub unit_price: f32,
    pub description: String,
    pub total_amount: f32,
    pub tax: f32,    
}

impl WalletTransaction {
    upsert!(wallet_transactions);
}