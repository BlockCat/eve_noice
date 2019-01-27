use std::collections::HashMap;

use rocket::Route;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_contrib::templates::*;

use chrono::{ NaiveDate, DateTime, Utc };

use crate::auth;
use crate::EveDatabase;
use crate::esi::EsiWalletTransactions;
use crate::models::{ EveCharacter, WalletTransaction, TransactionQueue, CompleteTransaction, CompleteTransactionView };
use crate::view_models::{ DashboardViewModel, ViewTransaction, DayProfit };

#[get("/")]
pub fn dashboard(eve_character: EveCharacter, db: EveDatabase) -> Template {

    let transactions = crate::repository::view_transactions(eve_character.id, 7, &db).expect("Could not get transactions");    
    
    let profits = get_transactions_view(&transactions);
    let per_day = get_profit_per_day(&transactions);
        
    Template::render("dashboard/dashboard", DashboardViewModel::new(
        eve_character.name, profits, per_day
    ))
}

#[get("/", rank = 2)]
pub fn index() -> Template {
    Template::render("dashboard/index", std::collections::HashMap::<String, String>::new())
}

// TODO: When updating we probably want some lock or something?
#[get("/update")]
pub fn update(eve_character: EveCharacter, mut client: auth::AuthedClient, db: EveDatabase) -> Redirect {

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
        let latest_id = (esi_transactions.last().unwrap().transaction_id - 1).to_string();
        let EsiWalletTransactions(new_transactions) = client.0.get_with(eve_character.id, &[("from_id", &latest_id)]).unwrap();
        if !new_transactions.is_empty() {            
            esi_transactions.extend(new_transactions);        
        } else {
            break;
        }        
    }

    // ----------------------------------------------------    

    // Collect into map
    let esi_transactions: Vec<_> = esi_transactions.into_iter()
        .take_while(|x| x.date > latest_transaction_date)        
        .map(|x| {
            let taxes = if x.is_buy {
                x.unit_price * 0.026
            } else {
                x.unit_price * 0.038
            };

            WalletTransaction::new(eve_character.id, x, taxes)
        })
        .collect();

    WalletTransaction::upsert_batch(&db, &esi_transactions).expect("Could not update the database");

    println!("Inserted transaction in database");

    let transaction_queue = esi_transactions.iter()
        .filter(|x| x.is_buy)
        .map(|x| TransactionQueue::new(eve_character.id, x.type_id, x.transaction_id, x.quantity))
        .collect::<Vec<_>>();

    TransactionQueue::upsert_batch(&db, &transaction_queue).expect("Could not insert bought into queue");

    println!("Inserted into bought queue");

    for transaction in esi_transactions.iter().rev().filter(|x| !x.is_buy) {
        // Now take it from queue
        let mut quantity_left = transaction.quantity;
        // While there is a quantity left that needs to be processed
        while quantity_left > 0 {
            print!("Quantity left: {}, ", quantity_left);
            // Take first transaction in the queue of the type that has a quantity left.
            let latest = TransactionQueue::find_latest(eve_character.id, transaction.type_id, transaction.date, &db).ok();

            println!("latest: {:?}", latest);

            // If that transaction actually exists:
            let (buy_transaction, quantity) = if let Some((mut latest, buy_transaction)) = latest {
                // The quantity take from this queue can not be more than quantity left.
                let quantity_taken = std::cmp::min(quantity_left, latest.amount_left);
                
                // Process quantity                
                latest.amount_left -= quantity_taken;

                // Update the latest queue
                if latest.amount_left > 0 {
                    latest.upsert(&db).expect("Could not upsert transaction queue");
                } else {
                    latest.delete(&db).expect("Could not delete from transaction queue");
                }
                
                (buy_transaction, quantity_taken)
            } else {
                // No latest transaction, this sell order is problematic :thinking:
                (None, quantity_left)
            };
            quantity_left -= quantity;
            // TODO: Fix taxes
            // Ehm, something about creating a new Finished Transaction
            // Probably means that sell transactions don't need to be added to the database

            let complete_transaction = CompleteTransaction::new(eve_character.id, buy_transaction, transaction, quantity);
            complete_transaction.upsert(&db).expect("Could not insert complete transaction into database.");
        }
        
    }
    

    // Merge the two into one by

    Redirect::to(uri!(dashboard))
}

#[get("/update", rank = 2)]
pub fn update_error() -> Status {
    Status::new(403, "Not authenticated")    
}

pub fn get_routes() -> Vec<Route> {
    routes![index, dashboard, update_error, update]
}

fn get_transactions_view(transactions: &Vec<CompleteTransactionView>) -> Vec<ViewTransaction> {
    transactions.iter()
        .map(|x| x.into())
        .collect()
}

fn get_profit_per_day(transactions: &Vec<CompleteTransactionView>) -> Vec<DayProfit> {
    let mut mapping = HashMap::<NaiveDate, DayProfit>::new();

    // Reminder that if the transaction is buy, it will be saved in sell fields
    for transaction in transactions {
        let tdate = transaction.sell_date.date();
        let entry = mapping.entry(tdate).or_insert(DayProfit {
            date: tdate,
            isk_buy: 0.0,
            isk_sell: 0.0,
            revenue: 0.0,
            taxes: 0.0,
            profit: 0.0
        });
        if transaction.is_buy {
            entry.isk_buy += transaction.sell_unit_price * transaction.quantity as f32;                                    
        } else {
            entry.isk_sell += transaction.sell_unit_price * transaction.quantity as f32;
            entry.taxes += (transaction.sell_unit_tax + transaction.buy_unit_tax.unwrap_or(0.0)) * transaction.quantity as f32;
            entry.revenue += (transaction.sell_unit_price - transaction.buy_unit_price.unwrap_or(0.0)) * transaction.quantity as f32;                    
        }
    }

    for vals in mapping.values_mut() {
        vals.profit = vals.revenue - vals.taxes;
    }
    
    let mut profit_days: Vec<DayProfit> = mapping.into_iter().map(|x| x.1).collect();
    profit_days.sort_by_key(|x| std::cmp::Reverse(x.date));
    profit_days    
}