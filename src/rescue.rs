use crate::pet::Pet;
use crate::user::User;
use crate::vet::Vet;
use crate::wallet::Wallet;

pub enum RescuePlaceType {
    Open,
    Closed,
}

pub enum RescueRiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct Rescue<'l> {
    id: String,
    pet: &'l Pet<'l>,
    user: &'l User,
    vet: &'l Vet<'l>,
    address: String,
    complement: String,
    number: String,
    place_type: &'l RescuePlaceType,
    rescue_lat: String,
    rescue_lng: String,
    has_success: bool,
    risk: &'l RescueRiskLevel,
    need_hospital: bool,
    rescue_wallet: Vec<&'l Wallet<'l>>
}