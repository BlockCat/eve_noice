use rocket::Route;
use rocket::response::*;
use rocket::request::Form;
use rocket::http::{Cookies, Cookie, Status};
use rocket_contrib::templates::*;

use jwt::Token;
use jwt::header::Header;
use jwt::claims::Claims;

use crate::auth;
use crate::models::EveCharacter;
use crate::EveDatabase;
use crate::view_models::{ CharacterConfigViewModel, CharacterRedirectViewModel };

#[get("/")]
pub fn index_authed(_character: EveCharacter) -> Redirect {
    Redirect::to("/")
}

#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::permanent(uri!("/characters", login))
}

#[get("/login")]
pub fn login() -> Redirect {
    let config = auth::create_config();
    Redirect::to(config.authorize_url().as_str().to_owned())
}

#[get("/config", rank = 2)]
pub fn config_unauth() -> Status {
    Status::new(403, "Not authenticated")
}
#[get("/config")]
pub fn config(character: EveCharacter) -> Template {
    let view_model = CharacterConfigViewModel {
        flash: None,
        sell_tax: format!("{:.02}", 100.0 * character.sell_tax),
        broker_fee: format!("{:.02}", 100.0 * character.broker_fee),
    };
    Template::render("characters/config", view_model)
}

#[derive(FromForm)]
pub struct EveConfig {
    sell_tax: String,
    broker_fee: String
}

#[post("/config", data = "<config>")]
pub fn config_post(mut character: EveCharacter, db: EveDatabase, config: Option<Form<EveConfig>>) -> Template {

    if config.is_none() {
        let view_model = CharacterConfigViewModel {
            flash: Some("You provided an invalid config".to_owned()),
            sell_tax: format!("{:.02}", 100.0 * character.sell_tax),
            broker_fee: format!("{:.02}", 100.0 * character.broker_fee),
        };
        return Template::render("characters/config", view_model);
    }

    let config = config.unwrap();

    let sell_tax: Option<f32> = config.sell_tax.replace(",", ".").parse().ok();
    let broker_fee: Option<f32> = config.broker_fee.replace(",", ".").parse().ok();

    

    let (sell_tax, broker_fee) = match (sell_tax, broker_fee) {
        (Some(st), Some(bf)) => (st, bf),
        _ => {
            let view_model = CharacterConfigViewModel {
                flash: Some("You provided an invalid config".to_owned()),
                sell_tax: format!("{:.02}", sell_tax.unwrap_or(character.sell_tax * 100.0)),
                broker_fee: format!("{:.02}", broker_fee.unwrap_or(character.broker_fee * 100.0)),
            };
            return Template::render("characters/config", view_model);
        }
    };

    character.sell_tax = sell_tax / 100.0;
    character.broker_fee = broker_fee / 100.0;

    let result = {
        use crate::schema::eve_characters::dsl::*;
        use diesel::prelude::*;
        diesel::update(eve_characters.filter(id.eq(character.id)))
            .set((sell_tax.eq(character.sell_tax), broker_fee.eq(character.broker_fee)))
            .execute(&db.0)
    }.ok();

    let message = match result {
        Some(_) => "Saved config",
        _ => "Failed saving config, pls send me an EVE Mail."
    };

    let view_model = CharacterConfigViewModel {
        flash: Some(message.to_owned()),
        sell_tax: format!("{:.02}", 100.0 * character.sell_tax),
        broker_fee: format!("{:.02}", 100.0 * character.broker_fee),
    };
    Template::render("characters/config", view_model)
}


#[get("/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {    
    cookies.remove_private(Cookie::named("key"));
    
    Redirect::to("/")
}


//TODO: To Result return
#[get("/callback?<code>&<state>")]
#[allow(unused)]
pub fn callback(code: String, state: String, conn: EveDatabase, mut cookies: Cookies) -> std::result::Result<Template, String> {
    let config = auth::create_config();

    let token_result = config.exchange_code(code).map_err(|_| {
        String::from("Token error: An incorrect code was provided.")
    })?;

    let (id, name) = {
        let jwt = Token::<Header, Claims>::parse(&token_result.access_token).map_err(|_| {
            String::from("Could not parse your token.")
        })?;

        let id = jwt.claims.reg.sub.unwrap().split(':').nth(2).map(|e| e.parse().unwrap()).expect("Couldn't extract id");    
        let name = jwt.claims.private["name"].as_string().unwrap().to_owned();

        (id, name)
    };
    
    let access_token = token_result.access_token;
    let refresh_token = token_result.refresh_token.expect("Not sure what should be here, but man did you do something wrong.\n No refresh token");
    let expiry_date = token_result.expires_in.expect("Not sure what should be here, but man did you do something wrong.\n No expiry date");
    
    let (existing, new_character) = match EveCharacter::find(id, &conn).ok() {
        Some(mut character) => {
            character.access_token = access_token;
            character.refresh_token = refresh_token;
            character.expiry_date = (chrono::Utc::now() + chrono::Duration::seconds(i64::from(expiry_date) - 60)).naive_utc();
            (true, character)
        },
        _ => (false, EveCharacter::new(id, name, access_token.clone(), refresh_token, expiry_date))
    };

    if !existing {
        new_character.insert(&conn).map_err(|e| {
            print!("{:?}", e);
            String::from("Something went wrong when adding character to the database.")
        })?;
    } else {
        use crate::schema::eve_characters::dsl::*;
        use diesel::prelude::*;
        diesel::update(eve_characters.filter(id.eq(new_character.id)))
            .set((access_token.eq(new_character.access_token.clone()), refresh_token.eq(new_character.refresh_token), expiry_date.eq(new_character.expiry_date)))
            .execute(&conn.0).map_err(|e| {
                print!("{:?}", e);
                String::from("Could not update character when logging in.")
            })?;
    }
    
    let cookie_char_id = Cookie::build("key", id.to_string())
        .path("/")
        //.secure(true)
        .finish();

    cookies.add_private(cookie_char_id);    

    println!("Adding cookies");
    
    let redirect = if existing {
        "/".to_owned()
    } else {
        "/characters/config".to_owned()
    };
    
    Ok(Template::render("characters/redirecting", CharacterRedirectViewModel {
        uri_link: redirect
    }))   
}

pub fn get_routes() -> Vec<Route> {
    routes![index, index_authed, login, logout, config_unauth, config, config_post, callback]
}