use std::env;
use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;
use bincode; // Ensure you have `bincode` and `serde` in your Cargo.toml
use sha3::{Digest, Keccak256};
use std::fs::File;
use std::io::prelude::*;
use hex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <to> <from> <amount>", args[0]);
        std::process::exit(1);
    }

    let to: u64 = args[1].parse().expect("Invalid 'to' argument");
    let from: u64 = args[2].parse().expect("Invalid 'from' argument");
    let amount: u64 = args[3].parse().expect("Invalid 'amount' argument");

    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Clone)]
    struct Account {
        account_addr: u64,
        balance: u64,
    }

    let mut accounts: Vec<Account> = vec![
        Account { account_addr: 1, balance: 100 },
        Account { account_addr: 2, balance: 200 },
        Account { account_addr: 3, balance: 100 },
    ];

    let mut encrypted_balances: Vec<FheUint8> = accounts.iter()
        .map(|acc| FheUint8::encrypt(acc.balance as u8, &client_key))
        .collect();

    set_server_key(server_key);

    let additional_encrypted_balance = FheUint8::encrypt(amount as u8, &client_key);

    encrypted_balances[to as usize] += &additional_encrypted_balance;
    encrypted_balances[from as usize] -= &additional_encrypted_balance;

let mut hasher = Keccak256::new();

for balance in &encrypted_balances {
    let serialized_balance = bincode::serialize(balance).unwrap();
    hasher.update(&serialized_balance);
}

let hash_result = hasher.finalize();
println!("{}", hex::encode(hash_result));
}