use crate::EveDatabase;
use crate::schema::complete_transactions;
use crate::models::WalletTransaction;

#[derive(Identifiable, Queryable, Insertable, Debug, Serialize)]
#[primary_key(buy_transaction_id, sell_transaction_id)]
pub struct CompleteTransaction {
    pub character_id: i32,
    pub buy_transaction_id: Option<i64>,
    pub sell_transaction_id: i64,
    pub amount: i32,
    pub taxes: f32
}

impl CompleteTransaction {

    pub fn new(character_id: i32, buy_transaction_id: Option<i64>, sell_transaction_id: i64, amount: i32, taxes: f32) -> Self {
        CompleteTransaction {
            character_id,
            buy_transaction_id,
            sell_transaction_id,
            amount,
            taxes
        }
    }

    pub fn all(character_tid: i32, conn: &EveDatabase) -> diesel::QueryResult<Vec<(CompleteTransaction, WalletTransaction, Option<WalletTransaction>)>> {
        use diesel::prelude::*;
        use crate::schema::wallet_transactions::dsl::{ transaction_id, wallet_transactions};        
        use crate::schema::complete_transactions::dsl::*;

        let sells:Vec<(CompleteTransaction, Option<WalletTransaction>)> = 
            complete_transactions
            .filter(character_id.eq(character_tid))
            .left_join(wallet_transactions.on(transaction_id.eq(sell_transaction_id)))            
            .load(&conn.0).expect("Could not get sell transactions for complete transactions");

        let buys: Vec<(CompleteTransaction, Option<WalletTransaction>)> = 
            complete_transactions
            .filter(character_id.eq(character_tid))            
            .left_join(wallet_transactions.on(buy_transaction_id.eq(transaction_id.nullable())))
            .load(&conn.0).expect("Could not get buy transactions for complete transactions");

        Ok(sells.into_iter().zip(buys.into_iter())
            .map(|(sell, buy)| {
                (sell.0, sell.1.unwrap(), buy.1)
            }).collect())        
    }

    upsert!(complete_transactions);
}