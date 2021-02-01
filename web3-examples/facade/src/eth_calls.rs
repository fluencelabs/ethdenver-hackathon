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
use crate::eth_utils::get_nonce;
use crate::eth_utils::{check_response_string, BLOCK_NUMBER_TAGS};
use crate::fce_results::JsonRpcResult;
use crate::jsonrpc_helpers::Request;
use chrono::Utc;
use fluence::fce;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};

#[fce]
pub fn eth_get_balance(url: String, account: String, block_number: String) -> JsonRpcResult {
    let method = String::from("eth_getBalance");
    let id = get_nonce();

    let block_identifier: String;
    let number_test = block_number.parse::<u64>();
    if number_test.is_ok() {
        block_identifier = format!("0x{:x}", number_test.unwrap());
    } else if BLOCK_NUMBER_TAGS.contains(&block_number.as_str()) {
        block_identifier = String::from(block_number);
    } else {
        block_identifier = String::from("latest");
    }

    let params: Vec<String> = vec![account, block_identifier];
    // let json_rpc_args = Request::new(method, params, id).as_sys_string(&url);
    // let curl_args = format!("-X POST --data '{}' {}", json_rpc_args, url);
    let curl_args: String = Request::new(method, params, id).as_sys_string(&url);

    let response: String = unsafe { curl_request(curl_args) };

    check_response_string(response, &id)
}

#[fce]
pub fn eth_get_block_height(url: String) -> JsonRpcResult {
    let method = "eth_blockNumber".to_string();
    let params: Vec<String> = Vec::new();
    let id = get_nonce();

    // let json_rpc_args = Request::new(method, params, id).as_sys_string(&url);
    // let curl_args = format!("-X POST --data '{}' {}", json_rpc_args, url);
    let curl_args: String = Request::new(method, params, id).as_sys_string(&url);

    let response: String = unsafe { curl_request(curl_args) };

    /*
    if response.len() == 0 {
        let err_msg = "{\"jsonrpc\":\"2.0\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
        let err_msg = str::replace(err_msg, "$ID", &id.to_string());
        return JsonRpcResult::from(Result::from(Err(err_msg)));
    }

    match response.contains("error") {
        true => JsonRpcResult::from(Result::from(Err(response))),
        false => JsonRpcResult::from(Result::from(Ok(response))),
    }
    */
    check_response_string(response, &id)
}
