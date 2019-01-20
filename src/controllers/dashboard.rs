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
    use crate::esi::wallet::{EsiWallet, EsiWalletTransactions, EsiWalletJournals};
    use restson::RestPath;

    let wallet: EsiWallet = match client.0.get(eve_character.id) {
        Ok(wallet) => wallet,
        Err(e) => return format!("Something went wrong: {:?}", e)
    };

    // - Retrieve latest transaction t' from database
    // - Do retrieve transactions While transaction_id(t') < id(last retrieved transaction)    
    // - Do retrieve journals (with regards to market transactions) While journal_id(t') < journal_id(last retrieved transaction)
    
    // - Add all journals to database
    // - Add all transactions to database
    // - journals and transactions are now... connected.

    // - Add freshly bought items to database-queue
    // - Pop freshly sold items from database-queue (fifo)
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ THE MOST DIFFICULT PART?
    
    let transactions: EsiWalletTransactions = client.0.get(eve_character.id).unwrap();
    let journals: EsiWalletJournals = client.0.get(eve_character.id).unwrap();

    // Merge the two into one by

    format!("Wallet: {:.2},\n", wallet.0)
}

#[get("/update", rank = 2)]
pub fn update_error() -> Status {
    Status::new(403, "Not authenticated")    
}

pub fn get_routes() -> Vec<Route> {
    routes![index, dashboard, update_error, update]
}