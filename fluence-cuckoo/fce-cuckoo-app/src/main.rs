#[macro_use]
extern crate fstrings;

use test_utils::{make_swarms, ConnectedClient, KAD_TIMEOUT};
use std::thread::sleep;
use maplit::hashmap;
use serde_json::json;

fn main() {
    let swarms = make_swarms(3);
    sleep(KAD_TIMEOUT);

    let mut client = ConnectedClient::connect_to(swarms[0].1.clone()).expect("connect client!");
    let mut query_client = ConnectedClient::connect_to(swarms[1].1.clone()).expect("connect client");

    let service_id  = "f3137ae9-e687-443d-be1a-9f20a3894d4a";
    
    let create_cf = f!(r#"
    (xor
        (seq
            (call relay (service "create_cf") ["0"] result)
            (call {query_client.node} (returnService "run") [result])
        )
        (seq
            (call relay ("op" "identity") [])
            (call {query_client.node} (returnService "run") ["XOR FAILED"])
        )
    )"#);

    let data = hashmap! {
        "host" => json!(client.node.to_string()),
        "relay" => json!(query_client.node.to_string()),
        "client" => json!(query_client.peer_id.to_string()),
        "service" => json!(service_id),
    };

    println!("about to query ...");
    // {"relay": String("12D3KooWCFZkf1iE81L5Ra1abv3PVEPM5fD56Xm6cwpJFjfJRXg9"), "service": String("f3137ae9-e687-443d-be1a-9f20a3894d4a"), "client": String("12D3KooWS6MJDyztzpuMrHtTqxQsr1RCFyWJWXLGtDLkvar8hZsB"), "host": String("12D3KooWCm3XEyGJBCKoEhsVhN8NbSstNAXKM7qC1PBN9DNr3nBQ")}

    // from fldist:
    // client seed: 76qEx9wTgUweViSCdLMc7Z9tma9AkawGTFWZCKZNER7Z
    // client peerId: 12D3KooWCeZV2qMyiaVTKUYBQXp5Moxf9gptDFHTDuCZfivZz9Fn
    // node peerId: 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb
    // Particle id: 95f65463-f211-469a-a9e5-66c4a50e1668. Waiting for results... Press Ctrl+C to stop the script.

    query_client.send_particle(
        create_cf,
        data,
    );

    let response = query_client.receive_args();
    println!("{:?}", response);
}