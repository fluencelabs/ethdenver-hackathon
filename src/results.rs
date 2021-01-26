use fluence::fce;

#[fce]
#[derive(Debug)]
pub struct JsonRpcResult {
    pub jsonrpc: String,
    pub result: String,
    pub error: String,
    pub id: u64,
}
