use crate::records::HealthRecord;
use crate::records::StepsRecord;
use base64;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use chrono::{NaiveDateTime, SecondsFormat};
use rand::Rng;
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

fn fetch_records<T: de::DeserializeOwned + HealthRecord>(
    record_type: i32,
    from: i32,
    to: i32,
) -> Vec<T> {
    if (fetch_records_count(record_type, from, to) > 0) {
        let s = unsafe { get_health_connect_records(record_type, from, to) };
        let json_data = unsafe { CStr::from_ptr(s).to_bytes().to_vec() };
        let subject_str = std::str::from_utf8(&json_data).unwrap();
        return serde_json::from_str::<Vec<T>>(subject_str).unwrap();
    } else {
        Vec::<T>::new()
    }
}

// pub extern "C" fn get_hc_steps_records_count(from: i32, to: i32) -> i32 {
//     fetch_records_count(StepsRecord::IDENTIFIER, from, to)
// }

pub extern "C" fn get_health_records<
    T: de::DeserializeOwned + HealthRecord,
>(
    from: i32,
    to: i32,
) -> Vec<T> {
    fetch_records::<T>(T::IDENTIFIER, from, to)
}
pub extern "C" fn generate_sample_steps_records(from: i32, to: i32) -> Vec<StepsRecord> {
    use chrono::NaiveDateTime;
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut records = Vec::new();
    let start_time = NaiveDateTime::from_timestamp_opt(from as i64, 0).unwrap();
    let end_time = NaiveDateTime::from_timestamp_opt(to as i64, 0).unwrap();
    let duration = end_time.signed_duration_since(start_time);
    let days = duration.num_days();

    for day in 0..days {
        let base_timestamp = start_time + chrono::Duration::days(day);
        let num_records = rng.gen_range(3..8);

        for hour in 0..num_records {
            let timestamp = base_timestamp + chrono::Duration::hours(hour * 4);
            let steps = rng.gen_range(100..5000);
            let record = StepsRecord {
                count: steps,
                startTime: NaiveDateTime::from_timestamp_opt(
                    timestamp.timestamp(),
                    timestamp.timestamp_subsec_nanos(),
                )
                .unwrap()
                .and_utc()
                .to_rfc3339_opts(SecondsFormat::Millis, true),
                endTime: NaiveDateTime::from_timestamp_opt(
                    (timestamp + chrono::Duration::hours(4)).timestamp(),
                    (timestamp + chrono::Duration::hours(4)).timestamp_subsec_nanos(),
                )
                .unwrap()
                .and_utc()
                .to_rfc3339_opts(SecondsFormat::Millis, true),
            };
            records.push(record);
        }
    }
    records
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
