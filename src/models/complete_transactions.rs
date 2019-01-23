use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::schema::complete_transactions;

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

    upsert!(complete_transactions);
}