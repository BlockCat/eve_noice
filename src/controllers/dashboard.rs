use rocket::Route;
use rocket::response::*;
use rocket::http::{Cookies, Cookie};
use rocket_contrib::templates::*;

use jwt::Token;
use jwt::header::Header;
use jwt::claims::Claims;

use crate::auth;
use crate::models::EveCharacter;
use crate::EveDatabase;

#[derive(Serialize)]
struct DashBoardTemplate {
    character: EveCharacter
}

#[get("/")]
pub fn dashboard(eve_character: EveCharacter) -> Template {
    Template::render("dashboard/dashboard", DashBoardTemplate {
        character: eve_character
    })
}

#[get("/", rank = 2)]
pub fn index() -> Template {
    Template::render("dashboard/index", std::collections::HashMap::<String, String>::new())
}

pub fn get_routes() -> Vec<Route> {
    routes![index, dashboard]
}