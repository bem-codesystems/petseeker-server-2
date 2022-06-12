use chrono::Utc;
use petseeker_server_2::{pet, vet, user, wallet, drone, rescue};
use petseeker_server_2::drone::{CamType, DroneBatteryState, DroneSource};
use petseeker_server_2::rescue::{Notify, Rescue, RescuePlaceType, RescueRiskLevel};
use petseeker_server_2::wallet::{Transaction, WalletObjective, WalletType};
use petseeker_server_2::Finances;

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


    let mut transaction = wallet::Transaction {
        id: String::from("34cb2836936v3946"),
        previous_hash: String::from("3i2b90278027rv36c367c5v7628736908173bn1027xv8373c23z28753c783x5"),
        hash: String::from("289262v83vruyfhdsjkldfkj983v7233"),
        from: String::from("Chuck Norris"),
        to: String::from("John Malone"),
        amount: 135762.12,
        validated: true,
        objective: &wallet::WalletObjective::Payment,
        created_at: Utc::now()
    };

   let mut wallet_one: wallet::Wallet = wallet::Wallet::new(String::from("3486xv946v2364v29vx6937x46387h3x49"),
                                                        &WalletType::Paper,
                                                        &new_user,
                                                        1818.10);

    let drone: drone::Drone = drone::Drone::new(String::from("859cv83463h4793246vc9363829476v2c4324982"),
                                                true,
                                                &CamType::Normal,
                                                &DroneSource::Own,
                                                &DroneBatteryState::Full,
                                                88,
                                                String::from("Mavics"),
                                                String::from("DJI"));

   let rescue: rescue::Rescue = rescue::Rescue::new(String::from("239074v38c65cv6839821x790937b92c7v0cx6219"),&kimba,&new_user,&veterinary,
    String::from("Robles St,1011"),String::from("ap 27"),String::from("100"),&RescuePlaceType::Closed,String::from("-10983.00"),
    String::from("98723.00"),false,&RescueRiskLevel::Medium,true,vec![&wallet_one],true);

   let _make_notification: String = rescue.notify();

   let _refund: () = rescue.ask_refund();

   let _pet_data: pet::Pet  = kimba.get();
   let _vet_data: vet::Vet = veterinary.get();
   let _user_data: user::User = new_user.get();
   let wallet_data: wallet::Wallet = wallet_one.get();
   let _drone_data: drone::Drone = drone.get();
   let _rescue_data: rescue::Rescue = rescue.get();

    let transaction_list: Vec<&Transaction> = vec![&transaction];
    println!("{:#?}",wallet_data);
    println!("{:#?}",transaction_list);

    let fee:String = wallet_one.total_fee();

    println!("{}",fee);

   // println!("{:#?}",pet_data);
   // println!("{:#?}",vet_data);
   // println!("{:#?}",user_data);
   // println!("{:#?}",wallet_data);
   // println!("{:#?}",drone_data);
}
