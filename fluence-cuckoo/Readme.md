# Cuckoo For Coca Puffs

In this tutorial we expose a Rust [cuckoofilter](https://crates.io/crates/cuckoofilter) crate as a Fluence service, deploy it to the Fluence test network and use that service in a frontend Rust app.

## Cuckoo Filters
The [Cuckoo filer]() ais a probabilistic data structure just like [bloom filters](https://en.wikipedia.org/wiki/Bloom_filter) but better; better, because we can not just add but also delete keys from the filter. How 'bout that. , most blockchain developers are familiar with bloom filters

## Cuckoo Filters as Fluence Services
Aside from the fact that cuckoo filters (CF) may be part of your distributed workflow and a service implementation comes in more than handy, there is another reason why a CF as a Service is useful: CF implementations tend to not follow one particular standard and consequently are implementation specific making sharing or re-using filter structures challenging.

## Getting Started



