use chrono::{DateTime, Utc};

#[derive(Debug)]
enum PetType {
    Dog,
    Cat,
    Bird,
    Other,
}
#[derive(Debug)]
enum PetSize {
    SmallSize,
    MediumSize,
    BigSize,
}
#[derive(Debug)]
enum PetHealthState {
    Unknown,
    Healthy,
    SmallDisease,
    MediumDisease,
    HighDisease,
}
#[derive(Debug)]
struct Pet<'p> {
    id: String,
    pet_type: &'p PetType,
    pet_size: &'p PetSize,
    health_state: &'p PetHealthState,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    picture_list: Vec<String>
}

impl <'p>Pet<'p> {
    fn new(id: String, pet_size: &'p PetSize, pet_type: &'p PetType, health_state: &'p PetHealthState) -> Pet<'p> {
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

    fn get(&self) -> Pet<'p> {
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

fn main() {

    let kimba: Pet = Pet::new(
        String::from("439bc904724023"),
        &PetSize::BigSize,
        &PetType::Dog,
        &PetHealthState::Healthy);

   let pet_data: Pet  = kimba.get();

   println!("{:#?}",pet_data);

}
