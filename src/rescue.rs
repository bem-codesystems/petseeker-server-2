use chrono::{DateTime, Utc};
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
    rescue_wallet: Vec<&'l Wallet<'l>>,
    inform_police: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl<'l> Rescue<'l> {
    pub fn new(id: String,pet: &'l Pet, user: &'l User, vet: &'l Vet, address: String, complement: String, number: String,
               place_type: &'l RescuePlaceType, rescue_lat: String, rescue_lng: String, has_success: bool, risk: &'l RescueRiskLevel,
               need_hospital: bool, rescue_wallet: Vec<&'l Wallet>,inform_police: bool) -> Rescue<'l> {
        Rescue {
            id,
            pet,
            user,
            vet,
            address,
            complement,
            number,
            place_type,
            rescue_lat,
            rescue_lng,
            has_success,
            risk,
            need_hospital,
            rescue_wallet,
            inform_police,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}