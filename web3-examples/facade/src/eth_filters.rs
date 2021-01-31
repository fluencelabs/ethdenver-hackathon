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
use crate::eth_utils::check_response_string;
use crate::eth_utils::get_nonce;
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
    println!("result: {:?}", result_obj);
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
