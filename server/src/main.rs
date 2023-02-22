use std::{env, process};

use server_manager::ServerManager;

pub mod server_manager;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server_manager_result = ServerManager::new(args);
    match server_manager_result {
        Ok(mut server_manager) => {
            let listener = server_manager.listen();
            match listener {
                Ok(()) => {},
                Err(error) => {
                    eprintln!("Erreur : {:?}", error);
                    process::exit(1)
                }
            }
        }
        Err(error) => {
            eprintln!("Erreur : {:?}", error);
            process::exit(1)
        }
    }
}
