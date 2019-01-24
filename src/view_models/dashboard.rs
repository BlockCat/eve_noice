use separator::FixedPlaceSeparatable;
use crate::models::{ WalletTransaction, CompleteTransaction, InvType };

#[derive(Serialize)]
pub struct DashboardViewModel {
    character_name: String,
    transactions: Vec<ViewTransaction>,    
}

impl DashboardViewModel {
    pub fn new(character_name: String, mut transactions: Vec<ViewTransaction>) -> Self {
        transactions.sort_by_key(|f| {
            std::cmp::Reverse(f.transaction_id)
        });
        DashboardViewModel {
            character_name,
            transactions
        }        
    }
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
    total_price: String,
    profit: String,
    markup_percentage: String,
    time_span: String,
}

impl From<(WalletTransaction, InvType)> for ViewTransaction {
    fn from((transaction, item): (WalletTransaction, InvType)) -> Self {
        let total_price = transaction.unit_price * transaction.quantity as f32;
        let total_taxes = transaction.unit_taxes * transaction.quantity as f32;
        ViewTransaction {
            type_name: item.type_name,
            type_id: item.type_id,
            transaction_id: transaction.transaction_id,
            date_time: transaction.date.format("%v %T").to_string(),
            is_buy_text: String::from(if transaction.is_buy { "B" } else { "S" } ),
            quantity: transaction.quantity,
            unit_price: transaction.unit_price.separated_string_with_fixed_place(2),
            total_price: total_price.separated_string_with_fixed_place(2),
            profit: total_taxes.separated_string_with_fixed_place(2),
            markup_percentage: String::from(""),
            time_span: String::from("")
        }
    }
}

impl From<(CompleteTransaction, WalletTransaction, InvType, Option<WalletTransaction>)> for ViewTransaction {
    fn from((complete, sold, item, bought): (CompleteTransaction, WalletTransaction, InvType, Option<WalletTransaction>)) -> Self {
        
        let (bought_price, bought_taxes, bought_date) = bought.map_or((0.0, 0.0, String::from("")), |x| {
            let bought_price = x.unit_price;
            let bought_taxes = x.unit_taxes * complete.amount as f32;
            let elapsed = sold.date - x.date;

            let elapsed = match (elapsed.num_weeks(), elapsed.num_days(), elapsed.num_hours(), elapsed.num_minutes(), elapsed.num_seconds()) {
                (0, 0, 0, 0, a) => format!("{} s", a),
                (0, 0, 0, a, _) => format!("{} m", a),
                (0, 0, a, _, _) => format!("{} h", a),
                (0, a, _, _, _) => format!("{} d", a),
                (a, _, _, _, _) => format!("{} w", a),                
            };

            (bought_price, bought_taxes, elapsed)
        });
        
        let profit = complete.amount as f32 * (sold.unit_price - bought_price);


        ViewTransaction {
            type_name: item.type_name,
            type_id: item.type_id,
            transaction_id: sold.transaction_id,
            date_time: sold.date.format("%v %T").to_string(),
            is_buy_text: String::from(if sold.is_buy { "B" } else { "S" } ),
            quantity: complete.amount,
            unit_price: sold.unit_price.separated_string_with_fixed_place(2),
            total_price: (sold.unit_price * complete.amount as f32).separated_string_with_fixed_place(2),
            profit: (profit - bought_taxes - 0.038 * profit ).separated_string_with_fixed_place(2),
            markup_percentage: ((sold.unit_price / bought_price - 1.0) * 100.0).separated_string_with_fixed_place(2),
            time_span: String::from(bought_date)
        }
    }
}