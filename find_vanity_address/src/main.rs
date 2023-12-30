use colored::*;
use solana_sdk::signature::{Keypair, Signer};
use std::sync::mpsc;
use std::thread;

fn main() {
    let desired_prefix = "X"; // Type your desired prefix here
    let desired_after_prefix = "1";
    let (tx, rx) = mpsc::channel();
    let num_threads = 4; // Number of threads to use

    println!(
        "Searching for address starting with '{}' and ending with '{}'",
        desired_prefix, desired_after_prefix
    );
    for _ in 0..num_threads {
        let thread_tx = tx.clone();
        thread::spawn(move || loop {
            let keypair = Keypair::new();
            let address = keypair.pubkey().to_string();
            if address.starts_with(&desired_prefix) && address.ends_with(&desired_after_prefix) {
                thread_tx
                    .send((address, keypair.to_bytes(), keypair.to_base58_string()))
                    .unwrap();
                break;
            }
        });
    }

    match rx.recv() {
        Ok((address, private_key, private_key_str)) => {
            println!("{}", "-".repeat(50).cyan());
            println!("{}", "Found address".cyan());
            println!("{}\n", address);
            println!("{}", "-".repeat(50).cyan());
            println!("{}", "Private key (Bytes)".cyan());
            println!("{:?}\n", private_key);
            println!("{}", "-".repeat(50).cyan());
            println!("{}", "Private key (String)".cyan());
            println!("{}\n", private_key_str);
        }
        Err(e) => println!("An error occurred: {}", e),
    }
}
