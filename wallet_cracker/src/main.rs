use reqwest::{Client, Result};
use serde_json::Value;
use tokio::task;
use std::sync::Arc;
// use itertools::Itertools;
use std::collections::HashMap;
use ethers::prelude::*;
use ethers::core::rand::thread_rng;
// use ethers::utils::secret_key_to_address;
// use ethers::types::H256;
// use rand::Rng;


fn print_non_zero_balances(result: &HashMap<String, Vec<Value>>) {
    for (a, b) in result{
        println!("Chain: {}, Address: {}, Balance: {}", a, 2, 5);
    }
    // for (chain, value) in &result {
    //   if let (Some(address), Some(balance)) = (
    //       item.get("account").and_then(|a| a.as_str()),
    //       item.get("balance").and_then(|b| b.as_str()),
    //   ) {
    //       let balance = u64::from_str_radix(balance, 10).unwrap_or(0);
    //       if balance > 0 {
    //           println!("Chain: {}, Address: {}, Balance: {}", chain, address, balance);
    //       }
    //      }
    // }
}

async fn get_balances(client: &Client, chain: &str, account: &HashMap<Address, String>, api_key_idx: usize, KEYS: &HashMap<&str, Vec<&str>>){
    let addresses: String = account.keys().map(|a| a.to_string()).collect::<Vec<_>>().join(",");
    let eth_url: String = format!(
        "https://api.{}scan.io/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
        chain, addresses, KEYS[chain][api_key_idx]
    );

    // let response = client.get(&eth_url).send().await;
    // let json = response.json().await;
    // print_non_zero_balances(json);
}

fn au8_to_string(signature_code: Vec<u8>) -> String {
    let mut private_key = String::new();
    for a in signature_code.iter() {
        let fstr = format!("{:02x}",a);//将二进制元素转换为16进制输出
        private_key.push_str(&fstr);
    }
    private_key
}

async fn main_async(){

    let mut keys: HashMap<&str, Vec<&str>> = HashMap::new();
    keys.insert("ethers", vec!["S4QVVTWYIFEFSKM5BYTXCXEWE228MSEFFW", "1UPSYQ9TD7E3WBDU32GDUE4B39QPJIY393"]);
    keys.insert("polygon", vec!["VH4ZZ1KKNQUB6XGBAUE2GIS18AH8Y7JA32", "HWAGARJQ7VRF28XJ9K9XNJ5T62I2IT3C8T"]);
    keys.insert("bsc", vec!["93HG2T4FJXDM28BKRP4R6MZFXMUR91XX7A", "W4ZC1DW94RA5ZQPVD497XBMAQ23738841G"]);
    keys.insert("arbi", vec!["DI6F7QEBHUESA4QWH4IFEZK2QES9SMAN94", "J95UPMPWKDPXU234JZEVAW2K2EDYX4HMWZ"]);

    loop {
        let client1 = Client::new();
        let client2 = Client::new();
        let client3 = Client::new();
        let client4 = Client::new();
        // let client2 = Client::new();

        // Generate 20 random private keys and their corresponding addresses
        let wallets: Vec<LocalWallet> = (0..20)
            .map(|_| LocalWallet::new(&mut thread_rng()))
            .collect();

        let mut account: HashMap<Address, String> = HashMap::new();
        // for wallet in wallets {
        //     let ss = wallet.signer().to_bytes();
        //     let ass = ss.as_slice();
        //     let private_key = au8_to_string(ass.to_vec());
        //     let address = wallet.address();
        //     account.insert(address, private_key);
        // }

        // Split the addresses into two groups for each API key
        // let (account1, account2) = account.split_at(20);

        // let account1_clone = account1.to_vec();
        // let account2_clone = account2.to_vec();
        // let account = Arc::new(account);
        // let keys = Arc::new(keys);
        // Create two tasks to query the balances using different API keys concurrently
        // let task1 = task::spawn(async move { get_balances(&client1, "ether", &account, 0, &keys).await });
        // let task2 = task::spawn(async move { get_balances(&client2, "polygon", &account, 0, &keys).await });
        // let task3 = task::spawn(async move { get_balances(&client3, "bsc", &account, 0, &keys).await });
        // let task4 = task::spawn(async move { get_balances(&client4, "arbi", &account, 0, &keys).await });
        // let task2 = task::spawn(async move { get_balances(&client2, account2_clone, 1).await });
    }
    // Ok(())
}

extern crate redis;
use redis::Commands;

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let _ : () = con.set("my_key", 42)?;
    Ok(())
}

fn main() {
    
    // use std::time::Instant;
    // let now = Instant::now();

    for _ in 0..10000 {
        // 1000 loops cost 1.14s in release mode
        // 10000 loops cost 11.38s in release mode
        let wallets: Vec<LocalWallet> = (0..20) 
            .map(|_| LocalWallet::new(&mut thread_rng()))
            .collect();
    }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    
    
    // do_something();
    // let rt = tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();
    // rt.block_on(main_async());
}