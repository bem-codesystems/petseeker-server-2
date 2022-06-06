use petseeker_server_2::{pet,vet,user};

fn main() {
    let kimba: pet::Pet = pet::Pet::new(String::from("439bc904724023"),
                                        &pet::PetSize::BigSize,
                                        &pet::PetType::Dog,
                                        &pet::PetHealthState::Healthy);

    let veterinary: vet::Vet = vet::Vet::new(String::from("439bc904724023"),
                                             &vet::VetType::Particular,
                                             &vet::VetTributes::PF,
                                             String::from("John Connor"),
                                             String::from("connor.john@terminator.vet"),
                                             String::from("853873s1937183g7g4897246f3871s93g2639127"),
                                             &vet::VetArea::Surgery,
                                             String::from("New York"),
                                             String::from("New York City"),
                                             String::from("War St. 299"),
                                             String::from("100"));

    let new_user: user::User = user::User::new(String::from("68435347384232t4yur23"),
                                               String::from("Mr Schneebly"),
                                               String::from("schneebly@schoolofrock.net"),
                                               true,
                                               String::from("7569vc890213cv26rxctxx93246v98"));

   let pet_data: pet::Pet  = kimba.get();
   let vet_data: vet::Vet = veterinary.get();
   let user_data: user::User = new_user.get();

   println!("{:#?}",pet_data);
   println!("{:#?}",vet_data);
   println!("{:#?}",user_data);
}
