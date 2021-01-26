use crate::curl::curl_request;
use crate::results::JsonRpcResult;
use fluence::fce;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering}; // to be replaced
use tiny_keccak::{Hasher, Sha3};

mod curl;
mod errors;
mod results;

const BLOCK_NUMBER_TAGS: [&'static str; 3] = ["latest", "earliest", "pending"];
static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[fce]
pub fn get_nonce() -> u64 {
    NONCE_COUNTER.load(Ordering::SeqCst) as u64
}

#[fce]
pub fn incr_nonce() -> u64 {
    NONCE_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
}

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
            version: "2.0".to_string(),
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
pub fn eth_get_block_height(url: String, id: u64) -> JsonRpcResult {
    let method = "eth_blockNumber".to_string();
    let params: Vec<Value> = Vec::new();
    let json_rpc_args = Request::new(method, params, id).as_string();

    let response = match curl_request(url, json_rpc_args) {
        Ok(res) => res,
        Err(err) => {
            let err_msg = "{\"jsonrpc\":\"2.0\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
            let err_msg = str::replace(err_msg, "$ID", &id.to_string());
            return JsonRpcResult::from(Result::from(Err(err_msg)));
        }
    };
    let raw_str = String::from_utf8(response.stdout).unwrap();
    match raw_str.contains("error") {
        true => JsonRpcResult::from(Result::from(Err(raw_str))),
        false => JsonRpcResult::from(Result::from(Ok(raw_str))),
    }
}

#[fce]
pub fn eth_get_balance(
    url: String,
    account: String,
    block_number: String,
    id: u64,
) -> JsonRpcResult {
    let method = "eth_getBalance".to_string();

    let block_identifier: Value;
    let number_test = block_number.parse::<u64>();
    if number_test.is_ok() {
        block_identifier = Value::from(number_test.unwrap());
    } else if BLOCK_NUMBER_TAGS.contains(&block_number.as_str()) {
        block_identifier = Value::from(block_number);
    } else {
        block_identifier = Value::from("latest");
    }

    let params: Vec<Value> = vec![Value::from(account), block_identifier];
    let json_rpc_args = Request::new(method, params, id).as_string();

    let response = match curl_request(url, json_rpc_args) {
        Ok(res) => res,
        Err(err) => {
            let err_msg = "{\"jsonrpc\":\"2.0\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
            let err_msg = str::replace(err_msg, "$ID", &id.to_string());
            return JsonRpcResult::from(Result::from(Err(err_msg)));
        }
    };

    let raw_str = String::from_utf8(response.stdout).unwrap();
    match raw_str.contains("error") {
        true => JsonRpcResult::from(Result::from(Err(raw_str))),
        false => JsonRpcResult::from(Result::from(Ok(raw_str))),
    }
}

// #[fce]
pub fn sha3(msg: Vec<u8>) -> [u8; 32] {
    let mut sha3 = Sha3::v256();
    let mut digest = [0u8; 32];
    sha3.update(&msg);
    sha3.finalize(&mut digest);
    digest
}

pub fn main() {}

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

    fn wei_to_eth(amount: u128) -> f64 {
        amount as f64 / (1_000_000_000.0 * 1_000_000_000.0)
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

    // #[test]
    fn eth_get_balance_good() {
        let infura_url = get_url();

        // burn account
        let account = "0x0000000000000000000000000000000000000000".to_string();
        let block_height = "latest".to_string();
        let id: u64 = get_nonce();
        let res = eth_get_balance(infura_url, account, block_height, id);
        assert!(res.error.len() == 0);
        let eth_balance: u128 = u128::from_str_radix(&res.result[2..], 16).unwrap();
        assert!(wei_to_eth(eth_balance) > KOVAN_BURN_BALANCE);
    }

    #[test]
    fn eth_get_balance_bad() {
        let infura_url = get_url();

        //bad account -- dropped a 0
        let account = "0x000000000000000000000000000000000000000".to_string();
        let block_height = "latest".to_string();
        let id: u64 = get_nonce();
        let res = eth_get_balance(infura_url, account, block_height, id);
        assert!(res.error.len() != 0);
    }

    #[test]
    fn eth_get_block() {
        let infura_url = get_url();
        let id: u64 = get_nonce();
        let res = eth_get_block_height(infura_url, id);
        assert!(res.error.len() == 0);
        let block_height = u64::from_str_radix(&res.result[2..], 16).unwrap();
        assert!(block_height > KOVAN_BLOCK_HEIGHT as u64);
    }

    #[test]
    fn uniswap_test() {
        // https://mainnet.infura.io/v3/0cc023286cae4ab886598ecd14e256fd

        let factory_addr = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5a";
        // let usdc_addr = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc";
        let token_1 = "0xCAFE000000000000000000000000000000000000";
        let token_2 = "0xF00D000000000000000000000000000000000000";
        let abi = r#"[{"inputs":[{"internalType":"address","name":"_feeToSetter","type":"address"}],"payable":false,
                            "stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":true,
                            "internalType":"address","name":"token0","type":"address"},{"indexed":true,"internalType":"address",
                            "name":"token1","type":"address"},{"indexed":false,"internalType":"address","name":"pair","type":"address"},
                            {"indexed":false,"internalType":"uint256","name":"","type":"uint256"}],"name":"PairCreated","type":"event"},
                            {"constant":true,"inputs":[{"internalType":"uint256","name":"","type":"uint256"}],"name":"allPairs","outputs":
                            [{"internalType":"address","name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},
                            {"constant":true,"inputs":[],"name":"allPairsLength","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],
                            "payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"internalType":"address","name":"tokenA","type":"address"},
                            {"internalType":"address","name":"tokenB","type":"address"}],"name":"createPair","outputs":[{"internalType":"address","name":"pair","type":"address"}],
                            "payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],
                            "name":"feeTo","outputs":[{"internalType":"address","name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},
                            {"constant":true,"inputs":[],"name":"feeToSetter","outputs":[{"internalType":"address","name":"","type":"address"}],
                            "payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"internalType":"address","name":"","type":"address"},
                            {"internalType":"address","name":"","type":"address"}],"name":"getPair","outputs":[{"internalType":"address","name":"","type":"address"}],
                            "payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"internalType":"address","name":"_feeTo","type":"address"}],
                            "name":"setFeeTo","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},
                            {"constant":false,"inputs":[{"internalType":"address","name":"_feeToSetter","type":"address"}],"name":"setFeeToSetter",
                            "outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"}]"#;

        /*
                address pair = address(uint(keccak256(abi.encodePacked(
                hex'ff',
                factory,
                keccak256(abi.encodePacked(token0, token1)),
                hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f'
        ))));
                */
    }
}
