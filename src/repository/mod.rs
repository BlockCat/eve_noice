use crate::models::{ CompleteTransactionView, TransactionQueueView};
use crate::EveDatabase;
use diesel::prelude::*;
use chrono::{ NaiveDateTime, NaiveTime};

pub fn view_transactions(character_tid: i32, days: i64, conn: &EveDatabase) -> QueryResult<Vec<CompleteTransactionView>> {
    use crate::complete_transactions_views::dsl::*;    
    
    let back_date = (chrono::Utc::today() - chrono::Duration::days(days)).naive_utc();
    let back_date = NaiveDateTime::new(back_date, NaiveTime::from_hms(0, 0, 0));

    complete_transactions_views.filter(
        character_id.eq(character_tid)
        .and(sell_date.ge(back_date)))
    .load(&conn.0)
} 

pub fn inventory(character_tid: i32, items: Option<String>, conn: &EveDatabase) -> QueryResult<Vec<TransactionQueueView>> {
    use crate::transaction_queue_views::dsl::*;

    transaction_queue_views.filter(character_id.eq(character_tid)).load(&conn.0)
}