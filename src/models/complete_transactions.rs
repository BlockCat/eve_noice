use crate::schema::complete_transactions;
use chrono::NaiveDateTime;

use crate::models::WalletTransaction;

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

#[derive(Queryable, Debug, Serialize)]
pub struct CompleteTransactionView {
    pub type_name: String,
    pub type_id: i32,
    pub character_id: i32,
    pub sell_transaction_id: i64,
    pub buy_transaction_id: Option<i64>,
    pub sell_date: NaiveDateTime,
    pub buy_date: Option<NaiveDateTime>,
    pub is_buy: bool,
    pub quantity: i32,
    pub buy_unit_price: Option<f32>,
    pub buy_unit_tax: Option<f32>,
    pub sell_unit_price: f32,
    pub sell_unit_tax: f32
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

    upsert!(complete_transactions);
}