use separator::FixedPlaceSeparatable;
use crate::models::CompleteTransactionView;

#[derive(Serialize)]
pub struct DashboardViewModel {
    character_name: String,
    transactions: Vec<ViewTransaction>,
    per_day: Vec<DayProfit>,
}

impl DashboardViewModel {
    pub fn new(character_name: String, mut transactions: Vec<ViewTransaction>, per_day: Vec<DayProfit>) -> Self {
        transactions.sort_by_key(|f| {
            std::cmp::Reverse(f.transaction_id)
        });
        DashboardViewModel {
            character_name,
            transactions,
            per_day
        }        
    }
}

#[derive(Serialize)]
pub struct ViewProfit {
    date_time: String,
    
}

#[derive(Serialize)]
pub struct ViewTransaction {
    type_name: String,
    type_id: i32,
    transaction_id: i64,
    date_time: String,
    is_buy_text: String,
    quantity: i32,
    unit_price: String,    
    taxes: String,
    profit: String,
    markup_percentage: String,
    time_span: String,
}

impl From<CompleteTransactionView> for ViewTransaction {
    fn from(transaction: CompleteTransactionView) -> Self {

        let buy_text = if transaction.is_buy { String::from("B")} else { String::from("S") };

        let elapsed = if let Some(d) = transaction.buy_date {
            let elapsed = transaction.sell_date - d;
            let date = (elapsed.num_weeks(), elapsed.num_days(), elapsed.num_hours(), elapsed.num_minutes(), elapsed.num_seconds());
            match date {
                (0, 0, 0, 0, a) => format!("{} s", a),
                (0, 0, 0, a, _) => format!("{} m", a),
                (0, 0, a, _, _) => format!("{} h", a),
                (0, a, _, _, _) => format!("{} d", a),
                (a, _, _, _, _) => format!("{} w", a),                
            }
        } else {
            String::from("")
        };

        // Reminder that if this is a buy transaction, then the transaction information is stored in the sell parts
        let (profit, taxes, markup) = if transaction.is_buy {
            let profit = transaction.sell_unit_tax * transaction.quantity as f32;
            let taxes = transaction.sell_unit_tax * transaction.quantity as f32;

            (profit, taxes, String::from(""))
        } else {
            let profit = (transaction.sell_unit_price - transaction.buy_unit_price.unwrap_or_default() - transaction.buy_unit_tax.unwrap_or_default() - transaction.sell_unit_tax) * transaction.quantity as f32;
            let taxes = (transaction.buy_unit_tax.unwrap_or_default() + transaction.sell_unit_tax) * transaction.quantity as f32;
            let markup_percentage = transaction.buy_unit_price.map_or(String::from(""), |x| {
                ((transaction.sell_unit_price / x - 1.0) * 100.0).separated_string_with_fixed_place(2)
            });

            (profit, taxes, markup_percentage)
        };        

        ViewTransaction {
            type_name: transaction.type_name,
            type_id: transaction.type_id,
            transaction_id: transaction.sell_transaction_id,
            date_time: transaction.sell_date.format("%v %T").to_string(),
            is_buy_text: buy_text,
            quantity: transaction.quantity,
            unit_price: transaction.sell_unit_price.separated_string_with_fixed_place(2),            
            taxes: taxes.separated_string_with_fixed_place(2),
            profit: profit.separated_string_with_fixed_place(2),
            markup_percentage: markup,
            time_span: elapsed
        }
    }
}

#[derive(Serialize)]
pub struct DayProfit {
    pub date: String,
    pub isk_buy: String,
    pub isk_sell: String,
    pub revenue: String,
    pub taxes: String,    
    pub profit: String
}