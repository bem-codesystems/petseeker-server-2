#[allow(unused_variables,dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum PetType {
    Dog,
    Cat,
    Bird,
    Other,
}
#[derive(Debug)]
pub enum PetSize {
    SmallSize,
    MediumSize,
    BigSize,
}
#[derive(Debug)]
pub enum PetHealthState {
    Unknown,
    Healthy,
    SmallDisease,
    MediumDisease,
    HighDisease,
}
#[derive(Debug)]
pub struct Pet<'p> {
    pub id: String,
    pub pet_type: &'p PetType,
    pub pet_size: &'p PetSize,
    pub health_state: &'p PetHealthState,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pub picture_list: Vec<String>
}

impl<'p> Pet<'p> {
    pub fn new(id: String, pet_size: &'p PetSize, pet_type: &'p PetType, health_state: &'p PetHealthState) -> Pet<'p> {
        Pet {
            id,
            pet_size,
            pet_type,
            health_state,
            picture_list: vec![String::from("")],
            created_at: Utc::now(),
            updated_at: Utc::now()
        }
    }

    pub fn get(&self) -> Pet<'p> {
        Pet {
            id: String::from(&self.id),
            pet_size: self.pet_size,
            pet_type: self.pet_type,
            health_state: self.health_state,
            created_at: self.created_at,
            updated_at: self.updated_at,
            picture_list: self.picture_list.to_vec(),
        }
    }
}