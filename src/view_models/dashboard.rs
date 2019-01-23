#[derive(Serialize)]
pub struct DashboardViewModel {
    character_name: String,
    bought: Vec<BoughtTransaction>,
    sold: Vec<SoldTransaction>
}

impl DashboardViewModel {
    pub fn new(character_name: String, bought: Vec<BoughtTransaction>, sold: Vec<SoldTransaction>) -> Self {
        DashboardViewModel {
            character_name,
            bought, sold
        }        
    }
}

#[derive(Serialize)]
pub struct BoughtTransaction {
    date_time: String,
    is_buy_text: String,
    quantity: i32,
    unit_price: String,
    total_price: String,
}

impl BoughtTransaction {
    pub fn new(date_time: chrono::NaiveDateTime, is_buy: bool, quantity: i32, unit_price: f32, total_price: f32) -> Self {

        BoughtTransaction {
            date_time: date_time.format("%v %T").to_string(),
            is_buy_text: String::from("B"),
            quantity,
            unit_price: format!("{:.02}", unit_price),
            total_price: format!("{:.02}", total_price)
        }
    }
}

#[derive(Serialize)]
pub struct SoldTransaction {
    date_time: String,
    is_buy_text: String,
    quantity: i32,
    unit_price: String,
    total_price: String,
    profit: String,
    markup_percentage: String,
    time_span: String,
}