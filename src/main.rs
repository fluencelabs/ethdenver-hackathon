use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

use fluence::fce;

mod curl;
use crate::curl::curl_request; // to be replaced

#[allow(dead_code)]
struct BalanceRecord {
    account: String,
    amount: u128,
    datetime: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub version: String,
    pub method: String,
    pub params: Vec<Value>,
    pub id: Value,
}

impl Request {
    pub fn new(method: String, params: Vec<Value>, id: u64) -> Self {
        Request {
            version: "2.0".into(),
            method,
            params,
            id: Value::from(id),
        }
    }
    pub fn as_string(&self) -> String {
        serde_json::to_string(&self).expect("Should not have failed")
    }
}

#[fce]
pub fn wei_to_eth(amount: u128) -> f64 {
    amount as f64 / (1_000_000_000.0 * 1_000_000_000.0)
}

#[fce]
pub fn eth_get_block_height(url: String, id: u64) -> Result<u64, String> {
    let method = "eth_blockNumber".to_string();
    let params: Vec<Value> = Vec::new();
    let json_rpc_args = Request::new(method, params, id).as_string();

    let response = curl_request(url, json_rpc_args).unwrap();
    if !response.status.success() {
        let msg = format!("Curl call failed with status: {}", response.status);
        return Err(msg);
    }

    let raw_str = String::from_utf8(response.stdout).unwrap();
    println!("{}", raw_str);
    if raw_str.contains("error") {
        let res: Value = serde_json::from_str(&raw_str).unwrap();
        return Err(format!("{}", res["error"]));
    }

    let result_obj: Value = serde_json::from_str(&raw_str).unwrap();

    let hex_str: String = serde_json::from_value(result_obj["result"].clone()).unwrap();
    let res = u64::from_str_radix(&hex_str[2..], 16).unwrap();
    Ok(res)
}

#[fce]
pub fn eth_get_balance(
    url: String,
    account: String,
    block_number: Value,
    id: u64,
) -> Result<u128, String> {
    let method = "eth_getBalance".to_string();

    let params: Vec<Value> = vec![Value::from(account), block_number];
    let json_rpc_args = Request::new(method, params, id).as_string();

    let response = curl_request(url, json_rpc_args).unwrap();
    if !response.status.success() {
        let msg = format!("Curl call failed with status: {}", response.status);
        return Err(msg);
    }

    let raw_str = String::from_utf8(response.stdout).unwrap();
    if raw_str.contains("error") {
        let res: Value = serde_json::from_str(&raw_str).unwrap();
        return Err(format!("{}", res["error"]));
    }
    println!("raw: {:?}", raw_str);

    let result_obj: Value = serde_json::from_str(&raw_str).unwrap();
    let hex_amount: String = serde_json::from_value(result_obj["result"].clone()).unwrap();
    let wei_amount = u128::from_str_radix(&hex_amount[2..], 16).unwrap();

    Ok(wei_amount)
}

pub fn main() {}

/*
#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const INFURA_URL: &str = "https://kovan.infura.io/v3/";
    const KOVAN_BLOCK_HEIGHT: u32 = 23117516; // 01/21/2021
    const KOVAN_BURN_BALANCE: f64 = 213.24; // 01/21/2021
    static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

    fn get_nonce() -> u64 {
        NONCE_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
    }

    fn get_url() -> String {
        let infura_api_key: String = env::var("INFURA_SECRET").unwrap();
        format!("{}{}", INFURA_URL, infura_api_key)
    }

    #[test]
    fn curl_test_success() {
        let url = get_url();
        let json_rpc_args = String::from(r#"{"jsonrpc": "2.0", "method": "ping", "id": 1}"#);

        let response = curl_request(url, json_rpc_args).unwrap();
        assert!(response.status.success());
    }
    #[test]
    fn curl_test_with_error() {
        let url = get_url();
        let json_rpc_args = String::from(r#"{"jsonrpc": "2.0", "method": "pinger", "id": 1}"#);
        let response = curl_request(url, json_rpc_args).unwrap();
        let raw_str = String::from_utf8(response.stdout).unwrap();
        assert!(raw_str.contains("error"));
    }

    #[test]
    fn eth_get_balance_good() {
        let infura_url = get_url();

        // burn account
        let account = "0x0000000000000000000000000000000000000000".to_string();
        let block_height = Value::from("latest");
        let id: u64 = get_nonce();

        let wei_balance = eth_get_balance(infura_url, account, block_height, id).unwrap();
        let eth_balance = wei_to_eth(wei_balance);
        assert!(eth_balance > KOVAN_BURN_BALANCE);
    }
    #[test]
    fn eth_get_balance_bad() {
        let infura_url = get_url();

        //bad account -- dropped a 0
        let account = "0x000000000000000000000000000000000000000".to_string();
        let block_height = Value::from("latest");
        let id: u64 = get_nonce();

        let eth_balance = eth_get_balance(infura_url, account, block_height, id);
        assert!(eth_balance.is_err());
    }

    #[test]
    fn eth_get_block() {
        let infura_url = get_url();
        let id: u64 = get_nonce();

        let block_height = eth_get_block_height(infura_url, id).unwrap();
        assert!(block_height > KOVAN_BLOCK_HEIGHT as u64);
    }
}
*/
