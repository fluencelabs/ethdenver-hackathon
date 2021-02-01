# EthDenver Virtual'21  

## Introduction  

Welcome to Ethdenver Virtual 2021 and the Fluence Hackathon where juicy bounties, extraordinary fame, oodles of fun and hordes of spork marmots await!  And yes, we are hiring !!

Below we annotate the enclosed code to give you a quick start to work with the Fluence stack and Ethereum. If you're new to Fluence, give the ol' [documentation](https://fluence-labs.readme.io/docs) a gander before diving in. Please note that the Fluence stack is under heavy development as is the underlying WASM and WASI 


## Fluence  
[Fluence](https://fluence.network/) is an open application platform powered by peer-to-peer computing protocol and a decentralized licensing system. Fluence enables developers to host applications in the decentralized network and collaborate on live applications, reusing components and data. The protocol creates an open marketplace of compute capacity, so availability and pricing are not controlled by a single company and instead are driven by competitive market forces.

Applications are faster to build, easier to integrate, and more secure due to the enhanced composability. Business logic is incorporated into data packets orchestrating the execution of distributed components. Just as code collaboration creates better products, composition via network protocol enables live apps to be forked, expanded, or re-arranged into new and enhanced user experiences.

let's get started.  
## Quickstart
If you haven't had a chance to work through the [greeting example](https://fluence-labs.readme.io/docs/how-to-develop-a-module), this might be a good time. For additional examples, check out the [fce](https://github.com/fluencelabs/fce/tree/master/examples) repo, [fluent pad](https://github.com/fluencelabs/fluent-pad), and the [aqua demo](https://github.com/fluencelabs/aqua-demo).  

Setup your [Rust](https://www.rust-lang.org/tools/install) and [Fluence environment](https://fluence-labs.readme.io/docs/how-to-develop-a-module).

Clone this repo to your machine or instance:

TODO: need final repo and urls.
```bash
git clone 
```

and build the 

```
cd  ....
./build.sh
```
if you get a permission error, `chmod +x build.sh`  

Recall from the [documentation](https://fluence-labs.readme.io/docs/services-development) that a service is comprised of one or more modules, facade, effector and pure module(s). 
Looking over the project structure we have the facade and several other ...  

### Getting Started With Fluence and Web3 Services  

[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) is a relatively new concept and WASM for backend services is even newer, e.g., [wasmer](https://github.com/wasmerio/wasmer), [WASI](https://github.com/CraneStation/wasi), and progressing at a rapid clip. Yet, there are still limitations we need to be aware of. For example, sock support and async capabilities are currently not available but should be soon. Not to worry, we can work with and around those constraints and still build effective solutions.  

For the the time being, our go-to transport comes courtesy of [curl](https://curl.se/docs/) as a service. Please note that curl generally does not provide web socket (ws, wss) capabilities, https is our transport tool of choice. This has a few implications especially with blockchain client access as a service, e.g., a subset of the Ethereum JSON RPC calls in [Infura](https://infura.io/docs/ethereum/wss/introduction), for example, are only accessible via wss, although [Alchemy](https://www.alchemyapi.io/) offers an alternative. Using curl generally has no performance penalties and in most cases actually speeds things, it should be noted that leaving the secure wasm sandbox comes at a cost: a node provider can easily monitor and exploit curl call data, such as api-keys. If that is a major, e.g., production, concern, we recommend you run your own node; if it is more of a testnet concern, we recommend using project-specific api-keys, and rotate them periodically. As soon as WASM enables sockets, curl calls can be replace with wasm-native transport(s).

As mentioned earlier, async is currently not quite there but the Fluence team has implemented a cron-based work-around to allow polling. See below, TODO need document link, for more info.

Another limitation that requires a little extra care concerns error management. Specifically, the Result<_,_> does not work in WASI. If you want to return a Result, you need to implement your own.
See web3-examples/facade/src/fce_results.rs for examples. 

In the web3-examples folder, we illustrate the core concepts of Web3 service development with a few Ethereum JSON-RPC calls. In a nutshell, FCE compliant services are written and compiled with `fce build`. The resulting WASM modules can then be locally inspected and executed with `fce-repl`.

### A Simple Example
Let's have a look at one of the examples, eth_get_balance, from `eth_calls_test.rs`:  

```rust
insert/link to eth_get_balance
```  

This example is based on the Ethereum JSON RPC [eth_getBalance](https://eth.wiki/json-rpc/API#eth_getbalance) and returns the balance of the named account for the destination chain specified. We implement that method by combining our custom code with the curl service.
```rust
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
    let curl_args: String = Request::new(method, params, id).as_sys_string(&url);
    let response: String = unsafe { curl_request(curl_args) };

    check_response_string(response, &id)
}
```
1.  We apply the fce macro to the function, which returns our custom JsonRpcResult (see fce_results.rs)
2.  We specify and the actual method name, which by the way, may deviate from the Ethereum spec's depending on the eth-client provider. See eth_filters.rs for an example.
3.  We generate our nonce, aka id, which is based on the thread-safe nonce counter implemented in eth_utils.rs:

```rust
pub static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);
```

4. We handle out block_number tag to makes sure it's either a valid (positive) number or one of ["latest", "pending", "earliest"]. Note that many of the node-as-a-service providers do not provide historical data without users signing up for archive services. 
5. Now we format our params and args into a json-rpc suitable for curl
6.  We finally check our response and return the result

We can now run that function in the fce-repl:
```bash
2> call facade eth_get_balance  ["https://eth-mainnet.alchemyapi.io/v2/<your key>", "0x0000000000000000000000000000000000000000", "latest"]
timestamp: 1612167486365
curl args: -X POST --data '{"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":2}' https://eth-mainnet.alchemyapi.io/v2/<your key>
INFO: Running "/usr/bin/curl -X POST --data {"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":2} https://eth-mainnet.alchemyapi.io/v2/<your key>" ...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   182  100    62  100   120     83    161 --:--:-- --:--:-- --:--:--   243
result: Object({"error": String(""), "id": Number(2), "jsonrpc": String("2.0"), "result": String("0x1c804d8c47f4e326821")})
 elapsed time: 756.728025ms

3>
```
Note that for the purpose of the examples, we return the raw result(s), which are usually hex strings. A due to the Result type, you need to explicitly check the error string before processing the result:

```rust
    // <snip>
    let result =  JsonRpcResult {error: "".to_string(), 
                                 id: 2u64,
                                 jsonrpc: "2.0".to_string(),
                                 result: "0x1c804d8c47f4e326821".to_string()};
    match result.error.len() {
       0 => println!("do something with ok such as {}", u128::from_str_radix(result[2..], 16)),
        _ => println!("do something with err")
    }
```


#### A Note On Testing  
Due to limitations in WASI for another few months, Rust unit tests proper are not working for fce modules when an external binary, such as curl, is imported. A workaround is to implement test methods in fce and run them in fce-repl. The examples below are based on eth_getBalance discussed above. 

```rust
#[fce]
fn test_eth_get_balance_good(url: String) -> TestResult {
    let burn_address = String::from("0x0000000000000000000000000000000000000000");
    let block_height = String::from("latest");
    // burn account balances, min, per 1/27/21:
    // https://etherscan.io/address/0x0000000000000000000000000000000000000000; 8412.0
    // https://kovan.etherscan.io/address/0x0000000000000000000000000000000000000000; 213.0
    // https://rinkeby.etherscan.io/address/0x0000000000000000000000000000000000000000; 1566.0
    // https://goerli.etherscan.io/address/0x0000000000000000000000000000000000000000; 1195.0

    let result = eth_get_balance(url, burn_address, block_height);
    let hex_balance: String = result.result;
    let wei_balance: u128 = u128::from_str_radix(&hex_balance[2..], 16).unwrap();
    let eth_balance: f64 = wei_to_eth(&wei_balance);
    if eth_balance > 213.0 {
        return TestResult::from(Result::from(Ok(String::from(""))));
    }
    let err_msg = format!("expected: gt {}, actual {:.2}", 213.0, eth_balance);
    TestResult::from(Result::from(Err(err_msg)))
}

#[fce]
fn test_eth_get_balance_bad(url: String) -> TestResult {
    let burn_address = String::from("0x0000000000000000000000000000000000000000");
    let block_height = String::from("latest");
    // burn account balances, min, per 1/27/21:
    // https://etherscan.io/address/0x0000000000000000000000000000000000000000; 8412.0
    // https://kovan.etherscan.io/address/0x0000000000000000000000000000000000000000; 213.0
    // https://rinkeby.etherscan.io/address/0x0000000000000000000000000000000000000000; 1566.0
    // https://goerli.etherscan.io/address/0x0000000000000000000000000000000000000000; 1195.0

    let result = eth_get_balance(url, burn_address, block_height);
    let hex_balance: String = result.result;
    let wei_balance: u128 = u128::from_str_radix(&hex_balance[2..], 16).unwrap();
    let eth_balance: f64 = wei_to_eth(&wei_balance);
    if eth_balance > 1_000_000.0 {
        return TestResult::from(Result::from(Ok(String::from(""))));
    }
    let err_msg = format!("expected: gt {}, actual {:.2}", 1_000_000, eth_balance);
    TestResult::from(Result::from(Err(err_msg)))
}
```

Here we test eth_get_balance with the burn account 0x0000000000000000000000000000000000000000 for the the latest block and return the result as TestResult (see eth_utils.rs). Running the functions in fce-repl:

```bash
2> call facade test_eth_get_balance_bad  ["https://eth-mainnet.alchemyapi.io/v2/<your key>"]
curl args: -X POST --data '{"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":1}' https://eth-mainnet.alchemyapi.io/v2/<your key>
INFO: Running "/usr/bin/curl -X POST --data {"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":1} https://eth-mainnet.alchemyapi.io/v2/<your key>" ...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   182  100    62  100   120    123    238 --:--:-- --:--:-- --:--:--   360
result: Object({"error": String("expected: gt 1000000, actual 8412.06"), "test_passed": Number(0)})
 elapsed time: 516.627078ms

3> call facade test_eth_get_balance_good  ["https://eth-mainnet.alchemyapi.io/v2/<your key>"]
curl args: -X POST --data '{"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":2}' https://eth-mainnet.alchemyapi.io/v2/<your key>
INFO: Running "/usr/bin/curl -X POST --data {"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":2} https://eth-mainnet.alchemyapi.io/v2/<your key>" ...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   182  100    62  100   120    164    319 --:--:-- --:--:-- --:--:--   482
result: Object({"error": String(""), "test_passed": Number(1)})
 elapsed time: 387.537328ms

4>
```  

A small, self-contained service, for example, could generate method id and topics generation. See [Solidity reference](https://docs.soliditylang.org/en/latest/abi-spec.html). A simple method generation may look like so:

```rust
#[fce]
use tiny_keccak::Sha3;

#[fce]
pub fn eth_hash_method_id(input: Vec<u8>) -> Vec<u8> {
    let mut output = [0u8; 32];
    let mut keccak = Keccak::v256();

    keccak.update(&input);
    keccak.finalize(&mut output);
    output.to_vec()
}
```

which the corresponding test:

```rust
#[fce]
pub fn test_eth_hash_method_id() -> String {
    use hex::encode;

    // see https://docs.soliditylang.org/en/latest/abi-spec.html#examples
    let input = b"baz(uint32,bool)".to_vec();
    let expected = String::from("cdcd77c0");
    let res = eth_hash_method(input);
    let res = format!("{}", hex::encode(&res[..4]));

    if res == expected {
        return "test passed".to_string();
    }
    "test failed".to_string()
}
```

and fce-repl execution:

```bash
4> call facade test_eth_hash_method []
result: String("test passed")
 elapsed time: 98.266µs

5>
```

Please note that if this function was part of a module not importing an external service, Rust unit tests can be used.

#### Curl  
--max-time, -m: set the timeout in seconds

```rust
pub fn as_sys_string(&self, url: String) -> String {
        let result = format!("-X POST --data '{{\"jsonrpc\":\"{}\", \"method\": \"{}\", \"params\":{:?}, \"id\":{}}}' {}", self.jsonrpc, self.method, self.params, self.id, url);
        result
    }
```  

could be changed to:  

```rust
pub fn as_sys_string(&self, url: String, max_time:u32) -> String {
        let result = format!("-X POST -m {} --data '{{\"jsonrpc\":\"{}\", \"method\": \"{}\", \"params\":{:?}, \"id\":{}}}' {}", max_time, self.jsonrpc, self.method, self.params, self.id, url);
        result
    }
```  

#### Persistence  
We currently ... which, due to it's single thread model, ...

There are different ways to handle this depending on your needs. For example, let's say you want to persist account balances from `eth_get_balance` into an external database accessible with HTTPS. You could then write a wrapper around `eth_get_balance` like so:

```rust
#[fce]
pub fn acco$$unt_balance_to_db(account: String, provider_url:String, db_url:String ) - SomeResultImpl {
    let result = eth_get_balance(account.clone(), provider_url);
    if result.error.len() >0 {
        return Err(result.error);
    }

    // extract balance from json string, e.g.
    let obj = serde_json::from_value(result);
    let hex_balance = obj["result"];
    let eth_balance:u128 = u128::from_str_radix(&hex_balance[2..], 16).unwrap() / 1_000_000_000 * 1_000_000_000;
    ...
    let mariadb_insert = format!("insert into table {} (account, balance, Utc_timestamp) values ({},{},{})", account, eth_balance, chrono::Utc::now().timestamp_millis());

    let curl_args = "-X POST ... ";
    let response = unsafe ( curl_request(curl_args));
    ...

}   
```  

If you are using the  from a front end application, then the ususal loca or remote process doesn't need to change. 


#### Composition 
Composition is a salient feature of the Fluence stack and entails the creation of [particles](). Please note that currently, services can <b>not</b> create particles. That is, all composition efforts need to be managed through front end applications.








## Contribution Guidelines  
