# Ether Price Quotation Service

## Overview
This module utilizes curl and the Coinbase API to allow the query of the latest Ether (ETH) price relative to some 120 currencies. The service requires a valid Coinmarketcap API key, which is freely available with [registration](). The query function wraps this API call: https://coinmarketcap.com/api/documentation/v1/#operation/getV2CryptocurrencyQuotesLatest:

```bash
ether_price_getter(api_key: String, currency_symbol: String)
```

which expects the Coinmarketcap API key and the three letter currency symbol, e.g.:

```
call ether_price_getter ether_price_getter  ["32fd4c0f-59cf-4182-8f22-d4083df0a738", "EUR"]
```

Please note that you need to supply a valid  Coinmarketcap API key and that the free account limits the
number of conversions to one. The result from the Fluence service call is the raw json string, e.g.:

```
result: String("{\"status\":{\"timestamp\":\"2021-02-22T20:47:59.960Z\",\"error_code\":0,\"error_message\":null,\"elapsed\":225,\"credit_count\":1,\"notice\":null},\"data\":{\"ETH\":{\"id\":1027,\"name\":\"Ethereum\",\"symbol\":\"ETH\",\"slug\":\"ethereum\",\"num_market_pairs\":5986,\"date_added\":\"2015-08-07T00:00:00.000Z\",\"tags\":[\"mineable\",\"pow\",\"smart-contracts\",\"coinbase-ventures-portfolio\",\"three-arrows-capital-portfolio\",\"polychain-capital-portfolio\"],\"max_supply\":null,\"circulating_supply\":114779907.124,\"total_supply\":114779907.124,\"is_active\":1,\"platform\":null,\"cmc_rank\":2,\"is_fiat\":0,\"last_updated\":\"2021-02-22T20:46:06.000Z\",\"quote\":{\"EUR\":{\"price\":1464.4187545109621,\"volume_24h\":34069036322.326084,\"percent_change_1h\":3.41360427,\"percent_change_24h\":-10.32669439,\"percent_change_7d\":-3.70439114,\"percent_change_30d\":41.39754247,\"market_cap\":168085848633.412,\"last_updated\":\"2021-02-22T20:47:05.000Z\"}}}}}")
```

## Where to Find The Service
The service is discoverable at [Fluence](https://dash.fluence.dev/) and ready for use.

## How to Build The Service
machine setup instructions

In the `eth-price-getter` dir, 
```bash
# if needed
chmod +x .build.sh

./build.sh
```

That compiles both the curl and the price getter code and places the resulting wasm code into the `artifacts` dir. For local testing run `fce-repl Config.toml` to initiate the repl:

```bash
mbp16~/localdev/ethdenver-hackathon/ether-price-getter(eth-price-getter|…) % fce-repl Config.toml
Welcome to the FCE REPL (version 0.1.33)
app service was created with service id = b0cc4d0d-fb3e-49e1-8532-54d77e03cdaa
elapsed time 82.757883ms

1> interface
Loaded modules interface:

curl_adapter:
  fn curl_request(url: String) -> String

ether_price_getter:
  fn ether_price_getter(api_key: String, currency_symbol: String) -> String

2> call ether_price_getter ether_price_getter ["35fd4b0e-56cf-4182-8f22-d4095cf0a738", "EUR"]
INFO: Running "/usr/bin/curl -H X-CMC_PRO_API_KEY: 35fd4b0e-56cf-4182-8f22-d4095cf0a738 -H Accept: application/json -d symbol=ETH&convert=EUR -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest" ...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   868    0   868    0     0   2755      0 --:--:-- --:--:-- --:--:--  2755
result: String("{\"status\":{\"timestamp\":\"2021-02-22T21:06:42.598Z\",\"error_code\":0,\"error_message\":null,\"elapsed\":18,\"credit_count\":1,\"notice\":null},\"data\":{\"ETH\":{\"id\":1027,\"name\":\"Ethereum\",\"symbol\":\"ETH\",\"slug\":\"ethereum\",\"num_market_pairs\":5986,\"date_added\":\"2015-08-07T00:00:00.000Z\",\"tags\":[\"mineable\",\"pow\",\"smart-contracts\",\"coinbase-ventures-portfolio\",\"three-arrows-capital-portfolio\",\"polychain-capital-portfolio\"],\"max_supply\":null,\"circulating_supply\":114779907.124,\"total_supply\":114779907.124,\"is_active\":1,\"platform\":null,\"cmc_rank\":2,\"is_fiat\":0,\"last_updated\":\"2021-02-22T21:05:03.000Z\",\"quote\":{\"EUR\":{\"price\":1459.5551152082585,\"volume_24h\":33903071552.817226,\"percent_change_1h\":2.64386327,\"percent_change_24h\":-10.46990835,\"percent_change_7d\":-4.37412016,\"percent_change_30d\":41.26794407,\"market_cap\":167527600565.963,\"last_updated\":\"2021-02-22T21:06:05.000Z\"}}}}}")
 elapsed time: 323.584665ms

3>
```

Calling `interface` lists both the curl_adapter and ether_price_getter interfaces. Calling the `ether_price_getter` service function with the API key and conversion currency symbol, `"35fd4b0e-56cf-4182-8f22-d4095cf0a738", "EUR"`, yields the raw json string result expected.

## How to Deploy The Service
ToDo: fldist installatin instructions


If you don't have a seed ready to use:

```bash
mbp16~/localdev/ethdenver-hackathon/ether-price-getter(eth-price-getter|…) % fldist create_keypair
{
  id: '12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP',
  privKey: 'CAESYI9nA79xXH9yYeuU4UDPiaa7C84U0OKnihdSkSdbMOIV4RtA5l9dooR41cO8lCz3okYpEboK6maL7yk1ABgCvrLhG0DmX12ihHjVw7yULPeiRikRugrqZovvKTUAGAK+sg==',
  pubKey: 'CAESIOEbQOZfXaKEeNXDvJQs96JGKRG6Cupmi+8pNQAYAr6y',
  seed: 'AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje'
}
```

which yields the keypair and seed and allows us to create a new service:

```bash
fldist new_service --env testnet -n "Ether Price Getter" -s AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje --ms artifacts/curl_adapter.wasm:curl_cfg.json artifacts/ether_price_getter.wasm:ether_price_getter_cfg.json
client seed: AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje
client peerId: 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
node peerId: 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb
uploading blueprint Ether Price Getter to node 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb via client 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
creating service c77f4b0f-b9e2-47ca-8358-7add50bab280

```

## Timeout Error:
```
fldist new_service --env testnet -n "Ether Price Getter" -s AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje --ms artifacts/curl_adapter.wasm:curl_cfg.json artifacts/ether_price_getter.wasm:ether_price_getter_cfg.json
client seed: AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje
client peerId: 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
node peerId: 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb
uploading blueprint Ether Price Getter to node 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb via client 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
creating service c77f4b0f-b9e2-47ca-8358-7add50bab280
Particle expired. Now: 1614029919574, ttl: 60000, ts: 1614029859570
Particle expired. Now: 1614029922669, ttl: 60000, ts: 1614029862668
Particle expired. Now: 1614029925656, ttl: 60000, ts: 1614029865653
Particle expired. Now: 1614029926178, ttl: 60000, ts: 1614029866176
Something went wrong!
Error: callback for _callback/createService timed out after 60000
    at Timeout._onTimeout (/Users/bebo/.nvm/versions/node/v12.16.3/lib/node_modules/@fluencelabs/fldist/node_modules/@fluencelabs/fluence/dist/api.js:169:28)
    at listOnTimeout (internal/timers.js:549:17)
    at processTimers (internal/timers.js:492:7)
```

and:

```
mbp16~/localdev/ethdenver-hackathon/ether-price-getter(eth-price-getter|✚1…) % fldist new_service --env testnet -n "Ether Price Getter" -s AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje --ms artifacts/curl_adapter.wasm:curl_cfg.json artifacts/ether_price_getter.wasm:ether_price_getter_cfg.json  --ttl 90000
client seed: AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje
client peerId: 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
node peerId: 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb
uploading blueprint Ether Price Getter to node 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb via client 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
creating service 95abbf2b-cf4f-4560-b3a5-2717b340ead8
Particle expired. Now: 1614030097045, ttl: 90000, ts: 1614030007040
Particle expired. Now: 1614030100739, ttl: 90000, ts: 1614030010735
Particle expired. Now: 1614030104327, ttl: 90000, ts: 1614030014325
Particle expired. Now: 1614030104850, ttl: 90000, ts: 1614030014846
Something went wrong!
Error: callback for _callback/createService timed out after 90000
    at Timeout._onTimeout (/Users/bebo/.nvm/versions/node/v12.16.3/lib/node_modules/@fluencelabs/fldist/node_modules/@fluencelabs/fluence/dist/api.js:169:28)
    at listOnTimeout (internal/timers.js:549:17)
    at processTimers (internal/timers.js:492:7)
mbp16~/localdev/ethdenver-hackathon/ether-price-getter(eth-price-getter|✚1…) %
```

```
mbp16~/localdev/ethdenver-hackathon/ether-price-getter(eth-price-getter|✚1…) % fldist new_service --env testnet -n "Ether Price Getter" -s AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje --ms artifacts/curl_adapter.wasm:curl_cfg.json artifacts/ether_price_getter.wasm:ether_price_getter_cfg.json  --ttl 180000
client seed: AenQdfKCRrh1sajNFkKKV1iojgVnLWyatdaqwzEMopje
client peerId: 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
node peerId: 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb
uploading blueprint Ether Price Getter to node 12D3KooWBUJifCTgaxAUrcM9JysqCcS4CS8tiYH5hExbdWCAoNwb via client 12D3KooWQy61BZ4P1DeJzhvsQ76uQAQc7N8tDk6NDz5BFnnhwuPP
creating service 5f7497e8-1140-44e4-ad93-87d52e7cb359
Particle expired. Now: 1614030312210, ttl: 180000, ts: 1614030132205
Particle expired. Now: 1614030315338, ttl: 180000, ts: 1614030135336
Particle expired. Now: 1614030318206, ttl: 180000, ts: 1614030138203
Particle expired. Now: 1614030318738, ttl: 180000, ts: 1614030138732
Something went wrong!
Error: callback for _callback/createService timed out after 180000
    at Timeout._onTimeout (/Users/bebo/.nvm/versions/node/v12.16.3/lib/node_modules/@fluencelabs/fldist/node_modules/@fluencelabs/fluence/dist/api.js:169:28)
    at listOnTimeout (internal/timers.js:549:17)
    at processTimers (internal/timers.js:492:7)
mbp16~/localdev/ethdenver-hackathon/ether-price-getter(eth-price-getter|✚1…) %
```