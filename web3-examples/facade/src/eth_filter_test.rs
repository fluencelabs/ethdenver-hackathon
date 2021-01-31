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

use crate::eth_calls::eth_get_balance;
use crate::eth_filters::{get_filter_changes, new_pending_tx_filter, uninstall_filter};
use crate::eth_utils::wei_to_eth;
use crate::fce_results::TestResult;
use fluence::fce;

#[fce]
fn test_filters(url: String) -> TestResult {
    let pending_filter_id = new_pending_tx_filter(url.clone());
    let result = get_filter_changes(url.clone(), pending_filter_id.clone());
    let result = uninstall_filter(url.clone(), pending_filter_id);

    if result {
        return TestResult::from(Result::from(Ok(String::from(String::from("")))));
    }
    let err_msg = format!("expected filter uninstall to be true but ot false");
    TestResult::from(Result::from(Err(String::from(err_msg))))
}
