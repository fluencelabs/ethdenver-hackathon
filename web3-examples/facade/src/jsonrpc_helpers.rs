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

pub const JSON_RPC: &'static str = "2.0";

#[derive(Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
    pub id: u64,
}

impl Request {
    pub fn new(method: String, params: Vec<String>, id: u64) -> Self {
        Request {
            jsonrpc: String::from(JSON_RPC),
            method,
            params,
            id,
        }
    }

    pub fn as_sys_string(&self, url: &String) -> String {
        let result = format!("-s -X POST --data '{{\"jsonrpc\":\"{}\", \"method\": \"{}\", \"params\":{:?}, \"id\":{}}}' {}", self.jsonrpc, self.method, self.params, self.id, url);
        result
    }
}
