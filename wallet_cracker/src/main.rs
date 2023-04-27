use reqwest::{Client, Result};
use serde_json::Value;
use tokio::task;
// use itertools::Itertools;
use std::collections::HashMap;
use ethers::prelude::*;
use ethers::core::rand::thread_rng;
// use ethers::utils::secret_key_to_address;
// use ethers::types::H256;
// use rand::Rng;

const ETH_API_KEYS: [&str; 2] = ["S4QVVTWYIFEFSKM5BYTXCXEWE228MSEFFW", "1UPSYQ9TD7E3WBDU32GDUE4B39QPJIY393"];
const POLYGON_API_KEYS: [&str; 2] = ["VH4ZZ1KKNQUB6XGBAUE2GIS18AH8Y7JA32", "HWAGARJQ7VRF28XJ9K9XNJ5T62I2IT3C8T"];
const BNB_API_KEYS: [&str; 2] = ["93HG2T4FJXDM28BKRP4R6MZFXMUR91XX7A", "W4ZC1DW94RA5ZQPVD497XBMAQ23738841G"];
const ARBI_API_KEYS: [&str; 2] = ["DI6F7QEBHUESA4QWH4IFEZK2QES9SMAN94", "J95UPMPWKDPXU234JZEVAW2K2EDYX4HMWZ"];
const CHAINS: [&str; 4] = ["eth", "polygon", "bnb", "arbi"];

fn print_non_zero_balances(result: &HashMap<String, Vec<Value>>) {
    for (chain, array) in result.iter() {
        for item in array {
            if let (Some(address), Some(balance)) = (
                item.get("account").and_then(|a| a.as_str()),
                item.get("balance").and_then(|b| b.as_str()),
            ) {
                let balance = u64::from_str_radix(balance, 10).unwrap_or(0);
                if balance > 0 {
                    println!("Chain: {}, Address: {}, Balance: {}", chain, address, balance);
                }
            }
        }
    }
}

async fn get_balances(client: &Client, account: HashMap<Address, String>, api_key_idx: usize) -> Result<HashMap<String, Vec<Value>>> {
    let addresses: String = account.keys().map(|a| a.to_string()).collect::<Vec<_>>().join(",");
    let eth_url: String = format!(
        "https://api.etherscan.io/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
        addresses, ETH_API_KEYS[api_key_idx]
    );

    let polygon_url: String = format!(
        "https://api.polygonscan.com/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
        addresses, POLYGON_API_KEYS[api_key_idx]
    );

    let bnb_url: String = format!(
        "https://api.bscscan.com/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
        addresses, BNB_API_KEYS[api_key_idx]
    );

    let arbi_url: String = format!(
        "https://api.arbiscan.com/api?module=account&action=balancemulti&address={}&tag=latest&apikey={}",
        addresses, ARBI_API_KEYS[api_key_idx]
    );
    

    let eth_response = client.get(&eth_url).send().await?;
    let eth_json = eth_response.json().await?;

    let polygon_response = client.get(&polygon_url).send().await?;
    let polygon_json = polygon_response.json().await?;

    let bnb_response = client.get(&bnb_url).send().await?;
    let bnb_json = bnb_response.json().await?;

    let arbi_response = client.get(&arbi_url).send().await?;
    let arbi_json = arbi_response.json().await?;

    let mut result = HashMap::new();
    result.insert("eth".to_string(), eth_json);
    result.insert("polygon".to_string(), polygon_json);
    result.insert("bnb".to_string(), bnb_json);
    result.insert("arbi".to_string(), arbi_json);

    Ok(result)
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

    loop {
        let client1 = Client::new();
        // let client2 = Client::new();

        // Generate 20 random private keys and their corresponding addresses
        let wallets: Vec<LocalWallet> = (0..20)
            .map(|_| LocalWallet::new(&mut thread_rng()))
            .collect();

        let mut account: HashMap<Address, String> = HashMap::new();
        for wallet in wallets {
            let ss = wallet.signer().to_bytes();
            let ass = ss.as_slice();
            let private_key = au8_to_string(ass.to_vec());
            let address = wallet.address();
            account.insert(address, private_key);
        }

        // Split the addresses into two groups for each API key
        // let (account1, account2) = account.split_at(20);

        // let account1_clone = account1.to_vec();
        // let account2_clone = account2.to_vec();

        // Create two tasks to query the balances using different API keys concurrently
        let task1 = task::spawn(async move { get_balances(&client1, account, 0).await });
        // let task2 = task::spawn(async move { get_balances(&client2, account2_clone, 1).await });

        // Await the results of both tasks
        let result1 = task1.await.unwrap().unwrap();
        // let result2 = task2.await.unwrap().unwrap();
        
        print_non_zero_balances(&result1);
        // print_non_zero_balances(&result2);
    }
    // Ok(())
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(main_async());
}
