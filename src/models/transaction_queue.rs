use diesel::prelude::*;
use crate::models::WalletTransaction;
use crate::schema::transaction_queues;

#[derive(Identifiable, Queryable, Insertable, Debug, Serialize)]
#[primary_key(character_id, transaction_id)]
pub struct TransactionQueue {
    pub character_id: i32,
    pub type_id: i32,
    pub transaction_id: i64,
    pub amount_left: i32,
}

impl TransactionQueue {

    pub fn new(character_id: i32, type_id: i32, transaction_id: i64, amount_left: i32) -> Self {
        TransactionQueue {
            character_id,
            type_id,
            transaction_id,
            amount_left
        }
    }
    upsert!(transaction_queues);

    pub fn find_latest(character_tid: i32, type_tid: i32, bdate: chrono::NaiveDateTime, conn: &crate::EveDatabase) -> diesel::QueryResult<(Self, Option<WalletTransaction>)> {
        use crate::schema::transaction_queues::dsl::*;
        use crate::schema::wallet_transactions::dsl::{wallet_transactions, transaction_id as buy_transaction, date as buy_date};

        transaction_queues.filter(
                character_id.eq(character_tid)
                .and(type_id.eq(type_tid))
                .and(amount_left.gt(0))
                .and(buy_date.le(bdate))
            )
            .order(transaction_id.asc())
            .left_join(wallet_transactions.on(transaction_id.eq(buy_transaction)))
            .first(&conn.0)
    }

    pub fn delete(&self, conn: &crate::EveDatabase) -> diesel::QueryResult<usize> {
        use crate::schema::transaction_queues::dsl::*;
        diesel::delete(transaction_queues.find((self.character_id, self.transaction_id))).execute(&conn.0)
    }
}