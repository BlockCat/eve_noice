#[derive(Serialize)]
pub struct InventoryViewModel {
    pub items: Vec<InventoryItem>
}

#[derive(Serialize)]
pub struct InventoryItem {
    pub date: chrono::NaiveDate,
    pub invtype: String,
    pub amount: i32,
    pub isk_buy: f32,
}