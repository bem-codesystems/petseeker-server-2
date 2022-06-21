#[allow(unused_imports)]
use chrono::Utc;
use petseeker_server_2::{pet, vet, user, wallet, drone, rescue};
use petseeker_server_2::drone::{CamType, DroneBatteryState, DroneSource};
use petseeker_server_2::rescue::{Notify, RescuePlaceType, RescueRiskLevel};
use petseeker_server_2::wallet::{Transaction, WalletType, adjust_finances, Wallet, check_finances};
use petseeker_server_2::{Finances,Info};
use std::io::{ErrorKind,Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[allow(unused_variables,dead_code)]

const LOCAL_ADDR: &str = "127.0.0.1:5000";

const MSG_SIZE: usize = 32;


fn main() {

    // let name: &str = "Este eh um nome";
    //
    // let mut name_pointer: String = String::from("Esse eh um nome de verdade.");
    //
    // println!("Conteudo stack:{0}",name);
    //
    // println!("Conteudo heap:{}",name_pointer);
    //
    // let byte_array_string: &[u8] = name_pointer.as_bytes();
    //
    // println!("{:#?}",byte_array_string);
    //
    // let byte_array_list = vec![
    //     69,
    //     115,
    //     115,
    //     101,
    //     32,
    //     101,
    //     104,
    //     32,
    //     117,
    //     109,
    //     32,
    //     110,
    //     111,
    //     109,
    //     101,
    //     32,
    //     100,
    //     101,
    //     32,
    //     118,
    //     101,
    //     114,
    //     100,
    //     97,
    //     100,
    //     101,
    //     46,
    // ];
    //
    // let string_from_bytes = String::from_utf8(byte_array_list);
    //
    // println!("{:?}",string_from_bytes);



    let server = TcpListener::bind(LOCAL_ADDR).expect("Unable to connect to bind address.");
    server.set_nonblocking(true).expect("Unable to set unblocking thread.");

    let mut client_list: Vec<TcpStream> = vec![];

    let (sender,receiver) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket,addr)) = server.accept() {
            println!("Client:{}, connected.",addr);
            let s = sender.clone();
            client_list.push(socket.try_clone().expect("Could not clone client."));

            thread::spawn(move || loop {
                let mut buffer: Vec<u8> = vec![0;MSG_SIZE];
                match socket.read_exact(&mut buffer) {
                    Ok(_) => {
                        let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let message = String::from_utf8(message).expect("Could not receive the message.");

                        println!("{}: {:#?}",addr,message);
                        s.send(message).expect("Could not address message received.");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("There was an error.Closing connection to: {}",addr);
                        break;
                    }
                };
                pause_request();
            });
        }
        if let Ok(message) = receiver.try_recv() {
            client_list = client_list.into_iter().filter_map(|mut client| {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MSG_SIZE,0);
                client.write_all(&buffer).map(|_| client).ok()
            }).collect::<Vec<_>>();
        };
        pause_request();
    };


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

   let make_notification: String = rescue.notify();

   let refund: () = rescue.ask_refund();

   let pet_data: pet::Pet  = kimba.get();
   let vet_data: vet::Vet = veterinary.get();
   let user_data: user::User = new_user.get();
   let wallet_data: wallet::Wallet = wallet_one.get();
   let drone_data: drone::Drone = drone.get();
   let rescue_data: rescue::Rescue = rescue.get();

    let transaction_list: Vec<&Transaction> = vec![&transaction];
    println!("{:#?}",wallet_data);
    println!("{:#?}",transaction_list);

    let fee:String = wallet_one.total_fee();
    let initial_fee = check_finances(&wallet_one);

    println!("{:?}",initial_fee);

   println!("{:#?}",pet_data);
   println!("{:#?}",vet_data);
   println!("{:#?}",user_data);
   println!("{:#?}",wallet_data);
   println!("{:#?}",drone_data);



}

// fn pause_request() {
//     thread::sleep(Duration::from_millis(200));
// }
