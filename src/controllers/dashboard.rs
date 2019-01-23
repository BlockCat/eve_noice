use rocket::Route;
use rocket::http::Status;
use rocket_contrib::templates::*;

use chrono::{ DateTime, Utc };

use crate::auth;
use crate::EveDatabase;
use crate::esi::{EsiWallet, EsiWalletTransactions};
use crate::models::{ EveCharacter, WalletTransaction, TransactionQueue, CompleteTransaction};
use crate::view_models::{ DashboardViewModel, BoughtTransaction, SoldTransaction};

#[get("/")]
pub fn dashboard(eve_character: EveCharacter, db: EveDatabase) -> Template {

    let sold = WalletTransaction::all_buy(eve_character.id, &db).expect("Could not get all bought transactions of character")
        .into_iter()
        .map(|x| {
            BoughtTransaction::new(
                x.date, 
                x.is_buy,
                x.quantity,
                x.unit_price,
                x.quantity as f32 * x.unit_price)
        }).collect();

    let bought = vec!();// CompleteTransaction::all(eve_character.id, &db).expect("Could not get complete transactions")
        
    Template::render("dashboard/dashboard", DashboardViewModel::new(
        eve_character.name, sold, bought
    ))
}

#[get("/", rank = 2)]
pub fn index() -> Template {
    Template::render("dashboard/index", std::collections::HashMap::<String, String>::new())
}

// TODO: When updating we probably want some lock or something?
#[get("/update")]
pub fn update(eve_character: EveCharacter, mut client: auth::AuthedClient, db: EveDatabase) -> String {
    let wallet: EsiWallet = match client.0.get(eve_character.id) {
        Ok(wallet) => wallet,
        Err(e) => return format!("Something went wrong: {:?}", e)
    };

    // - Retrieve latest transaction t' from database
    // - Do retrieve transactions While transaction_id(t') < id(last retrieved transaction)

    // - Add all transactions to database
    // - journals and transactions are now... connected.

    // - Add freshly bought items to database-queue
    // - Pop freshly sold items from database-queue (fifo)
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ THE MOST DIFFICULT PART?

    let latest_transaction_date = match WalletTransaction::find_latest(eve_character.id, &db) {
        Ok(latest_transaction) => DateTime::from_utc(latest_transaction.date, Utc),
        Err(diesel::NotFound) => (chrono::Utc::now() - chrono::Duration::days(30)),
        Err(e) => panic!(e)
    };    

    // Walk back through esi transactions and journals
    // ----------------------------------------------------    
    let EsiWalletTransactions(mut esi_transactions) = client.0.get(eve_character.id).unwrap();

    println!("Starting walkback till: {:?}, right now: {:?}", latest_transaction_date, esi_transactions.last().unwrap().date);

    while esi_transactions.last().unwrap().date > latest_transaction_date {        
        let latest_id = esi_transactions.last().unwrap().transaction_id.to_string();
        let EsiWalletTransactions(new_transactions) = client.0.get_with(eve_character.id, &[("from_id", &latest_id)]).unwrap();
        println!("transactions: {:?}", new_transactions);
        esi_transactions.extend(new_transactions);        
    }

    // ----------------------------------------------------    

    // Collect into map

    println!("Retrieved: {:?}", esi_transactions);
    
    let esi_transactions: Vec<_> = esi_transactions.into_iter()
        .take_while(|x| x.date > latest_transaction_date)
        .map(|x| WalletTransaction::new(eve_character.id, x))
        .collect();

    WalletTransaction::upsert_batch(&db, &esi_transactions).expect("Could not update the database");

    let transaction_queue = esi_transactions.iter()
        .filter(|x| x.is_buy)
        .map(|x| TransactionQueue::new(eve_character.id, x.type_id, x.transaction_id, x.quantity))
        .collect::<Vec<_>>();

    TransactionQueue::upsert_batch(&db, &transaction_queue).expect("Could not insert bought into queue");

    for transaction in esi_transactions.iter().filter(|x| !x.is_buy) {
        // Now take it from queue
        let mut quantity_left = transaction.quantity;

        // While there is a quantity left that needs to be processed
        while quantity_left > 0 {
            // Take first transaction in the queue of the type that has a quantity left.
            let latest = TransactionQueue::find_latest(eve_character.id, transaction.type_id, &db).ok();

            // If that transaction actually exists:
            let (buy_transaction, quantity) = if let Some(mut latest) = latest {
                // The quantity take from this queue can not be more than quantity left.
                let quantity_taken = std::cmp::min(quantity_left, latest.amount_left);
                
                // Process quantity
                quantity_left -= quantity_taken;
                latest.amount_left -= quantity_taken;

                // Update the latest queue
                if latest.amount_left > 0 {
                    latest.upsert(&db).expect("Could not upsert transaction queue");
                } else {
                    latest.delete(&db).expect("Could not delete from transaction queue");
                }
                
                (Some(latest), quantity_taken)
            } else {
                // No latest transaction, this sell order is problematic :thinking:
                (None, quantity_left)
            };

            // TODO: Fix taxes
            // Ehm, something about creating a new Finished Transaction
            // Probably means that sell transactions don't need to be added to the database
            let complete_transaction = CompleteTransaction::new(
                eve_character.id,
                buy_transaction.map(|x| x.transaction_id), 
                transaction.transaction_id, quantity, 0.0);
            complete_transaction.upsert(&db).expect("Could not insert complete transaction into database.");
        }
        
    }
    

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