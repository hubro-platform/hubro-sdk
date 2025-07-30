use crate::records::StepsRecord;
use crate::records::HealthRecord;
use base64;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::de;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_void};

#[link(wasm_import_module = "hubro_sdk")]
unsafe extern "C" {
    fn get_health_connect_records(record_type: i32, from: i32, to: i32) -> *mut c_char;
    fn get_health_connect_number_of_records(record_type: i32, from: i32, to: i32) -> i32;
    fn print_line(nf_name: *mut c_char);
}

pub extern "C" fn allocate(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

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

fn fetch_records<T: de::DeserializeOwned + HealthRecord>(record_type: i32, from: i32, to: i32) -> Vec<T> {
    if (fetch_records_count(record_type, from, to) > 0) {
        let s = unsafe { get_health_connect_records(record_type, from, to) };
        let subject2 = unsafe { CStr::from_ptr(s).to_bytes().to_vec() };
        let decoded = BASE64_STANDARD.decode(subject2).unwrap();
        let subject_str = std::str::from_utf8(&decoded).unwrap();
        let mut p: Vec<T> = serde_json::from_str::<Vec<T>>(subject_str).unwrap();
        p
    } else {
        Vec::<T>::new()
    }
}

pub extern "C" fn get_hc_steps_records_count(from: i32, to: i32) -> i32 {
    fetch_records_count(StepsRecord::IDENTIFIER, from, to)
}

pub extern "C" fn get_health_records<T: de::DeserializeOwned + HealthRecord>(from: i32, to: i32) -> Vec<T> {
    fetch_records::<T>(T::IDENTIFIER, from, to)
}

pub extern "C" fn debug_print_line(output: &str) {
    let size = output.len();
    let ptr = unsafe { allocate(size + 1) as *mut c_char };
    unsafe {
        std::ptr::copy(output.as_ptr(), ptr as *mut u8, size);
        *ptr.add(size) = 0;
        print_line(ptr);
    }
}