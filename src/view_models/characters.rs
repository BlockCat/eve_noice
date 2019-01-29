#[derive(Serialize, Deserialize)]
pub struct CharacterConfigViewModel {
    pub flash: Option<String>,
    pub sell_tax: String,
    pub broker_fee: String
}