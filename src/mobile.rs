mod records;

use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_char, c_void};
use serde::{Deserialize, Serialize};
use base64;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::de;

#[derive(Debug, Serialize, Deserialize)]
pub struct StepsRecord {
    pub startTime: String,
    pub endTime: String,
    pub count: u32,
}


#[link(wasm_import_module = "hubro_sdk")]
extern "C" {
    fn get_health_connect_records(record_type: i32, from: i32, to: i32) -> *mut c_char;
    fn get_health_connect_number_of_records(record_type: i32, from: i32, to: i32) -> i32;
    fn println(nf_name: *mut c_char);
}

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

// #[no_mangle]
// pub extern "C" fn addx(left: i32, right: i32) -> i32 {
//     let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
//     let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

// let df: DataFrame = DataFrame::new(vec![s1, s2]).unwrap();
// let dataset = linfa_datasets::diabetes();
//
// let lin_reg = TweedieRegressor::params().power(0.).alpha(0.);
// let model = lin_reg.fit(&dataset).unwrap();
// let s3 = unsafe { push_result(4.5) };
// let s4 = unsafe { str_test("fds") };
//     3
// }

fn fetch_records_count(record_type: i32, from: i32, to: i32) -> i32 {
    let mut s;
    loop {
        s = unsafe { get_health_connect_number_of_records(record_type, from, to) };
        if s != -1 {
            break;
        }
    }
    s
}

fn fetch_records<T: de::DeserializeOwned>(record_type: i32, from: i32, to: i32) -> Vec<T>
{
    if (fetch_records_count(record_type, from, to) > 0) {
        let s = unsafe { get_health_connect_records(0, from, to) };
        let subject2 = unsafe { CStr::from_ptr(s).to_bytes().to_vec() };
        let decoded = BASE64_STANDARD.decode(subject2).unwrap();
        let subject_str = std::str::from_utf8(&decoded).unwrap();
        let mut p: Vec<T> = serde_json::from_str::<Vec<T>>(subject_str).unwrap();
        p
    } else {
        Vec::<T>::new()
    }
}


#[no_mangle]
pub extern "C" fn get_hc_steps_records_count(from: i32, to: i32) -> i32 {
    fetch_records_count(0, from, to)
}

#[no_mangle]
pub extern "C" fn get_hc_steps_records(from: i32, to: i32) -> Vec<StepsRecord> {
    fetch_records::<StepsRecord>(0, from, to)
}