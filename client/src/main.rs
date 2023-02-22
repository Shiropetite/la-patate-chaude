use std::env;
use std::process;

use client_manager::ClientManager;

pub mod client_manager;

fn main() {
    let args: Vec<String> = env::args().collect();

    let client_manager_option = ClientManager::new(args);
    match client_manager_option {
        Ok(mut client_manager) => {
            match client_manager.listen() {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("Erreur : {:?}", error);
                    process::exit(1);
                }
            }
        },
        Err(error) =>  {
            eprintln!("Erreur de connexion : {:?}", error);
            process::exit(1);
        },
    }
    
}

