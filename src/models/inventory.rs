#[derive(Queryable, Debug, Serialize, Clone)]
pub struct TransactionQueueView {
    pub type_name: String,
    pub type_id: i32,
    pub character_id: i32,    
    pub buy_transaction_id: i64,    
    pub buy_date: chrono::NaiveDateTime,    
    pub quantity: i32,
    pub buy_unit_price: f32,    
}