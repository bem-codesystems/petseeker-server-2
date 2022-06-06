use petseeker_server_2::{pet};

fn main() {
    let kimba: pet::Pet = pet::Pet::new(
        String::from("439bc904724023"),
        &pet::PetSize::BigSize,
        &pet::PetType::Dog,
        &pet::PetHealthState::Healthy);

   let pet_data: pet::Pet  = kimba.get();

   println!("{:#?}",pet_data);

}
