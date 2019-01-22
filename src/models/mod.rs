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

#[macro_export]
macro_rules! find {
    ($x:ident => $id:ident: $t:ty) => {
        pub fn all(conn: &crate::EveDatabase) -> diesel::QueryResult<Vec<Self>> {
            use diesel::QueryDsl;
            use diesel::RunQueryDsl;
            
            use crate::schema::$x::dsl::*;

            $x.limit(30).load(&conn.0)            
        }

        pub fn find(tid: $t, conn: &crate::EveDatabase) -> diesel::QueryResult<Self> {
            use diesel::QueryDsl;
            use diesel::ExpressionMethods;
            use diesel::RunQueryDsl;

            use crate::schema::$x::dsl::*;
            
            $x.filter($id.eq(tid))
                .first(&conn.0)
        }
    };

    ($x:ident => $id:ident: $t:ty | $f:ident) => {
        pub fn all(character_tid: i32, conn: &crate::EveDatabase) -> diesel::QueryResult<Vec<Self>> {
            use diesel::QueryDsl;
            use diesel::ExpressionMethods;
            use diesel::RunQueryDsl;
            
            use crate::schema::$x::dsl::*;

            $x.filter($f.eq(character_tid)).limit(30).load(&conn.0)            
        }

        pub fn find(tid: $t, conn: &crate::EveDatabase) -> diesel::QueryResult<Self> {
            use diesel::QueryDsl;
            use diesel::ExpressionMethods;
            use diesel::RunQueryDsl;

            use crate::schema::$x::dsl::*;
            
            $x.filter($id.eq(tid))
                .first(&conn.0)
        }
    };
}

#[macro_export]
macro_rules! find_extremes {
    ($x:ident on $or:ident for $f:ident) => {
        pub fn find_latest(character_tid: i32, conn: &crate::EveDatabase) -> diesel::QueryResult<Self> {
            use diesel::QueryDsl;
            use diesel::ExpressionMethods;
            use diesel::RunQueryDsl;

            use crate::schema::$x::dsl::*;
            
            $x.filter($f.eq(character_tid))
                .order($or.desc())
                .first(&conn.0)
        }

        pub fn find_earliest(character_tid: i32,  conn: &crate::EveDatabase) -> diesel::QueryResult<Self> {
            use diesel::QueryDsl;
            use diesel::ExpressionMethods;
            use diesel::RunQueryDsl;

            use crate::schema::$x::dsl::*;
            
            $x.filter($f.eq(character_tid))
                .order($or.asc())
                .first(&conn.0)
        }
    };
}

#[macro_use] mod evecharacter;
#[macro_use] mod transactions;
#[macro_use] mod transaction_queue;

pub use self::evecharacter::EveCharacter;
pub use self::transaction_queue::TransactionQueue;
pub use self::transactions::WalletTransaction;