
use std::ffi::{ c_char, CStr, CString, NulError};

// use crate::{error_code::LexActivatorErrorCode};

// --------------------------- String operations ------------------------

pub fn string_to_cstring(mut key: String) -> Result<CString, NulError> {
    if key.contains('\0') {
        key = key.replace('\0', "");
        println!("key after operation: {}", key);
        println!("Length after operation: {}", key.len());
    }
    let c_license_key = CString::new(key)?;
    Ok(c_license_key)
}
// pub fn string_to_cstringa(key: &str) -> CString {
//     let license_key_result: Result<CString, NulError> = CString::new(key);
//     let c_license_key: CString = match license_key_result {
//         Ok(cstring) => cstring,
//         Err(err) => {
//             // panic!("Failed to convert Rust string to C string: {}", err);
//             return Err(LexActivatorErrorCode::from(err));
//         }
//     };
//     c_license_key
// }

// fn to_utf16_A(key: &str) -> *const u16 {
//     let utf16_sequence: Vec<u16> = key.encode_utf16().chain(std::iter::once(0)).collect();
//     let utf16_ptr = utf16_sequence.as_ptr();
//     utf16_ptr
// }

pub fn to_utf16a(product_id: &str) -> Vec<u16> {
    let utf16: Vec<u16> = product_id
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    utf16
}

pub fn to_utf16(rust_string: String) -> Vec<u16> {
    let utf16: Vec<u16> = rust_string
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
