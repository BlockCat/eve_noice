use crate::auth::AuthedClient;
use chrono::NaiveDateTime;

#[macro_export]
macro_rules! upsert {
    ($x:ident) => {        
        pub fn upsert(&self, conn: &crate::EveDatabase) -> diesel::result::QueryResult<usize> {
            use diesel::RunQueryDsl;
            diesel::replace_into(crate::schema::$x::table)
                .values(self)
                .execute(&conn.0)
        }

        pub fn upsert_batch(conn: &crate::EveDatabase, batch: &[Self]) -> diesel::result::QueryResult<usize> {
            use diesel::RunQueryDsl;
            use diesel::Connection;

            conn.0.transaction(|| {
                let result = batch.iter().filter_map(|e| {
                    diesel::replace_into(crate::schema::$x::table)
                        .values(e)
                        .execute(&conn.0).ok()
                }).sum();
                Ok(result)
            })
        }
    };
}

#[macro_use] mod evecharacter;
#[macro_use] mod transactions;

pub use self::evecharacter::EveCharacter;
pub use self::transactions::WalletTransaction;