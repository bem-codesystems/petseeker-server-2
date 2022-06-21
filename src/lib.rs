pub mod pet;
pub mod vet;
pub mod wallet;
pub mod user;
pub mod drone;
pub mod rescue;
pub mod transaction;
pub mod info;

pub trait Finances {
    fn initial_fee(&self) -> f64 {
      0.00
    }
    fn request_fee(&self) -> f64;
    fn total_fee(&self) -> String {
        format!("The current amount is {}", self.initial_fee() + self.request_fee())
    }
}

pub trait Info {
    fn info(&self) -> String;
    fn join(&self, amount: f64) -> String {
        format!("Joining the chain with the amount of: {}",amount)
    }
}
