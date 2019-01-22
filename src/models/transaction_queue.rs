
use crate::schema::transaction_queues;

#[derive(Queryable, Insertable, Debug, Serialize)]
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

    pub fn find_latest(character_tid: i32, type_tid: i32, conn: &crate::EveDatabase) -> diesel::QueryResult<Self> {
            use diesel::QueryDsl;
            use diesel::ExpressionMethods;
            use diesel::RunQueryDsl;
            use diesel::BoolExpressionMethods;

            use crate::schema::transaction_queues::dsl::*;
            
            transaction_queues.filter(character_id.eq(character_tid).and(type_id.eq(type_tid)).and(amount_left.lt(0)))
                .order(transaction_id.desc())
                .first(&conn.0)
        }
}