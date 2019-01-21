use chrono::NaiveDateTime;
use crate::schema::wallet_transactions;

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct WalletTransaction {
    pub transaction_id: i32,
    pub client_id: i32,
    pub journal_id: i64,
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
    find!(wallet_transactions => transaction_id: i32);
    find_extremes!(wallet_transactions on date);

    /*fn find(id: i32, conn: &crate::EveDatabase) -> diesel::QueryResult<Self> {
        use diesel::QueryDsl;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;

        use crate::schema::wallet_transactions::dsl::*;
        
        wallet_transactions.filter(transaction_id.eq(id))
            .first(&conn.0)
    }*/
}