use restson::RestPath;
use chrono::NaiveDateTime;

use crate::schema::esi_wallet_transactions;
use crate::schema::esi_wallet_journals;


#[derive(Serialize, Deserialize)]
pub struct EsiWallet(pub f32);

#[derive(Serialize, Deserialize)]
pub struct EsiWalletTransactions(pub Vec<EsiWalletTransaction>);

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(EsiWalletJournal, foreign_key = "journal_ref_id")]
pub struct EsiWalletTransaction {
    pub client_id: i32,
    pub date: NaiveDateTime,
    pub is_buy: bool,
    pub is_personal: bool,    
    pub journal_ref_id: i64,
    pub location_id: i64,
    pub quantity: i32,
    pub transaction_id: i64,
    pub type_id: i32,
    pub unit_price: f32
}

#[derive(Serialize, Deserialize)]
pub struct EsiWalletJournals(pub Vec<EsiWalletJournal>);

#[derive(Serialize, Deserialize, )]
pub struct EsiWalletJournal {
    pub amount: Option<f32>,
    pub balance: Option<f32>,
    pub date: NaiveDateTime,
    pub description: String,
    pub id: i64,
    pub ref_type: String,
    pub tax: Option<f32>
}

restpath!(EsiWallet, "/characters/{character_id}/wallet/", [character_id: i32]);
restpath!(EsiWalletTransactions, "/characters/{character_id}/wallet/transactions/", [character_id: i32]);
restpath!(EsiWalletJournals, "/characters/{character_id}/wallet/journal/", [character_id: i32]);