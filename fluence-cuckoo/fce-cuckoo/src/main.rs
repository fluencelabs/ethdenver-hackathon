use cuckoofilter::{CuckooFilter, ExportedCuckooFilter};
use fluence::fce;
use serde::Serialize;
use serde_json;
use std::collections::hash_map::DefaultHasher;

type CF = CuckooFilter<DefaultHasher>;

fn main() {}

fn ser_cf(cf: CF) -> String {
    let exp_cf: ExportedCuckooFilter = cf.export();
    let ser_cf: String = serde_json::to_string(&exp_cf).unwrap();
    ser_cf
}

fn de_cf(cf: String) -> Result<CF, String> {
    let ser: std::result::Result<ExportedCuckooFilter, serde_json::Error> =
        serde_json::from_str(&cf.as_str());
    if ser.is_err() {
        return Err(format!("Failed to deserialize cf: {}", cf));
    }

    Ok(cuckoofilter::CuckooFilter::<DefaultHasher>::from(
        ser.unwrap(),
    ))
}

#[fce]
pub fn create_cf(with_capacity: u32) -> String {
    let cf = match with_capacity {
        0 => CuckooFilter::<DefaultHasher>::new(),
        _ => CuckooFilter::<DefaultHasher>::with_capacity(with_capacity as usize),
    };
    ser_cf(cf)
}

#[fce]
// one day, this may be available
// pub fn create_and_add_cf<T: ?Sized + Hash>(data: &T) -> String {
// until then, we use bytesrings although a json string of array of values should also work
// regardless, we lose some type discrimintation as 5u32 != 5u64 where in &[u8] it is.
pub fn create_and_add_cf(data: Vec<Vec<u8>>) -> String {
    let mut cf: CF = CuckooFilter::<DefaultHasher>::new();
    for v in data.iter() {
        cf.add(v);
    }
    ser_cf(cf)
}

#[fce]
pub fn add(data: Vec<Vec<u8>>) -> String {
    let mut cf: CF = CuckooFilter::<DefaultHasher>::new();
    let mut result = Vec::<bool>::new();
    for v in data.iter() {
        cf.add(v).unwrap();
        // TODO check for error
    }
    ser_cf(cf)
}

#[fce]
pub fn delete(cf: String, items: Vec<Vec<u8>>) -> Vec<bool> {
    let mut cf = de_cf(cf).unwrap();
    let mut result = Vec::<bool>::new();
    for item in items.iter() {
        result.push(cf.delete(item));
    }
    result
}

#[fce]
pub fn contains(cf: String, items: Vec<Vec<u8>>) -> Vec<bool> {
    let cf = de_cf(cf).unwrap();
    let mut result = Vec::<bool>::new();
    for item in items.iter() {
        result.push(cf.contains(item));
    }
    result
}

#[fce]
pub fn is_empty(cf: String) -> bool {
    let cf = de_cf(cf).unwrap();
    cf.is_empty()
}

#[fce]
pub fn memory_usage(cf: String) -> u64 {
    let cf = de_cf(cf).unwrap();
    cf.memory_usage() as u64
}

#[fce]
pub fn len(cf: String) -> u64 {
    let cf = de_cf(cf).unwrap();
    cf.len() as u64
}

#[fce]
pub fn service_info() -> String {
    #[derive(Serialize)]
    struct ServiceInfo {
        name: String,
        package: String,
        source: String,
        version: String,
    }

    let info = ServiceInfo {
        name: String::from("Cuckoo Filter"),
        package: String::from("https://crates.io/crates/cuckoofilter"),
        source: String::from("https://github.com/axiomhq/rust-cuckoofilter"),
        version: String::from("0.5.0"),
    };

    serde_json::to_string(&info).unwrap()
}

/*
#[fce]
pub fn smoker() {
    let mut data: Vec<Vec<u8>> = Vec::new();
    data.push(5_u32.to_le_bytes().to_vec());
    data.push("hello".as_bytes().to_vec());
    data.push("fluence".as_bytes().to_vec());
    data.push(r#"{"result": 10.64}"#.as_bytes().to_vec());
}
*/
