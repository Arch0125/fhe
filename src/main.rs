use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;
use bincode; // Ensure you have `bincode` and `serde` in your Cargo.toml
use sha3::{Digest, Keccak256};
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let config = ConfigBuilder::default().build();

    // Client-side
    let (client_key, server_key) = generate_keys(config);

    let clear_a = 2u8;
    let clear_b = 3u8;

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

    

    let a = FheUint8::encrypt(clear_a, &client_key);
    let b = FheUint8::encrypt(clear_b, &client_key);

    let mut encrypted_balances: Vec<FheUint8> = accounts.iter()
        .map(|acc| FheUint8::encrypt(acc.balance as u8, &client_key))
        .collect();


        set_server_key(server_key);

    let additional_balance = 5u8;
    let additional_encrypted_balance = FheUint8::encrypt(additional_balance, &client_key);

    for balance in encrypted_balances.iter_mut() {
        *balance += &additional_encrypted_balance;
    }

    // let mut hasher = Keccak256::new();
    // hasher.update(&serialized_result);
    // let hashed_result = hasher.finalize();
    // let mut file = File::create("result.txt").expect("Unable to create file");
    // file.write_all(&hashed_result).expect("Unable to write hash");

    let decrypted_balances: Vec<u8> = encrypted_balances.iter()
        .map(|balance| balance.decrypt(&client_key))
        .collect();




    for (index, balance) in decrypted_balances.iter().enumerate() {
        println!("Decrypted balance for account {}: {}", index + 1, balance);
    }

    //Client-side
    // let decrypted_result: u8 = result.decrypt(&client_key);

    // let clear_result = clear_a * clear_b;

    // println!("Decrypted result: {}", decrypted_result);
    // println!("Clear result: {}", clear_result);

    // assert_eq!(decrypted_result, clear_result);
}