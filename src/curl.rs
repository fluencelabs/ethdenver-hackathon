// replace with service !!
use std::process::{Command, Output}; // only for testing of console curl

// #[cfg(feature = "console-curl")]
pub fn curl_request(url: String, json_rpc_args: String) -> Result<Output, std::io::Error> {
    let res: Output = Command::new("curl")
        .args(&["-X", "POST", "-H", "Content-Type: application/json", "-d"])
        .arg(&json_rpc_args)
        .arg(&url)
        .output()
        .unwrap();
    Ok(res)
}
