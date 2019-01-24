use chrono::NaiveDateTime;
use crate::esi::EsiWalletTransaction;
use crate::schema::wallet_transactions;

#[derive(Identifiable, Queryable, Insertable, Debug, Serialize)]
#[primary_key(transaction_id)]
pub struct WalletTransaction {
    pub transaction_id: i64,
    pub character_id: i32,
    pub client_id: i32,    
    pub date: NaiveDateTime,
    pub is_buy: bool,
    pub is_personal: bool,
    pub location_id: i64,
    pub quantity: i32,
    pub type_id: i32,
    pub unit_price: f32,
    pub taxes: f32,
}

impl WalletTransaction {

    pub fn new(character_id: i32, transaction: EsiWalletTransaction, taxes: f32) -> Self {
        WalletTransaction {
            transaction_id: transaction.transaction_id,
            character_id: character_id,
            client_id: transaction.client_id,
            date: transaction.date.naive_utc(),
            is_buy: transaction.is_buy,
            is_personal: transaction.is_personal,
            location_id: transaction.location_id,
            quantity: transaction.quantity,
            type_id: transaction.type_id,
            unit_price: transaction.unit_price,
            taxes: taxes,
        }
    }

    pub fn all_buy(character_tid: i32, conn: &crate::EveDatabase) -> diesel::QueryResult<Vec<Self>> {
        use crate::schema::wallet_transactions::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        

        wallet_transactions.filter(character_id.eq(character_tid).and(is_buy.eq(true)))
            .limit(50)
            .load(&conn.0)
    }   
    
    upsert!(wallet_transactions);
    find!(wallet_transactions => transaction_id: i64 | character_id);

    find_extremes!(wallet_transactions on date for character_id);
}