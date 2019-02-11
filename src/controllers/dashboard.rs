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
use crate::view_models::{ DashboardViewModel, ViewTransaction, DayProfit, TypeProfit };

#[get("/?<days>")]
pub fn dashboard(eve_character: EveCharacter, db: EveDatabase, days: Option<i64>) -> Template {
    use std::cmp::{ max, min };
    let days = min(30, days.unwrap_or(7));
    let days = max(0, days);

    let transactions = crate::repository::view_transactions(eve_character.id, days, &db).expect("Could not get transactions");    
    
    let profits = get_transactions_view(&transactions);
    let per_day = get_profit_per_day(&transactions);
    let type_profit = get_max_profits(&transactions);
        
    Template::render("dashboard/dashboard", DashboardViewModel::new(
        eve_character.name, profits, per_day, type_profit, days
    ))
}

#[get("/", rank = 2)]
pub fn index() -> Template {
    Template::render("dashboard/index", std::collections::HashMap::<String, String>::new())
}

// TODO: When updating we probably want some lock or something?
#[get("/update")]
pub fn update(eve_character: EveCharacter, mut client: auth::AuthedClient, db: EveDatabase) -> Redirect {

    let last_updated = eve_character.last_update;
    let duration = Utc::now().naive_utc() - last_updated;

    if duration.num_seconds() < 3600 {        
        return Redirect::to(uri!(dashboard: _));
    }

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
    println!("Retrieved esi_transactions");
    // Collect into map
    let esi_transactions: Vec<_> = esi_transactions.into_iter()
        .take_while(|x| x.date > latest_transaction_date)        
        .map(|x| {
            let taxes = if x.is_buy {
                x.unit_price * (eve_character.broker_fee)
            } else {
                x.unit_price * (eve_character.broker_fee + eve_character.sell_tax)
            };

            WalletTransaction::new(eve_character.id, x, taxes)
        }).collect();

    WalletTransaction::upsert_batch(&db, &esi_transactions).expect("Could not update the database");

    println!("Inserted esi_transactions");

    let transaction_queue = esi_transactions.iter()
        .filter(|x| x.is_buy)
        .map(|x| TransactionQueue::new(eve_character.id, x.type_id, x.transaction_id, x.quantity))
        .collect::<Vec<_>>();

    TransactionQueue::upsert_batch(&db, &transaction_queue).expect("Could not insert bought into queue");
    
    for transaction in esi_transactions.iter().rev().filter(|x| !x.is_buy) {        
        // Now take it from queue
        let mut quantity_left = transaction.quantity;
        let mut to_be_upserted = Vec::new();
        let mut to_be_deleted = Vec::new();
        let mut complete_transactions = Vec::new();
        let mut page = 0;
        // While there is a quantity left that needs to be processed
        while quantity_left > 0 {            
            // Take first transaction in the queue of the type that has a quantity left.
            let latest = TransactionQueue::find_latest(eve_character.id, transaction.type_id, transaction.date, 20, page, &db).expect("Could not get latest transacations");            
            if !latest.is_empty() {
                for (mut latest, buy_transaction) in latest {    

                    if quantity_left == 0 { // No more left
                        break;
                    }                
                    let quantity_taken = std::cmp::min(quantity_left, latest.amount_left);
                    latest.amount_left -= quantity_taken;

                    if latest.amount_left > 0 {
                        to_be_upserted.push(latest); // Add to transactions needed to be upsert
                    } else { 
                        to_be_deleted.push(latest); // Add to transactions needed to be deleted
                    }

                    quantity_left -= quantity_taken;
                    complete_transactions.push(CompleteTransaction::new(eve_character.id, buy_transaction, transaction, quantity_taken));
                }
            } else {

                // Set quantity to 0
                complete_transactions.push(CompleteTransaction::new(eve_character.id, None, transaction, quantity_left));
                quantity_left = 0;
            }

            page += 1;
            TransactionQueue::upsert_batch(&db, &to_be_upserted).expect("Could not update transaction queue");
            TransactionQueue::delete_batch(&db, &to_be_deleted).expect("Could not delete from transaction queue");
            CompleteTransaction::upsert_batch(&db, &complete_transactions).expect("Could not insert complete transactions");
            println!();
        }
    }

    // Merge the two into one by
    let mut eve_character = eve_character;
    eve_character.last_update = Utc::now().naive_utc();
    eve_character.upsert(&db).expect("Could not update eve character");


    Redirect::to(uri!(dashboard: _))
}

#[get("/update", rank = 2)]
pub fn update_error() -> Status {
    Status::new(403, "Not authenticated")    
}

pub fn get_routes() -> Vec<Route> {
    routes![index, dashboard, update_error, update]
}

fn get_transactions_view(transactions: &[CompleteTransactionView]) -> Vec<ViewTransaction> {
    transactions.iter()
        .map(|x| x.into())
        .collect()
}

fn get_profit_per_day(transactions: &[CompleteTransactionView]) -> Vec<DayProfit> {
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

fn get_max_profits(transactions: &[CompleteTransactionView]) -> Vec<TypeProfit> {
    let mut mapping = HashMap::<String, (f32, i64, i64)>::new();

    for transaction in transactions.iter().filter(|x| !x.is_buy && x.buy_unit_price.is_some()) {
        let entry = mapping.entry(transaction.type_name.clone()).or_insert((0.0, 0, 0));
        let sell_price = transaction.sell_unit_price;        
        let sell_tax = transaction.sell_unit_tax;
        let buy_price = transaction.buy_unit_price.unwrap();
        let buy_tax = transaction.buy_unit_tax.unwrap();
        let amount = transaction.quantity as f32;

        let duration = transaction.sell_date - transaction.buy_date.unwrap();

        entry.0 += (sell_price - buy_price - sell_tax - buy_tax) * amount;
        entry.1 += duration.num_seconds();
        entry.2 += 1;
    }

    let mut profits: Vec<_> = mapping.into_iter().map(|(item_name, (profit, seconds, items))| {
        TypeProfit {
            item_name,
            profit,
            avg_time: seconds / items,
            avg_profit: (profit * 86400.0) / seconds as f32
        }
    }).collect();
    profits.sort_by_key(|x| std::cmp::Reverse((x.avg_profit * 100.0) as i32)); // sigh.

    profits
}