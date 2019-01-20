use rocket::Route;
use rocket::response::*;
use rocket::http::{Cookies, Cookie, Status};
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

#[get("/update")]
pub fn update(eve_character: EveCharacter, mut client: auth::AuthedClient) -> String {
    use crate::esi::wallet::{EsiWallet, EsiWalletTransactions};
    use restson::RestPath;

    let wallet: EsiWallet = match client.0.get(eve_character.id) {
        Ok(wallet) => wallet,
        Err(e) => return format!("Something went wrong: {:?}", e)
    };
    
    let transactions: EsiWalletTransactions = client.0.get(eve_character.id).unwrap();
    
    let transactions = transactions.0.into_iter().map(|t| {
        format!("date: {:?}, bought: {}, quantity: {}, type: {}, unit price: {:.2}\n", t.date, t.is_buy, t.quantity, t.type_id, t.unit_price)
    }).collect::<String>();

    format!("Wallet: {:.2},\n Transactions:\n {}", wallet.0, transactions)
}

#[get("/update", rank = 2)]
pub fn update_error() -> Status {
    Status::new(403, "Not authenticated")    
}

pub fn get_routes() -> Vec<Route> {
    routes![index, dashboard, update_error, update]
}