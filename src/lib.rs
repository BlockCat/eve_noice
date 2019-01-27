#![feature(plugin, proc_macro_hygiene, decl_macro)]

// Clippy stuff
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv_codegen;

extern crate dotenv;
extern crate chrono;
extern crate oauth2;
extern crate jwt;
extern crate ammonia;
extern crate restson;
extern crate separator;
extern crate itertools;

pub mod models;
pub mod schema;
pub mod view_models;
pub mod repository;

mod auth;
mod esi;
mod controllers;

use rocket_contrib::templates::{ Template, Engines };
use rocket_contrib::templates::tera;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;

#[database("eve_db")]
pub struct EveDatabase(diesel::SqliteConnection);

pub fn rocket_factory() -> Result<rocket::Rocket, String> {    
    // Source environment variable
    dotenv::dotenv().ok();

    let tera = Template::custom(|engines: &mut Engines| {
        
        engines.tera.register_filter("isk", isk_filter);
    });

    let rocket = rocket::ignite()
        .attach(EveDatabase::fairing())
        .attach(tera)
        .mount("/", controllers::dashboard::get_routes())
        .mount("/characters", controllers::characters::get_routes())
        .mount("/public", StaticFiles::from("assets"));

    Ok(rocket)
}

fn isk_filter(value: tera::Value, _: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    use separator::FixedPlaceSeparatable;
    if value.is_f64() {
        let isk = value.as_f64().unwrap().separated_string_with_fixed_place(2);
        Ok(isk.into())
    } else {
        Err(tera::Error::from(tera::ErrorKind::Msg("Was not an f64".to_owned())))
    }
}

table! {
    complete_transactions_views (sell_transaction_id, buy_transaction_id) {
        type_name -> Text,
        type_id -> Integer,
        character_id -> Integer,
        sell_transaction_id -> BigInt,
        buy_transaction_id -> Nullable<BigInt>,
        sell_date -> Timestamp,
        buy_date -> Nullable<Timestamp>,
        is_buy -> Bool,
        quantity -> Integer,
        buy_unit_price -> Nullable<Float>,
        buy_unit_tax -> Nullable<Float>,
        sell_unit_price -> Float,
        sell_unit_tax -> Float,        
        
    }
 }