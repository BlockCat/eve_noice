use rocket::Route;
use rocket::response::*;
use rocket::http::{Cookies, Cookie};

use jwt::Token;
use jwt::header::Header;
use jwt::claims::Claims;

use crate::auth;
use crate::models::EveCharacter;
use crate::EveDatabase;



#[get("/", rank = 2)]
pub fn index() -> &'static str {
    "Log in now"
}



#[get("/login")]
pub fn login() -> Redirect {
    let config = auth::create_config();
    Redirect::to(config.authorize_url().as_str().to_owned())
}


#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {    
    cookies.remove_private(Cookie::named("key"));
    
    Redirect::to("/")
}


//TODO: To Result return
#[get("/callback?<code>&<state>")]
pub fn callback(code: String, state: String, conn: EveDatabase, mut cookies: Cookies) -> std::result::Result<Redirect, String> {
    let config = auth::create_config();

    let token_result = config.exchange_code(code).map_err(|_| {
        String::from("Token error: An incorrect code was provided.")
    })?;

    let (id, name) = {
        let jwt = Token::<Header, Claims>::parse(&token_result.access_token).map_err(|_| {
            String::from("Could not parse your token.")
        })?;

        let id = jwt.claims.reg.sub.unwrap().split(":").nth(2).map(|e| e.parse().unwrap()).expect("Couldn't extract id");    
        let name = jwt.claims.private["name"].as_string().unwrap().to_owned();

        (id, name)
    };
    
    let access_token = token_result.access_token;
    let refresh_token = token_result.refresh_token.expect("Not sure what should be here, but man did you do something wrong.\n No refresh token");
    let expiry_date = token_result.expires_in.expect("Not sure what should be here, but man did you do something wrong.\n No expiry date");
    
    let new_character = EveCharacter::new(id, name, access_token.clone(), refresh_token, expiry_date);

    new_character.upsert(&conn).map_err(|e| {
        print!("{:?}", e);
        String::from("Something went wrong when adding character to the database.")
    })?;
    
    let cookie_char_id = Cookie::build("key", id.to_string())
        .path("/")
        //.secure(true)
        .finish();

    cookies.add_private(cookie_char_id);    

    println!("Adding cookies");
    
    Ok(Redirect::to(uri!(index)))
    
}

pub fn get_routes() -> Vec<Route> {
    routes![index, login, logout, callback]
}