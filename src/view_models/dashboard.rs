
use crate::models::CompleteTransactionView;

#[derive(Serialize)]
pub struct DashboardViewModel {
    days: i64,
    character_name: String,
    transactions: Vec<ViewTransaction>,
    per_day: Vec<DayProfit>,
    type_profits: Vec<TypeProfit>
}

impl DashboardViewModel {
    pub fn new(character_name: String, mut transactions: Vec<ViewTransaction>, per_day: Vec<DayProfit>, type_profits: Vec<TypeProfit>, days: i64) -> Self {
        transactions.sort_by_key(|f| {
            std::cmp::Reverse(f.transaction_id)
        });
        DashboardViewModel {
            days,
            character_name,
            transactions,
            per_day,
            type_profits,
        }        
    }
}

#[derive(Serialize)]
pub struct ViewProfit {
    date_time: String,
    
}

#[derive(Serialize, Clone)]
pub struct ViewTransaction {
    type_name: String,
    type_id: i32,
    pub transaction_id: i64,
    pub date_time: chrono::NaiveDateTime,
    pub is_buy_text: String,
    pub quantity: i32,
    pub unit_price: f32,    
    pub taxes: f32,
    pub profit: f32,
    pub markup_percentage: f32,
    pub time_span: i64,
}

impl<'a> std::iter::Sum<&'a ViewTransaction> for ViewTransaction {
    fn sum<I: Iterator<Item=&'a ViewTransaction>>(mut iter: I) -> Self {
        let base = iter.next().unwrap().clone();
        iter.fold(base, |x, acc| {
            ViewTransaction {
                type_name: acc.type_name.clone(),
                type_id: acc.type_id,
                transaction_id: x.transaction_id,
                date_time: acc.date_time,
                is_buy_text: acc.is_buy_text.clone(),
                quantity: acc.quantity + x.quantity,
                unit_price: acc.unit_price,
                taxes: acc.taxes + x.taxes,
                profit: acc.profit + x.profit,
                markup_percentage: acc.markup_percentage + x.markup_percentage,
                time_span: acc.time_span + x.time_span
            }
        })
    }
}

impl From<CompleteTransactionView> for ViewTransaction {
    fn from(transaction: CompleteTransactionView) -> Self {
        (&transaction).into()
    }
}

impl From<&CompleteTransactionView> for ViewTransaction {
    fn from(transaction: &CompleteTransactionView) -> Self {

        let buy_text = if transaction.is_buy { String::from("B")} else { String::from("S") };

        let elapsed = if let Some(d) = transaction.buy_date {
            (transaction.sell_date - d).num_seconds()
        } else {
            -1
        };

        // Reminder that if this is a buy transaction, then the transaction information is stored in the sell parts
        let (profit, taxes, markup) = if transaction.is_buy {
            let profit = -transaction.sell_unit_tax * transaction.quantity as f32;
            let taxes = transaction.sell_unit_tax * transaction.quantity as f32;

            (profit, taxes, 0.0)
        } else {
            let profit = (transaction.sell_unit_price - transaction.buy_unit_price.unwrap_or_default() - transaction.buy_unit_tax.unwrap_or_default() - transaction.sell_unit_tax) * transaction.quantity as f32;
            let taxes = (transaction.buy_unit_tax.unwrap_or_default() + transaction.sell_unit_tax) * transaction.quantity as f32;
            let markup_percentage = transaction.buy_unit_price.map_or(0.0, |x| {
                let p = transaction.sell_unit_price - x;
                ((p / x) * 100.0)
            });

            (profit, taxes, markup_percentage)
        };        

        ViewTransaction {
            type_name: transaction.type_name.clone(),
            type_id: transaction.type_id,
            transaction_id: transaction.sell_transaction_id,
            date_time: transaction.sell_date,
            is_buy_text: buy_text,
            quantity: transaction.quantity,
            unit_price: transaction.sell_unit_price,            
            taxes,
            profit,
            markup_percentage: markup,
            time_span: elapsed
        }
    }
}

#[derive(Serialize)]
pub struct DayProfit {
    pub date: chrono::NaiveDate,
    pub isk_buy: f32,
    pub isk_sell: f32,
    pub revenue: f32,
    pub taxes: f32,    
    pub profit: f32
}

#[derive(Serialize)]
pub struct TypeProfit {
    pub item_name: String,
    pub profit: f32,
    pub avg_time: i64,
    pub avg_profit: f32
}