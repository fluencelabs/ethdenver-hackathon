use crate::results::JsonRpcResult;
use serde_json::Value;
use std::error::Error;

const JSON_RPC: &'static str = "2.0";

pub(crate) type Result<T> = std::result::Result<T, T>;

pub struct JsonRpcError {
    code: i32,
    message: String,
}

impl From<Result<String>> for JsonRpcResult {
    fn from(result: Result<String>) -> Self {
        let jsonrpc = JSON_RPC.into();
        match result {
            Ok(res) => {
                let result_obj: Value = serde_json::from_str(&res).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();
                let result: String = serde_json::from_value(result_obj["result"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    result,
                    error: "".to_string(),
                }
            }
            Err(err) => {
                let result_obj: Value = serde_json::from_str(&err).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();
                println!("obj: {:?}", result_obj["error"].clone());
                // let error: String = serde_json::from_value(result_obj["error"].clone()).unwrap();
                Self {
                    jsonrpc,
                    id,
                    result: "".to_string(),
                    //TODO need a JsonRpcError struct { code: i32, message: String } but since we can't di
                    error: err,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_rpc_result_good() {
        let json_response =
            r#"{"jsonrpc":"2.0","id":1,"result":"0x00000000000000004d2"}"#.to_string();
        let res = Result::from(Ok(json_response));
        assert!(res.is_ok());
    }

    #[test]
    fn json_rpc_result_bad() {
        let json_response: String = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"The method eth_getBalanceee does not exist/is not available"}}"#.to_string();
        let res = Result::from(Err(json_response));
        assert!(res.is_err());
    }
}
