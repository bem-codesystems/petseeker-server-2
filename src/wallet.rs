use chrono::{Date, DateTime, Utc};
use crate::user::User;

pub enum WalletType {
    FileSystem,
    Paper,
    Hardware,
}

pub enum WalletObjective {
    Social,
    Payment,
}

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

pub struct Wallet<'w> {
    id: String,
    wallet_type: &'w WalletType,
    addresses: Vec<String>,
    belongs_to: &'w User,
    balance: f64,
    transactions: Vec<Transaction<'w>>,
    created_at: DateTime<Utc>
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
}

