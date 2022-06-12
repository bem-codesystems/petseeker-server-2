pub mod pet;
pub mod vet;
pub mod wallet;
pub mod user;
pub mod drone;
pub mod rescue;

pub trait Finances {
    fn initial_fee(&self) -> f64 {
      0.00
    }
    fn request_fee(&self) -> f64;
    fn total_fee(&self) -> String {
        format!("The current amount is {}", self.initial_fee() + self.request_fee())
    }
}
