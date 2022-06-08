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
}

pub struct Wallet<'w> {
    id: String,
    wallet_type: &'w WalletType,
    addresses: Vec<String>,
    belongs_to: &'w User,
    balance: f64,
    transactions: Vec<Transaction<'w>>
}

