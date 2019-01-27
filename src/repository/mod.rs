use crate::models::{ CompleteTransactionView, WalletTransaction};
use separator::FixedPlaceSeparatable;
use crate::view_models::DayProfit;
use crate::EveDatabase;
use std::collections::HashMap;
use diesel::prelude::*;
use chrono::{ NaiveDateTime, NaiveTime, NaiveDate };

pub fn view_transactions(character_tid: i32, days: i64, conn: &EveDatabase) -> QueryResult<Vec<CompleteTransactionView>> {
    use crate::complete_transactions_views::dsl::*;    
    let back_date = (chrono::Utc::today() - chrono::Duration::days(days)).naive_utc();
    let back_date = NaiveDateTime::new(back_date, NaiveTime::from_hms(0, 0, 0));
    complete_transactions_views.filter(character_id.eq(character_tid).and(sell_date.ge(back_date)))
        .load(&conn.0)
} 

pub fn view_profit_per_day(character_tid: i32, days: i64, conn: &EveDatabase) -> Vec<DayProfit> {        
    let mut mapping = HashMap::<NaiveDate, TempProfit>::with_capacity(days as usize);

    struct TempProfit {
        pub date: chrono::NaiveDate,
        pub isk_buy: f32,
        pub isk_sell: f32,
        pub revenue: f32,
        pub taxes: f32,    
        pub profit: f32
    }

    view_transactions(character_tid, days, conn)
        .map(|x| {
            // Reminder that if the transaction is buy, it will be saved in sell fields
            for transaction in x {
                let tdate = transaction.sell_date.date();
                let entry = mapping.entry(tdate).or_insert(TempProfit {
                    date: tdate,
                    isk_buy: 0.0,
                    isk_sell: 0.0,
                    revenue: 0.0,
                    taxes: 0.0,
                    profit: 0.0
                });
                if transaction.is_buy {
                    entry.isk_buy += transaction.sell_unit_price * transaction.quantity as f32;                    
                    //entry.taxes += transaction.sell_unit_tax * transaction.quantity as f32;
                } else {
                    entry.isk_sell += transaction.sell_unit_price * transaction.quantity as f32;
                    entry.taxes += (transaction.sell_unit_tax + transaction.buy_unit_tax.unwrap_or(0.0)) * transaction.quantity as f32;
                    entry.revenue += (transaction.sell_unit_price - transaction.buy_unit_price.unwrap_or(0.0)) * transaction.quantity as f32;                    
                }
            }

            for vals in mapping.values_mut() {
                vals.profit = vals.revenue - vals.taxes;
            }
        }).expect("Could not get wallet transactions");
    let mut profit_days: Vec<TempProfit> = mapping.into_iter().map(|x| x.1).collect();
    profit_days.sort_by_key(|x| std::cmp::Reverse(x.date));

    let profit_days = profit_days.into_iter()
            .map(|temp| DayProfit {
                date: temp.date.format("%v").to_string(),
                isk_buy: temp.isk_buy.separated_string_with_fixed_place(2),
                isk_sell: temp.isk_sell.separated_string_with_fixed_place(2),
                revenue: temp.revenue.separated_string_with_fixed_place(2),
                taxes: temp.taxes.separated_string_with_fixed_place(2),
                profit: temp.profit.separated_string_with_fixed_place(2)
            }).collect();
    

    profit_days    
}