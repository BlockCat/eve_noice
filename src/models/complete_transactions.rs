use crate::EveDatabase;
use crate::schema::complete_transactions;

use crate::models::{ WalletTransaction, InvType };

#[derive(Identifiable, Queryable, Insertable, Debug, Serialize)]
#[primary_key(buy_transaction_id, sell_transaction_id)]
pub struct CompleteTransaction {
    pub character_id: i32,
    pub buy_transaction_id: Option<i64>,
    pub sell_transaction_id: i64,
    pub bought_unit_price: Option<f32>,
    pub bought_unit_taxes: Option<f32>,
    pub sold_unit_price: f32,
    pub sold_unit_taxes: f32,
    pub amount: i32    
}

impl CompleteTransaction {

    pub fn new(character_id: i32, buy_transaction: Option<WalletTransaction>, sell_transaction: &WalletTransaction, amount: i32) -> Self {
        let (buy_transaction_id, bought_unit_price, bought_unit_taxes) = buy_transaction.map_or((None, None, None), |x| {
            (Some(x.transaction_id), Some(x.unit_price), Some(x.unit_taxes))
        });
        CompleteTransaction {
            character_id,
            buy_transaction_id,
            sell_transaction_id: sell_transaction.transaction_id,
            bought_unit_price,
            bought_unit_taxes,
            sold_unit_price: sell_transaction.unit_price,
            sold_unit_taxes: sell_transaction.unit_taxes,            
            amount,
        }
    }

    pub fn all(character_tid: i32, conn: &EveDatabase) -> diesel::QueryResult<Vec<(CompleteTransaction, WalletTransaction, InvType, Option<WalletTransaction>)>> {
        use diesel::prelude::*;
        use crate::schema::wallet_transactions::dsl::{ transaction_id, wallet_transactions, type_id};        
        use crate::schema::inv_types::dsl::{inv_types, type_id as inv_type_id};
        use crate::schema::complete_transactions::dsl::*;

        let sells:Vec<(CompleteTransaction, WalletTransaction, InvType)> = 
            complete_transactions
            .filter(character_id.eq(character_tid))            
            .inner_join(wallet_transactions.on(transaction_id.eq(sell_transaction_id)))            
            .inner_join(inv_types.on(type_id.eq(inv_type_id)))
            .load(&conn.0).expect("Could not get sell transactions for complete transactions");

        let buys: Vec<(CompleteTransaction, Option<WalletTransaction>)> = 
            complete_transactions
            .filter(character_id.eq(character_tid))            
            .left_join(wallet_transactions.on(buy_transaction_id.eq(transaction_id.nullable())))
            .load(&conn.0).expect("Could not get buy transactions for complete transactions");

        Ok(sells.into_iter().zip(buys.into_iter())
            .map(|(sell, buy)| {
                (sell.0, sell.1, sell.2, buy.1)
            }).collect())        
    }

    upsert!(complete_transactions);
}