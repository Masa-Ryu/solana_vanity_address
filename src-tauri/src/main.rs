#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::mpsc;
use std::thread;

#[derive(Serialize)]
struct Wallet {
    address: String,
    private_key: Vec<u8>,
    private_key_str: String,
}

#[tauri::command]
fn find_address(prefix: String, suffix: String, num_threads: i8) -> Wallet {
    let (tx, rx) = mpsc::channel();
    println!("Searching for address matching any of the specified patterns");
    for _ in 0..num_threads {
        let thread_tx = tx.clone();
        let prefix = prefix.clone();
        let suffix = suffix.clone();
        thread::spawn(move || loop {
            let keypair = Keypair::new();
            let address = keypair.pubkey().to_string();
            if address.starts_with(&prefix) && address.ends_with(&suffix) {
                thread_tx
                    .send((
                        address,
                        keypair.to_bytes().to_vec(),
                        keypair.to_base58_string(),
                    ))
                    .unwrap();
                break;
            }
        });
    }

    match rx.recv() {
        Ok((address, private_key, private_key_str)) => Wallet {
            address,
            private_key,
            private_key_str,
        },
        Err(_) => {
            println!("Error receiving address");
            Wallet {
                address: "".to_string(),
                private_key: vec![0; 64],
                private_key_str: "".to_string(),
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
