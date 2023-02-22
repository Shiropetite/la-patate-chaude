use std::{net::TcpStream, io::{Error, Read, Write, self}, process};

use common::{models::{communication::*, challenge::{*, md5::*}}, challenges::md5::*};

pub struct ClientManager {
    name_current_player: String,
    players: Vec<PublicPlayer>,
    stream: TcpStream
}

impl ClientManager {

    // Create a client manager and etablish a connection with the server
    pub fn new(args: Vec<String>) -> Result<ClientManager, Error> {
        let name = &args[1];
        let server_address = &args[2];

        let port = if args.len() > 3 { &args[3] } else { "7878" };
        let address = format!("{}:{}", server_address, port);

        let stream = TcpStream::connect(&address)?;

        Ok(ClientManager { name_current_player: name.clone(), players: Vec::new(), stream })        
    }

    // Listen to the server 
    pub fn listen(&mut self) -> Result<(), Error> {
        self.write_message(SentMessage::Hello)?;
        
        loop {
            let message = self.read_message()?;
            self.process_message(message)?;
        }
    }

    // Proccess the message sent by the server
    fn process_message(&mut self, received_message: ReceivedMessage) -> Result<(), Error> {
        match received_message {
            ReceivedMessage::Welcome(_) => { self.on_welcome_message()? }
            ReceivedMessage::SubscribeResult(subscribe_result) => { 
                self.on_subscribe_result(subscribe_result)? 
            }
            ReceivedMessage::PublicLeaderBoard(public_leader_board) => { 
                self.on_public_leader_board(public_leader_board)?
            }
            ReceivedMessage::Challenge(challenge) => {
                self.on_challenge(challenge)?
            }
            ReceivedMessage::ChallengeResult(challenge_result) => {
                self.on_challenge_result(challenge_result)?
            }
            ReceivedMessage::ChallengeTimeout(challenge_timout) => {
                self.on_challenge_timeout(challenge_timout)?
            }
            ReceivedMessage::RoundSummary(round_summary) => {
                self.on_round_summary(round_summary)?
            }
            ReceivedMessage::EndOfGame(end_of_game) => {
                self.on_end_of_game(end_of_game)?
            }
        }
        Ok(())
    }

    // Process welcome message
    fn on_welcome_message(&mut self) -> Result<(), Error> {
        let message_subscribe = SentMessage::Subscribe(Subscribe { name: self.name_current_player.clone() });
        self.write_message(message_subscribe)?;
        Ok(())
    }

    // Process subcribe result
    fn on_subscribe_result(&mut self, subscribe_result: SubscribeResult) -> Result<(), Error> {
        match subscribe_result {
            SubscribeResult::Ok => {
                println!("Subscribe successful");
                Ok(())
            }
            SubscribeResult::Err(error) => {
                println!("Subscribe failed : {error:?}");
                process::exit(1)
            }
        }
    }

    // Process the public leader board
    fn on_public_leader_board(&mut self, leader_board: Vec<PublicPlayer>) -> Result<(), Error> {
        self.players = leader_board;
        println!("Leader board : {:?}", self.players);
        Ok(())
    }

    // Process the type of challenge return by the server
    fn on_challenge(&mut self, challenge: Challenge) -> Result<(), Error> {
        match challenge {
            Challenge::MD5HashCash(input) => {
            self.on_md5_challenge(input)?
            }
        }
        Ok(())
    }

    // Solve md5 challenge and write anwser to the server
    fn on_md5_challenge(&mut self, input: MD5HashcashInput) -> Result<(), Error> {
        let md5 = Md5::new(input);
        let result = md5.solve();
        let challenge_answer = ChallengeAnswer::MD5HashCash(result);

        let message = SentMessage::ChallengeResult(
            ChallengeResult 
            { 
                answer: challenge_answer, 
                next_target: self.choose_next_player()?
            }
        );

        self.write_message(message)?;
        Ok(())
    }

    // Return next player 
    fn choose_next_player(&mut self) -> Result<String, Error> {
        for name_player in &self.players {
            if name_player.name != self.name_current_player{
                return Ok(name_player.name.clone());
            }
        }
        Ok(String::from(""))
    }

    // Process chalenge result
    fn on_challenge_result(&mut self, challenge_result: ChallengeResult) -> Result<(), Error> {
        println!("Challenge success ! {challenge_result:?}");
        Ok(())
    }

    // Process challenge timeout
    fn on_challenge_timeout(&mut self, challenge_timeout: ChallengeTimeout) -> Result<(), Error> {
        println!("Challenge timeout ! {challenge_timeout:?}");
        process::exit(1)
    }

    // Process the end of a round
    fn on_round_summary(&mut self, round_summary: RoundSummary) -> Result<(), Error> {
        println!("Round finished ! {round_summary:?}");
        Ok(())
    }

    // Process the end of the game
    fn on_end_of_game(&mut self, end_of_game: EndOfGame) -> Result<(), Error> {
        println!("The game is finished ! {end_of_game:?}");
        process::exit(0);
    }

    // Read a message from the server
    fn read_message(&mut self) -> Result<ReceivedMessage, Error> {
        let mut size_buffer  = [0u8; 4];
        self.stream.read_exact(size_buffer .as_mut()).expect("Server message failed to be read");

        let message_size = u32::from_be_bytes(size_buffer ); 

        let mut message_buffer  = vec![0u8; message_size as usize]; 
        self.stream.read_exact(message_buffer .as_mut()).expect("Server message failed to be read"); 

        let result = String::from_utf8_lossy(&message_buffer); 
        let received_message = serde_json::from_str::<ReceivedMessage>(&result).expect("Server message failed to be deserialized");

        println!("Message received : {received_message:?}");
        Ok(received_message)
    }

    // Write message to the server
    fn write_message(&mut self, message_to_send: SentMessage) -> io::Result<()> {
        let serialized = serde_json::to_string(&message_to_send).expect("The message failed to be serialized");
        let serialized_size = serialized.len() as u32;

        self.stream.write(&serialized_size.to_be_bytes()).expect("The message size failed to be sent");
        self.stream.write(&serialized.as_bytes()).expect("The message failed to be sent");
        self.stream.flush().expect("The message failed to be flushed");


        println!("Message sent : {serialized}");
        Ok(())
    }

}