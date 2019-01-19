#![feature(plugin, proc_macro_hygiene, decl_macro)]

// Clippy stuff
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate oauth2;
extern crate jwt;
extern crate ammonia;
extern crate hyper;

pub mod models;
pub mod schema;

mod auth;
mod controllers;
mod esi;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

#[database("eve_db")]
pub struct EveDatabase(diesel::SqliteConnection);

pub fn rocket_factory() -> Result<rocket::Rocket, String> {    
    let rocket = rocket::ignite()
        .attach(EveDatabase::fairing())
        .attach(Template::fairing())
        .mount("/", controllers::dashboard::get_routes())
        .mount("/characters", controllers::characters::get_routes())
        .mount("/public", StaticFiles::from("assets"));

    Ok(rocket)
}
