# EthDenver Virtual'21  

## Introduction  

Welcome to Ethdenver Virtual 2021 and the Fluence Hackathon where juicy bounties, extraordinary fame, oodles of fun and herdes of spork marmots await!  And yes, we are hiring !!

Below we annotate the enclosed code to give you a quick start to work with the Fluence stack and Etheereum. If you're new to Fluence, give the ol' [documentation](https://fluence-labs.readme.io/docs) a gander before diving in. Please note that the Fluence stack is under heavy development as is the underlying WASM and WASI 


## Fluence  
[Fluence](https://fluence.network/) is an open application platform powered by peer-to-peer computing protocol and a decentralized licensing system. Fluence enables developers to host applications in the decentralized network and collaborate on live applications, reusing components and data. The protocol creates an open marketplace of compute capacity, so availability and pricing are not controlled by a single company and instead are driven by competitive market forces.

Applications are faster to build, easier to integrate, and more secure due to the enhanced composability. Business logic is incorporated into data packets orchestrating the execution of distributed components. Just as code collaboration creates better products, composition via network protocol enables live apps to be forked, expanded, or re-arranged into new and enhanced user experiences.


## Setting up your environment  
Setup your [Rust](https://www.rust-lang.org/tools/install) and [Fluence enviornment](https://fluence-labs.readme.io/docs/how-to-develop-a-module).

## Quickstart
If you haven't had a chance to work through the [greeting example](https://fluence-labs.readme.io/docs/how-to-develop-a-module), this might be a good time. For additional examples, check out the [fce](https://github.com/fluencelabs/fce/tree/master/examples) repo, [fleunt pad](https://github.com/fluencelabs/fluent-pad), and the [aqua demo](https://github.com/fluencelabs/aqua-demo).  

## Getting Started With Web3 Services
We illustrate the core concepts of service development with a few Ethereum JSON-RPC calls. In a nutshell, FCE compliant services are written and compiled with `fce build`. The resulting wasm modules can then be locally inspected and executed with `fce-repl`. 


#### Project Structure  
Facade,....

The fce builder automatically creates a facade slot if not specified.  
 


Clone this repo to your machine or instance:

```bash
git clone 
```
and build the 

```
cd .../
./build.sh
```
if you get a permission error, `chmod +x build.sh`  

Recall from the [documentation] that a service is comprised of one or more service types: facade, ???, and ???. 
Looking over the project structure we have the facade and several other ...  



#### A Note On Testing  
Due to limitations in WASI for another few months, unit tests are not working for #[fce] marked functions when an external binary is imported.  

fce-repl examples from `eth_calls_test.rs`:

```bash
2> call facade test_eth_get_balance_bad  ["https://eth-mainnet.alchemyapi.io/v2/<your key>"]
timestamp: 1612052469735
curl args: -X POST --data '{"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":1}' https://eth-mainnet.alchemyapi.io/v2/<your key>
INFO: Running "/usr/bin/curl -X POST --data {"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":1} https://eth-mainnet.alchemyapi.io/v2/<your key>" ...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   182  100    62  100   120    123    238 --:--:-- --:--:-- --:--:--   360
result: Object({"error": String("expected: gt 1000000, actual 8412.06"), "test_passed": Number(0)})
 elapsed time: 516.627078ms

3> call facade test_eth_get_balance_good  ["https://eth-mainnet.alchemyapi.io/v2/<your key>"]
timestamp: 1612052485019
curl args: -X POST --data '{"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":2}' https://eth-mainnet.alchemyapi.io/v2/<your key>
INFO: Running "/usr/bin/curl -X POST --data {"jsonrpc":"2.0", "method": "eth_getBalance", "params":["0x0000000000000000000000000000000000000000", "latest"], "id":2} https://eth-mainnet.alchemyapi.io/v2/<your key>" ...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   182  100    62  100   120    164    319 --:--:-- --:--:-- --:--:--   482
result: Object({"error": String(""), "test_passed": Number(1)})
 elapsed time: 387.537328ms

4>
```  

A small, self-contained service, for example, could be concerned wit ABI method id and topics generation. See solidity reference ():

```
#[fce]
use tiny_keccak::Sha3;

pub fn method_hasher(fn_name:String, params: String) -> String {

    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];

    let input = b"baz(uint32,bool)";
    let mut output = [0u8; 32];
    keccak.update(input_a);
    keccak.finalize(&mut output);
    println!("digest: {:x?}", &output[..16]);
    println!("digest: 0x{}", hex::encode(&output[..4]));

    let v = 69u32;
    let mut vb: [u8; 32] = [0; 32];
    let res = v.to_be_bytes();

    println!("{:?}", vb);
    println!("res: {:?}", res);
}

```






#### A Note On WASM  
The Fleunce stack is utilizing [WASI](https://wasi.dev/), the web assembly system interface. Aside from the fact that WASI itself is under active development, this als means that WASM modules not compliant with WASI cannot be directly ported to the Fluence stack. Please keep this in mind. Fluence is curently supporting WASI v 0.1.7... ().

For valid types, please see ;

Error Handling:
Error handling is currently not quite as ergoanic as in std Rust and takes a little effort both in creating handling Results. have a look at  and  for examples.  



#### Ethereum JSON-RPC  
If you re running your own eth client, you should be fine as long as you are

Since curl does not easily provide websocket support, it may be easier to stick with eth client service providers that accept HTTP flavors. 

[Infura]() requires 

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
