use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

mod challenges;
mod models;

use challenges::md5::Md5;
use models::communication::*;
use models::challenge::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_address = &args[1];
    let name = &args[2];

    let mut stream = TcpStream::connect(server_address + ":7878").unwrap();

    let mut endOfGame = false;
   
    

    send_message(&mut stream, SentMessage::Hello);
    receive_message(&mut stream);

    send_message(&mut stream, SentMessage::Subscribe(Subscribe { name: name.to_string() }));
    receive_message(&mut stream);

    receive_message(&mut stream); // public leader board
    receive_message(&mut stream); // minigame

    while(endOfGame) {

    }
}

fn send_message(stream: &mut TcpStream, send_message: SentMessage) {
    let serialized = serde_json::to_string(&send_message).unwrap();
    let serialized_size = serialized.len() as u32;

    stream.write(&serialized_size.to_be_bytes()).unwrap();
    stream.write(&serialized.as_bytes()).unwrap();

    println!("{serialized}")
}

fn receive_message(stream: &mut TcpStream) {
    let mut buf_len = [0u8; 4]; 
    stream.read_exact(buf_len.as_mut()).unwrap(); 

    let len = u32::from_be_bytes(buf_len); 

    let mut buf = vec![0u8; len as usize]; 
    stream.read_exact(buf.as_mut()).unwrap(); 
   
    let result = String::from_utf8_lossy(&buf); 
    let received_message = serde_json::from_str::<ReceivedMessage>(&result).unwrap();

    match received_message {
        ReceivedMessage::Welcome(welcome) => {
            println!("{welcome:?}");
        }
        ReceivedMessage::SubscribeResult(subscribe_result) => {
            println!("{subscribe_result:?}");
        }
        ReceivedMessage::PublicLeaderBoard(public_leader_board) => {
            println!("{public_leader_board:?}");
        }
        ReceivedMessage::Challenge(challenge) => {
            println!("{challenge:?}");
            match challenge {
              Challenge::MD5HashCash(input) => {
                let md5 = Md5::new(input);
                let result = md5.solve();
                send_message(stream, SentMessage::ChallengeResult(ChallengeResult 
                  { 
                    answer: (ChallengeAnswer::MD5HashCash(result)), 
                    next_target: "titi".to_owned() 
                  })); // minigame completed
                receive_message(stream);
              }
            }
        }
        ReceivedMessage::ChallengeTimeout(challenge_timeout) => {
            println!("{challenge_timeout:?}");
        }
        ReceivedMessage::RoundSummary(round_summary) => {
            println!("{round_summary:?}");
        }
        ReceivedMessage::EndOfGame(end_of_game) => {
            println!("{end_of_game:?}");
        }
    }
}