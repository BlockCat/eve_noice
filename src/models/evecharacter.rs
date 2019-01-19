use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::BoolExpressionMethods;

use crate::schema::eve_characters;
use crate::EveDatabase;
use chrono::NaiveDateTime;
use rocket::request;

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct EveCharacter {
    pub id: i32,
    pub name: String,
    access_token: String,
    refresh_token: String,
    expiry_date: NaiveDateTime,
}

impl EveCharacter {
    pub fn new(id: i32, name: String, access_token: String, refresh_token: String, expiry_date: u32) -> EveCharacter {
        
        let expiry_date = chrono::Utc::now() + chrono::Duration::seconds(expiry_date as i64 - 60);
        EveCharacter {
            id, name, access_token, refresh_token, 
            expiry_date: expiry_date.naive_utc()
        }
    }

    pub fn insert(&self, conn: &EveDatabase) -> diesel::result::QueryResult<usize> {
        use crate::schema::eve_characters;
        diesel::replace_into(eve_characters::table)        
            .values(self)
            .execute(&conn.0)
    }
}

impl<'a, 'r> request::FromRequest<'a, 'r> for EveCharacter {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {
        use crate::schema::eve_characters::dsl::*;

        let db = request.guard::<EveDatabase>()?;        

        let user_id: i32 = match request.cookies()
            .get("char_id")
            .and_then(|cookie| cookie.value().parse().ok()) {
                Some(user_id) => user_id,
                None => return rocket::Outcome::Forward(())
            };

        let raccess_token: String = match request.cookies()
            .get("key")
            .map(|cookie| cookie.value().to_owned()) {
                Some(raccess_token) => raccess_token,
                None => return rocket::Outcome::Forward(())
            };

        let eve_character = eve_characters
            .filter(id.eq(user_id).and(access_token.eq(raccess_token)))
            .first::<EveCharacter>(&db.0);

        match eve_character {
            Ok(eve_character) => rocket::Outcome::Success(eve_character),
            Err(_) => rocket::Outcome::Forward(())
        }
    }

}