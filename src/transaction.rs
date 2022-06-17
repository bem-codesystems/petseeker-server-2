use chrono::{DateTime, Utc};

pub enum FinancialType {
    Payment,
    Donation,
}

pub enum Origin {
    InternalWallet,
    ExternalWallet,
}

#[derive(Debug)]
pub struct Transaction<'a> {
    id: &'a str,
    from: &'a str,
    to: &'a str,
    created_at: DateTime<Utc>,
    financial_type: &'a FinancialType,
    origin: &'a Origin
}

impl<'a> Transaction<'a> {
    pub fn new(id: &'a str,from: &'a str,to: &'a str,financial_type: &'a FinancialType,origin: &'a Origin) -> Self {
        Self {
            id,
            from,
            to,
            financial_type,
            origin,
            created_at: Utc::now(),
        }
    }
}