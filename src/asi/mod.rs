use std::{error::Error, ffi::CStr};

pub mod asi_api;
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub(super) mod asicamera2;

pub fn chars_to_string(chars: &[::std::os::raw::c_char]) -> Result<String, Box<dyn Error>> {
    unsafe { Ok(CStr::from_ptr(chars.as_ptr()).to_str()?.to_string()) }
}

pub fn bytes_to_chars(bytes: &[u8]) -> &[::std::os::raw::c_char] {
    unsafe { std::mem::transmute(bytes) }
}
