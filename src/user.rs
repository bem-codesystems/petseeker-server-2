use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct User {
    id: String,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    has_wallet: bool,
    wallet_id: String,
}

impl User {
    pub fn new(id: String,name: String,email: String, has_wallet: bool, wallet_id: String) -> User {
        User {
            id,
            name,
            email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            has_wallet,
            wallet_id,
        }
    }
    pub fn get(&self) -> User {
        User {
            id: String::from(&self.id),
            name: String::from(&self.name),
            email: String::from(&self.email),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            has_wallet: self.has_wallet,
            wallet_id: String::from(&self.wallet_id),
        }
    }
}

