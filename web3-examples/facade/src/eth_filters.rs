/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::curl_request;
use crate::eth_utils::{check_response_string, get_nonce};
use crate::eth_calls::eth_get_tx_by_hash;
use crate::fce_results::JsonRpcResult;
use crate::jsonrpc_helpers::Request;
use crate::jsonrpc_helpers::JSON_RPC;
use fluence::fce;
use serde_json::Value;

/// see:
/// https://eth.wiki/json-rpc/API#eth_uninstallfilter
/// https://docs.alchemyapi.io/alchemy/documentation/alchemy-api-reference/json-rpc#eth_uninstallfilter
/// https://infura.io/docs/ethereum/json-rpc/eth-uninstallFilter
#[fce]
pub fn uninstall_filter(url: String, filter_id: String) -> bool {
    let method = String::from("eth_uninstallFilter");
    let params: Vec<String> = vec![filter_id];
    let id = get_nonce();

    // let request = Request::new(method, params, id);
    let curl_args = Request::new(method, params, id).as_sys_string(&url);

    let response: String = unsafe { curl_request(curl_args) };

    /*
    if response.len() == 0 || response.contains("error") {
        return false;
    }
    */

    let result_obj: Value = serde_json::from_str(&response).unwrap();
    let result: bool = serde_json::from_value(result_obj["result"].clone()).unwrap();
    result
}

// see
// https://eth.wiki/json-rpc/API#eth_newpendingtransactionfilter
// https://infura.io/docs/ethereum/wss/eth_newPendingTransactionFilter
// https://docs.alchemyapi.io/alchemy/documentation/alchemy-api-reference/json-rpc#eth_getfilterchanges
#[fce]
pub fn new_pending_tx_filter(url: String) -> String {
    let method: String;
    let mut params: Vec<String> = Vec::new();

    // Note: Service provider implementations may provide json-rpc wrappers we need to handle
    if url.contains("infura") {
        // please note that this is a wss call for infura which mostlikely will not work
        method = String::from("eth_subscribe");
        params.push(String::from("newPendingTransactions"));
    } else if url.contains("alchemyapi") {
        method = String::from("eth_newPendingTransactionFilter");
    } else {
        method = String::from("eth_newPendingTransactionFilter");
    }

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response: String = unsafe { curl_request(curl_args) };

    let result_obj: Value = serde_json::from_str(&response).unwrap();
    let result: String = serde_json::from_value(result_obj["result"].clone()).unwrap();
    result
}

// https://eth.wiki/json-rpc/API#eth_getfilterchanges
// https://infura.io/docs/ethereum/json-rpc/eth-getFilterChanges
// https://docs.alchemyapi.io/alchemy/documentation/alchemy-api-reference/json-rpc#eth_getfilterchanges
#[fce]
pub fn get_filter_changes(url: String, filter_id: String) -> String {
    let method = String::from("eth_getFilterChanges");
    let params: Vec<String> = vec![filter_id];
    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);

    let response: String = unsafe { curl_request(curl_args) };
    response
}


#[fce]
pub fn get_filter_changes_list(url: String, filter_id: String) -> Vec<String> {
    let method = String::from("eth_getFilterChanges");
    let params: Vec<String> = vec![filter_id];
    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);

    let response = unsafe { curl_request(curl_args) };
    log::info!("response: {}", response);
    let mut response: Value = serde_json::from_str(&response).unwrap_or_else(|_| {
        log::error!("failed to parse ETH RPC response as json");
        panic!("failed to parse ETH RPC response as json");
    });
    let result = response.get_mut("result").unwrap_or_else(|| {
        log::error!("no 'result' field found in ETH RPC response");
        panic!("no 'result' field found in ETH RPC response");
    });
    if let Value::Array(results) = result.take() {
        let tx_hashes: Vec<_> = results.into_iter().flat_map(|r| {
            let hash = r.as_str()?;
            Some(hash.to_string())
        }).collect();
        log::info!("got {} tx hashes", tx_hashes.len());
        tx_hashes
    } else {
        log::error!("expected result to be an array. it wasn't.");
        panic!("expected result to be an array. it wasn't.")
    }
}

#[derive(serde::Deserialize)]
pub struct TxSerde {
    // blockHash: DATA, 32 Bytes - hash of the block where this transaction was in. null when its pending.
    pub blockHash: Option<String>,
    // blockNumber: QUANTITY - block number where this transaction was in. null when its pending.
    pub blockNumber: Option<String>,
    // from: DATA, 20 Bytes - address of the sender.
    pub from: Option<String>,
    // gas: QUANTITY - gas provided by the sender.
    pub gas: Option<String>,
    // gasPrice: QUANTITY - gas price provided by the sender in Wei.
    pub gasPrice: Option<String>,
    // hash: DATA, 32 Bytes - hash of the transaction.
    pub hash: Option<String>,
    // input: DATA - the data send along with the transaction.
    pub input: Option<String>,
    // nonce: QUANTITY - the number of transactions made by the sender prior to this one.
    pub nonce: Option<String>,
    // to: DATA, 20 Bytes - address of the receiver. null when its a contract creation transaction.
    pub to: Option<String>,
    // transactionIndex: QUANTITY - integer of the transactions index position in the block. null when its pending.
    pub transactionIndex: Option<String>,
    // value: QUANTITY - value transferred in Wei.
    pub value: Option<String>,
}

#[derive(serde::Deserialize)]
struct GetTxResponse {
    result: TxSerde
}

#[fce]
pub struct Tx {
    pub blockHash: String,
    pub blockNumber: String,
    pub from: String,
    pub gas: String,
    pub gasPrice: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    pub transactionIndex: String,
    pub value: String,
}

impl From<TxSerde> for Tx {
    fn from(ser: TxSerde) -> Self {
        Self {
            blockHash: ser.blockHash.unwrap_or_default(),
            blockNumber: ser.blockNumber.unwrap_or_default(),
            from: ser.from.unwrap_or_default(),
            gas: ser.gas.unwrap_or_default(),
            gasPrice: ser.gasPrice.unwrap_or_default(),
            hash: ser.hash.unwrap_or_default(),
            input: ser.input.unwrap_or_default(),
            nonce: ser.nonce.unwrap_or_default(),
            to: ser.to.unwrap_or_default(),
            transactionIndex: ser.transactionIndex.unwrap_or_default(),
            value: ser.value.unwrap_or_default()
        }
    }
}

#[fce]
pub fn get_filter_changes_without_null(url: String, filter_id: String) -> Vec<Tx> {
    let tx_hashes = get_filter_changes_list(url.clone(), filter_id.clone());

    let get_tx = |hash: String| -> Option<Tx> {
        let r = eth_get_tx_by_hash(url.clone(), hash);
        let r = serde_json::from_str::<GetTxResponse>(r.as_str());
        match r {
            Ok(r) => Some(r.result.into()),
            Err(e) => {
                log::error!("Error while deserializing GetTxResponse: {}", e);
                None
            }
        }
    };

    let txes: Vec<_> = tx_hashes.into_iter().flat_map(|hash| get_tx(hash)).collect();

    txes
}
