
use std::ffi::*;

use crate::{error_code::LexActivatorErrorCode};

// --------------------------- String operations ------------------------

// fn string_to_wstring(key: &str) -> *const u16 {
//     let utf16: Vec<u16> = key.encode_utf16().chain(std::iter::once(0)).collect();
//     utf16.as_ptr()
// }

// diff between unwrap() and expect() or the above method match{} lies only in error handling as unwrap simply returns the default
// msg of the error while expect() allows us to specify the error msg. They all panic iof the conversion faisl due to some error.

pub fn string_to_cstring_a(key: &str) -> CString {
    let key_cstring: CString = CString::new(key).unwrap();
    key_cstring
}

pub fn string_to_cstring(key: &str) -> CString {
    let license_key_result: Result<CString, NulError> = CString::new(key);
    let c_license_key: CString = match license_key_result {
        Ok(cstring) => cstring,
        Err(err) => {
            panic!("Failed to convert Rust string to C string: {}", err);
            // return Err(LexActivatorErrorCode::from(err));
        }
    };
    c_license_key
}
pub fn string_to_cstringx(key: &str) -> Result<CString, LexActivatorErrorCode> {
    let license_key_result: Result<CString, NulError> = CString::new(key);
    let c_license_key: CString = match license_key_result {
        Ok(cstring) => cstring,
        Err(err) => {
            return Err(LexActivatorErrorCode::from(err));
        }
    };
    Ok(c_license_key)
}

// fn to_utf16_A(key: &str) -> *const u16 {
//     let utf16_sequence: Vec<u16> = key.encode_utf16().chain(std::iter::once(0)).collect();
//     let utf16_ptr = utf16_sequence.as_ptr();
//     utf16_ptr
// }

pub fn to_utf16(product_id: &str) -> Vec<u16> {
    let utf16: Vec<u16> = product_id
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    utf16
}

pub fn utf16_to_string(buffer: &[u16]) -> String {
    let string = String::from_utf16_lossy(buffer);
    string.trim_end_matches('\0').to_owned()
}
pub fn c_char_to_string(buffer: &[c_char]) -> String {
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    c_str.to_string_lossy().into_owned()
}

pub fn u32_to_bool(value: u32) -> bool {
    value != 0
}
