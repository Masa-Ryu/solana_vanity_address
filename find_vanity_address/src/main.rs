use colored::*;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::mpsc;
use std::thread;

fn main() {
    let desired_prefix = "x"; // Type your desired prefix here
    let (tx, rx) = mpsc::channel();
    let num_threads = 4; // Number of threads to use

    for _ in 0..num_threads {
        let thread_tx = tx.clone();
        thread::spawn(move || loop {
            let keypair = Keypair::new();
            let address = keypair.pubkey().to_string();
            if address.starts_with(&desired_prefix) {
                thread_tx.send((address, keypair.to_bytes())).unwrap();
                break;
            } else {
                println!("No match: {}", address);
            }
        });
    }

    match rx.recv() {
        Ok((address, private_key)) => {
            println!("\n{}", "-".repeat(50).green());
            println!("Found address: {}\n", address.green());
            println!("\n{}", "-".repeat(50).green());
            println!("Private key: {:?}\n", private_key);
        }
        Err(e) => println!("An error occurred: {}", e),
    }
}
