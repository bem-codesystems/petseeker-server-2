use chrono::{ DateTime, Utc};
use crate::user::User;

#[derive(Debug)]
pub enum WalletType {
    FileSystem,
    Paper,
    Hardware,
}
#[derive(Debug)]
pub enum WalletObjective {
    Social,
    Payment,
}
#[derive(Debug,)]
pub struct Transaction<'t> {
    id: String,
    previous_hash: String,
    hash: String,
    from: String,
    to: String,
    amount: String,
    validated: bool,
    objective: &'t WalletObjective,
    created_at: DateTime<Utc>
}
#[derive(Debug)]
pub struct Wallet<'w> {
    pub(crate) id: String,
    pub wallet_type: &'w WalletType,
    pub addresses: Vec<String>,
    pub belongs_to: &'w User,
    pub balance: f64,
    pub transactions: Vec<&'w mut Transaction<'w>>,
    pub created_at: DateTime<Utc>
}

impl<'w> Wallet<'w> {
    pub fn new(id: String, wallet_type: &'w WalletType,belongs_to: &'w User, balance: f64) -> Wallet<'w> {
        Wallet {
            id,
            wallet_type,
            addresses: vec![],
            belongs_to,
            balance,
            transactions: vec![],
            created_at: Utc::now(),
        }
    }

    pub fn get(&self) -> Wallet<'w> {
        Wallet {
            id: String::from(&self.id),
            wallet_type: self.wallet_type,
            addresses: self.addresses.to_vec(),
            belongs_to: self.belongs_to,
            balance: self.balance,
            transactions: self.transactions.to_vec(),
            created_at: self.created_at,
        }
    }

    pub fn insert_transaction(&mut self, transaction: &mut Transaction) -> () {
        self.transactions.push(transaction)
    }
}

