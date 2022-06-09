use petseeker_server_2::{pet,vet,user,wallet,drone};
use petseeker_server_2::drone::{CamType, DroneBatteryState, DroneSource};

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


   // let _wallet_one: wallet::Wallet = wallet::Wallet::new(String::from("3486xv946v2364v29vx6937x46387h3x49"),
   //                                                      &WalletType::Paper,
   //                                                      &new_user,
   //                                                      1818.10);

    let drone: drone::Drone = drone::Drone::new(String::from("859cv83463h4793246vc9363829476v2c4324982"),
                                                true,
                                                &CamType::Normal,
                                                &DroneSource::Own,
                                                &DroneBatteryState::Full,
                                                88,
                                                String::from("Mavics"),
                                                String::from("DJI"));

   let pet_data: pet::Pet  = kimba.get();
   let vet_data: vet::Vet = veterinary.get();
   let user_data: user::User = new_user.get();
   // let wallet_data: wallet::Wallet = wallet_one.get();
   let drone_data: drone::Drone = drone.get();

   println!("{:#?}",pet_data);
   println!("{:#?}",vet_data);
   println!("{:#?}",user_data);
   // println!("{:#?}",wallet_data);
   println!("{:#?}",drone_data);
}
