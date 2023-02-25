use std::{
    io::{self, Error, Read, Write},
    net::{TcpListener, TcpStream},
};

use common::models::communication::{ReceivedMessageServer, SentMessageServer, Welcome};

pub struct ServerManager {
    listener: TcpListener,
    timeout: u32,
}

// pub struct NewPlayer {
//   name: String
// }

impl ServerManager {
    pub fn new(_args: Vec<String>) -> Result<ServerManager, Error> {
        let listener = TcpListener::bind("0.0.0.0:7878").expect("Error connecting to the server");
        println!("Server is running");
        println!("Timeout in set to 2s");

        Ok(ServerManager {
            listener,
            timeout: 2,
        })
    }

    pub fn listen(&mut self) -> Result<(), Error> {
        let listener = &self.listener;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
                    self.read_message(&stream)?;
                    self.write_message(
                        &stream,
                        SentMessageServer::Welcome(Welcome { version: 1 }),
                    )?;
                    self.read_message(&stream)?;
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }

        Ok(())
    }

    // Read a message from the server
    fn read_message(&self, mut stream: &TcpStream) -> Result<ReceivedMessageServer, Error> {
        let mut size_buffer = [0u8; 4];
        stream
            .read_exact(size_buffer.as_mut())
            .expect("Server message failed to be read");

        let message_size = u32::from_be_bytes(size_buffer);

        let mut message_buffer = vec![0u8; message_size as usize];
        stream
            .read_exact(message_buffer.as_mut())
            .expect("Server message failed to be read");

        let result = String::from_utf8_lossy(&message_buffer);
        let received_message = serde_json::from_str::<ReceivedMessageServer>(&result)
            .expect("Server message failed to be deserialized");

        println!("Message received : {received_message:?}");
        Ok(received_message)
    }

    // Write message to the listener
    fn write_message(
        &self,
        mut stream: &TcpStream,
        message_to_send: SentMessageServer,
    ) -> io::Result<()> {
        let serialized =
            serde_json::to_string(&message_to_send).expect("The message failed to be serialized");
        let serialized_size = serialized.len() as u32;

        stream
            .write(&serialized_size.to_be_bytes())
            .expect("The message size failed to be sent");
        stream
            .write(&serialized.as_bytes())
            .expect("The message failed to be sent");
        stream.flush().expect("The message failed to be flushed");

        println!("Message sent : {serialized}");
        Ok(())
    }
}
