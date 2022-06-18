#[allow(unused_variables,dead_code)]
use chrono::{ DateTime, Utc};
use crate::user::User;
use crate::{Finances, Info};

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
#[derive(Debug,Clone)]
pub struct Transaction<'t> {
    pub id: String,
    pub previous_hash: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub validated: bool,
    pub objective: &'t WalletObjective,
    pub created_at: DateTime<Utc>
}

#[derive(Debug)]
pub struct Wallet<'w> {
    pub(crate) id: String,
    pub wallet_type: &'w WalletType,
    pub addresses: Vec<String>,
    pub belongs_to: &'w User,
    pub balance: f64,
    pub transactions: Vec<&'w Transaction<'w>>,
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

    pub fn insert_transaction(&mut self, transaction: &'w mut Transaction<'w>) -> () {
        self.transactions.push(transaction)
    }
}

impl<'w> Finances for Wallet<'w> {
    fn request_fee(&self) -> f64 {
        1562.12
    }
}

impl Info for Wallet<'_> {
    fn info(&self) -> String {
        format!("Wallet balance: {}",self.balance)
    }
}

pub fn adjust_finances(item: &(impl Finances + Info)) -> f64 {
    item.initial_fee()
}

pub fn check_finances<T: Finances + Info>(item: &T) -> f64 {
    if item.initial_fee() > 0.00 {
        item.initial_fee() + item.request_fee()
    } else {
        item.request_fee() * 2.00 + 2.00
    }
}

