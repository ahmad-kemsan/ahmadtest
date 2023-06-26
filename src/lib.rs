// importing all ?
use std::ffi::*;
use serde::{Deserialize};

pub use extern_functions::*;
pub use error_code::*;
pub use string_utils::*;

pub mod extern_functions;
pub mod error_code;
pub mod string_utils;


// use crate::extern_functions;
// use crate::error_code::*;


#[derive(Debug)] 
pub struct LicenseMeterAttribute {
    pub name: String,
    pub allowed_uses: u32,
    pub total_uses: u32,
    pub gross_uses: u32
}

#[derive(Debug)] 
pub struct ProductVersionFeatureFlag {
    pub name: String,
    pub enabled: bool,
    pub data: String
}

#[derive(Debug)] 
pub struct ActivationMode {
    pub initial_mode: String,
    pub current_mode: String
}
#[derive(Debug, Deserialize)] 
pub struct OrganizationAddress {
    #[serde(rename = "addressLine1")]
    pub address_line_1: String,
    #[serde(rename = "addressLine2")]
    pub address_line_2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    #[serde(rename = "postalCode")]
    pub postal_code: String
}
#[repr(u32)]
pub enum PermissionFlags {
    LA_USER = 1,
    LA_SYSTEM = 2,
    LA_IN_MEMORY = 4,
}

// // --------------------------- String operations ------------------------

// // fn string_to_wstring(key: &str) -> *const u16 {
// //     let utf16: Vec<u16> = key.encode_utf16().chain(std::iter::once(0)).collect();
// //     utf16.as_ptr()
// // }

// // diff between unwrap() and expect() or the above method match{} lies only in error handling as unwrap simply returns the default
// // msg of the error while expect() allows us to specify the error msg. They all panic iof the conversion faisl due to some error.

// pub fn string_to_cstring_a(key: &str) -> CString {
//     let key_cstring: CString = CString::new(key).unwrap();
//     key_cstring
// }

// pub fn string_to_cstring(key: &str) -> CString {
//     let license_key_result: Result<CString, NulError> = CString::new(key);
//     let c_license_key: CString = match license_key_result {
//         Ok(cstring) => cstring,
//         Err(err) => {
//             panic!("Failed to convert Rust string to C string: {}", err);
//         }
//     };
//     c_license_key
// }

// // fn to_utf16_A(key: &str) -> *const u16 {
// //     let utf16_sequence: Vec<u16> = key.encode_utf16().chain(std::iter::once(0)).collect();
// //     let utf16_ptr = utf16_sequence.as_ptr();
// //     utf16_ptr
// // }

// pub fn to_utf16(product_id: &str) -> Vec<u16> {
//     let utf16: Vec<u16> = product_id
//         .encode_utf16()
//         .chain(std::iter::once(0))
//         .collect::<Vec<_>>();
//     utf16
// }

// pub fn utf16_to_string(buffer: &[u16]) -> String {
//     let string = String::from_utf16_lossy(buffer);
//     string.trim_end_matches('\0').to_owned()
// }
// pub fn c_char_to_string(buffer: &[c_char]) -> String {
//     let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
//     c_str.to_string_lossy().into_owned()
// }

// pub fn u32_to_bool(value: u32) -> bool {
//     value != 0
// }


type CallbackType = extern "C" fn(u32);

extern "C" {
    fn SetLicenseCallback(callback: CallbackType) -> i32;
}

// --------------- Setter functions ------------------------

pub fn set_product_file(file_path: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { extern_functions::SetProductFile(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path = string_to_cstring(file_path);
        status = unsafe { SetProductFile(c_file_path.as_ptr()) };
    }
    // print!("SetLicenseKey status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_product_data(product_data: &str) -> Result<(), LexActivatorErrorCode> {
    
    let status: i32;
    #[cfg(windows)]
    {
        let c_product_data = to_utf16(product_data);
        status = unsafe { extern_functions::SetProductData(c_product_data.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_product_data = string_to_cstring(product_data);
        status = unsafe { extern_functions::SetProductData(c_product_data.as_ptr()) };
    }
    // print!("setProductData status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

/// Sets the product id of your application. This function must be called on every start of your program before
/// any other functions are called, with the exception of SetProductFile() or SetProductData() function.
/// 
/// # Arguments
///
/// * `product_id` - A `string` value representing the unique product id of your application as mentioned
///                  on the product page in the dashboard.
///
/// * `permission_flags` - depending upon whether your application requires admin/root
///                        permissions to run or not, this parameter can have one of the following
///                        values: LA_SYSTEM, LA_USER, LA_IN_MEMORY
/// # Returns
///
/// Returns `Result<(), LexActivatorErrorCode>`. If the product_id is set successfully,
/// `Ok(())` is returned. If an error occurs, an `Err` variant
/// containing a `LexActivatorErrorCode` is returned.
///
/// # Panics
///
/// This function will panic if the input string contains the null character.
///
 
pub fn set_product_id(product_id: &str, permission_flags: PermissionFlags) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    let c_flags: c_uint = permission_flags as u32 as c_uint;
    #[cfg(windows)]
    {
        let c_product_id = to_utf16(product_id);
        status = unsafe { SetProductId(c_product_id.as_ptr(), c_flags) };
    }
    #[cfg(not(windows))]
    {
        
        let c_product_id = string_to_cstring(product_id);
        status = unsafe { SetProductId(c_product_id.as_ptr(), c_flags) };
    }
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    } 
}

/// In case you want to change the default directory used by LexActivator to
/// store the activation data on Linux and macOS, this function can be used to
/// set a different directory.
/// 
/// # Arguments
/// 
/// * `data_dir` - A `string` value representing the absolute path of the directory
///               where LexActivator should store the activation data.
/// 
/// # Returns
/// 
/// Returns `Result<(), LexActivatorErrorCode>`. If the data directory is set successfully,
/// `Ok(())` is returned. If an error occurs, an `Err` variant
/// containing a `LexActivatorErrorCode` is returned.
/// 
/// # Panics
/// 
/// This function will panic if the input string contains the null character.
/// 

pub fn set_data_directory(data_dir: &str) -> Result<(), LexActivatorErrorCode> {

    let status: i32;
    #[cfg(windows)]
    {
        let c_data_dir = to_utf16(data_dir);
        status = unsafe { extern_functions::SetDataDirectory(c_data_dir.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_data_dir = string_to_cstring(data_dir);
        status = unsafe { extern_functions::SetDataDirectory(c_data_dir.as_ptr()) };
    }
    // print!("SetDataDirectory status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

/// In case you don't want to use the LexActivator's advanced
/// device fingerprinting algorithm, this function can be used to set a custom device fingerprint.
/// 
/// # Arguments
/// 
/// * `device_fingerprint` - A `string` value representing the custom device fingerprint of the user's device.
/// 
/// # Returns
/// 
/// Returns `Result<(), LexActivatorErrorCode>`. If the custom device fingerprint is set successfully,
/// `Ok(())` is returned. If an error occurs, an `Err` variant
/// containing a `LexActivatorErrorCode` is returned.
/// 
/// # Panics
/// 
/// This function will panic if the input string contains the null character.
///

pub fn set_custom_device_fingerprint(device_fingerprint: &str) -> Result<(), LexActivatorErrorCode> {

    let status: i32;
    #[cfg(windows)]
    {
        // let c_product_id = get_utf16_ptr(product_id);
        let c_device_fingerprint = to_utf16(device_fingerprint);
        status = unsafe { extern_functions::SetCustomDeviceFingerprint(c_device_fingerprint.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_device_fingerprint = string_to_cstring(device_fingerprint);
        status = unsafe { extern_functions::SetCustomDeviceFingerprint(c_device_fingerprint.as_ptr()) };
    }
    print!("SetDataDirectory status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}
 
pub fn set_license_key(license_key: &str) -> Result<(), LexActivatorErrorCode> {

    let status: i32;
    #[cfg(windows)]
    {
        let c_license_key = to_utf16(license_key);
        status = unsafe { extern_functions::SetLicenseKey(c_license_key.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_license_key = string_to_cstring(license_key);
        status = unsafe { extern_functions::SetLicenseKey(c_license_key.as_ptr()) };
    }
    print!("SetDataDirectory status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_license_user_credential(email: &str, password: &str) -> Result<(), LexActivatorErrorCode> {
    
    let status: i32;
    #[cfg(windows)]
    {
        let c_email = to_utf16(email);
        let c_password = to_utf16(password);
        status = unsafe { extern_functions::SetLicenseUserCredential(c_email.as_ptr(), c_password.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_email = string_to_cstring(email);
        let c_password = string_to_cstring(password);
        status = unsafe { extern_functions::SetLicenseUserCredential(c_email.as_ptr(), c_password.as_ptr()) };
    }
    print!("SetLicenseUserCredential status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_license_callback(callback: CallbackType) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    status = unsafe { extern_functions::SetLicenseCallback(callback) };
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_activation_lease_duration(lease_duration: u32) -> Result<(), LexActivatorErrorCode> {
    let c_lease_duration: c_uint = lease_duration as c_uint;
    let status = unsafe { extern_functions::SetActivationLeaseDuration(c_lease_duration) };
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_activation_metadata(key: &str, value: &str) -> Result<(), LexActivatorErrorCode>  {
    let status: i32;
    #[cfg(windows)]
    {
        let c_key = to_utf16(key);
        let c_value = to_utf16(value);
        status = unsafe { extern_functions::SetActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_key = string_to_cstring(key);
        let c_value = string_to_cstring(value);
        status = unsafe { extern_functions::SetActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    print!("SetActivationMetadata status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_trial_activation_metadata(key: &str, value: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_key = to_utf16(key);
        let c_value = to_utf16(value);
        status = unsafe { extern_functions::SetTrialActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_key = string_to_cstring(key);
        let c_value = string_to_cstring(value);
        status = unsafe { extern_functions::SetTrialActivationMetadata(c_key.as_ptr(), c_value.as_ptr()) };
    }
    print!("SetTrialActivationMetadata status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_release_version(version: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_version = to_utf16(version);
        status = unsafe { extern_functions::SetReleaseVersion(c_version.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_version = string_to_cstring(version);
        status = unsafe { extern_functions::SetReleaseVersion(c_version.as_ptr()) };
    }
    print!("SetReleaseVersion status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_release_published_date(release_published_date: u32) -> Result<(), LexActivatorErrorCode>{
    let c_release_published_date: c_uint = release_published_date as c_uint;
    let status = unsafe { extern_functions::SetReleasePublishedDate(c_release_published_date) };
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_release_platform(platform: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_platform = to_utf16(platform);
        status = unsafe { extern_functions::SetReleasePlatform(c_platform.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_platform = string_to_cstring(platform);
        status = unsafe { extern_functions::SetReleasePlatform(c_platform.as_ptr()) };
    }
    print!("SetReleasePlatform status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_release_channel(channel: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_channel = to_utf16(channel);
        status = unsafe { extern_functions::SetReleaseChannel(c_channel.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_channel = string_to_cstring(channel);
        status = unsafe { extern_functions::SetReleaseChannel(c_channel.as_ptr()) };
    }
    print!("SetReleaseChannel status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_offline_activation_request_meter_attribute_uses(name: &str, uses: i32) -> Result<(), LexActivatorErrorCode>{
    let status: i32;
    let c_uses: c_uint = uses as c_uint;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { extern_functions::SetOfflineActivationRequestMeterAttributeUses(c_name.as_ptr(), c_uses) };
    }
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(name);
        status = unsafe { extern_functions::SetOfflineActivationRequestMeterAttributeUses(c_name.as_ptr(), c_uses) };
    }
    print!("SetOfflineActivationRequestMeterAttributeUses status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_network_proxy(proxy: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_proxy = to_utf16(proxy);
        status = unsafe { extern_functions::SetNetworkProxy(c_proxy.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_proxy = string_to_cstring(proxy);
        status = unsafe { extern_functions::SetNetworkProxy(c_proxy.as_ptr()) };
    }
    print!("SetNetworkProxy status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn set_cryptlex_host(host: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_host = to_utf16(host);
        status = unsafe { extern_functions::SetCryptlexHost(c_host.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_host = string_to_cstring(host);
        status = unsafe { extern_functions::SetCryptlexHost(c_host.as_ptr()) };
    }
    print!("SetCryptlexHost status: {}", status);
    if status == 0 {
        Ok(())
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

// ------------------- Getter Functions --------------------

pub fn get_product_metadata(key: &str) -> Result<String, LexActivatorErrorCode> {
    
    let status: i32;
    const LENGTH: usize = 256;
    let mut product_metadata_value: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let utf16_ptr =  to_utf16(key);
        status = unsafe { extern_functions::GetProductMetadata(utf16_ptr.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_metadata_value = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let key_cstring: CString  = string_to_cstring(key);
        status = unsafe { extern_functions::GetProductMetadata(key_cstring.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_metadata_value = c_char_to_string(&buffer);
    }
    if status == 0 {
        Ok(product_metadata_value)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_product_version_name() -> Result<Option<String>, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256;
    let product_version_name: Option<String>;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetProductVersionName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        // product_version_name = utf16_to_string(&buffer);
        product_version_name = if status == 0 {
            Some(utf16_to_string(&buffer)) // Wrap the value in Some
        } else {
            None // No value, set to None
        };
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetProductVersionName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        // product_version_name = c_char_to_string(&buffer);
        product_version_name = if status == 0 {
            Some(c_char_to_string(&buffer)) // Wrap the value in Some
        } else {
            None // No value, set to None
        };
    }
    if status == 0 {
        Ok(product_version_name)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_product_version_display_name() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut product_version_display_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetProductVersionDisplayName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_version_display_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetProductVersionDisplayName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        product_version_display_name = c_char_to_string(&buffer);
    }
    println!("Product Version Display Name: {}", product_version_display_name);
    println!("Result: {}", status);
    if status == 0 {
        Ok(product_version_display_name)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_product_version_feature_flag(name: &str) -> Result<ProductVersionFeatureFlag, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut data: String;
    let mut c_enabled: c_uint = 0;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetProductVersionFeatureFlag(c_name.as_ptr(), &mut c_enabled, buffer.as_mut_ptr(), LENGTH as c_uint) };
        data = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(name);
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetProductVersionFeatureFlag(c_name.as_ptr(), &mut c_enabled, buffer.as_mut_ptr(), LENGTH as c_uint) };
        data = c_char_to_string(&buffer);
    }
    let product_version_feature_flag = ProductVersionFeatureFlag {
        name: name.to_string(),
        enabled: u32_to_bool(c_enabled),
        data: data
    };
    println!("Result: {}", status);
    if status == 0 {
        Ok(product_version_feature_flag)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }

    
}

pub fn get_license_metadata(key: &str) -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut license_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key =  to_utf16(key);
        status = unsafe { extern_functions::GetLicenseMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key);
        status = unsafe { extern_functions::GetLicenseMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_metadata = c_char_to_string(&buffer);
    }
    println!("License Metadata: {}", license_metadata);
    println!("Result: {}", status);
    if status == 0 {
        Ok(license_metadata)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_meterattribute(name: &str) -> Result<LicenseMeterAttribute, LexActivatorErrorCode> {
    let status: i32;
    let mut c_allowed_uses: c_uint = 0;
    let mut c_total_uses: c_uint = 0;
    let mut c_gross_uses: c_uint = 0;
    #[cfg(windows)]
    {
        let c_name =  to_utf16(name);
        status = unsafe { extern_functions::GetLicenseMeterAttribute(c_name.as_ptr(), &mut c_allowed_uses, &mut c_total_uses, &mut c_gross_uses) };
    }
    #[cfg(not(windows))]
    {
        let c_name = string_to_cstring(name);
        status = unsafe { extern_functions::GetLicenseMeterAttribute(c_name.as_ptr(), &mut c_allowed_uses, &mut c_total_uses, &mut c_gross_uses) };
    }
    let meter_attribute = LicenseMeterAttribute {
        name: name.to_string(),
        allowed_uses: c_allowed_uses,
        total_uses: c_total_uses,
        gross_uses: c_gross_uses,
    };
    println!("Result: {}", status);
    if status == 0 {
        Ok(meter_attribute)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_key() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut license_key: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseKey(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_key = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseKey(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_key = c_char_to_string(&buffer);
    }
    // println!("License Key: {}", license_key);
    // println!("Result: {}", status);
    if status == 0 {
        Ok(license_key)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_allowed_activations() -> Result<u32, LexActivatorErrorCode> {
    let mut allowed_activations: c_uint = 0;
    let status = unsafe { extern_functions::GetLicenseAllowedActivations(&mut allowed_activations) };
    println!("Allowed Activations: {}", allowed_activations);
    println!("Result: {}", status);
    if status == 0 {
        Ok(allowed_activations)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_total_activations() -> Result<u32, LexActivatorErrorCode> {
    let mut total_activations: c_uint = 0;
    let status = unsafe { extern_functions::GetLicenseTotalActivations(&mut total_activations) };
    println!("Total Activations: {}", total_activations);
    println!("Result: {}", status);
    if status == 0 {
        Ok(total_activations)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_expiry_date() -> Result<u32, LexActivatorErrorCode> {
    let mut expiry_date: c_uint = 0;
    let status = unsafe { extern_functions::GetLicenseExpiryDate(&mut expiry_date) };
    println!("Expiry Date: {}", expiry_date);
    println!("Result: {}", status);
    if status == 0 {
        Ok(expiry_date)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_maintenance_expiry_date() -> Result<u32, LexActivatorErrorCode> {
    let mut expiry_date: c_uint = 0;
    let status = unsafe { extern_functions::GetLicenseMaintenanceExpiryDate(&mut expiry_date) };
    println!("Maintenance Expiry Date: {}", expiry_date);
    println!("Result: {}", status);
    if status == 0 {
        Ok(expiry_date)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_max_allowed_release_version() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut max_allowed_release_version: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseMaxAllowedReleaseVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        max_allowed_release_version = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseMaxAllowedReleaseVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        max_allowed_release_version = c_char_to_string(&buffer);
    }
    println!("Max Allowed Release Version: {}", max_allowed_release_version);
    println!("Result: {}", status);
    if status == 0 {
        Ok(max_allowed_release_version)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_user_email() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut user_email: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseUserEmail(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_email = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseUserEmail(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_email = c_char_to_string(&buffer);
    }
    println!("User Email: {}", user_email);
    println!("Result: {}", status);
    if status == 0 {
        Ok(user_email)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_user_name() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut user_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseUserName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseUserName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_name = c_char_to_string(&buffer);
    }
    println!("User Name: {}", user_name);
    println!("Result: {}", status);
    if status == 0 {
        Ok(user_name)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_user_company() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut user_company: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseUserCompany(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_company = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseUserCompany(buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_company = c_char_to_string(&buffer);
    }
    println!("User Company: {}", user_company);
    println!("Result: {}", status);
    if status == 0 {
        Ok(user_company)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_user_metadata(key: &str) -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut user_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key = to_utf16(key);
        status = unsafe { extern_functions::GetLicenseUserMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key);
        status = unsafe { extern_functions::GetLicenseUserMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        user_metadata = c_char_to_string(&buffer);
    }
    println!("User Metadata: {}", user_metadata);
    println!("Result: {}", status);
    if status == 0 {
        Ok(user_metadata)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_organization_name() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut organization_name: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseOrganizationName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        organization_name = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseOrganizationName(buffer.as_mut_ptr(), LENGTH as c_uint) };
        organization_name = c_char_to_string(&buffer);
    }
    println!("Organization Name: {}", organization_name);
    println!("Result: {}", status);
    if status == 0 {
        Ok(organization_name)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_license_organization_address() -> Result<OrganizationAddress, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut org_address_json: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseOrganizationAddressInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        org_address_json = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseOrganizationAddressInternal(buffer.as_mut_ptr(), LENGTH as c_uint) };
        org_address_json = c_char_to_string(&buffer);
    }
    let org_address: OrganizationAddress = serde_json::from_str(&org_address_json).expect("Failed to parse JSON");
    println!("Organization Address: {:?}", org_address);
    println!("Result: {}", status);
    if status == 0 {
        Ok(org_address)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }

}

pub fn get_license_type() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut license_type: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseType(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_type = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLicenseType(buffer.as_mut_ptr(), LENGTH as c_uint) };
        license_type = c_char_to_string(&buffer);
    }
    println!("License type: {}", license_type);
    println!("Result: {}", status);
    if status == 0 {
        Ok(license_type)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_activation_metadata(key: &str) -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut activation_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key = to_utf16(key);
        status = unsafe { extern_functions::GetActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        activation_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key);
        status = unsafe { extern_functions::GetActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        activation_metadata = c_char_to_string(&buffer);
    }
    println!("Activation Metadata: {}", activation_metadata);
    println!("Result: {}", status);
    if status == 0 {
        Ok(activation_metadata)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_activation_mode() -> Result<ActivationMode, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut initial_activation_mode: String;
    let mut current_activation_mode: String;
    #[cfg(windows)]
    {
        let mut initial_mode_buffer: [u16; LENGTH] = [0; LENGTH];
        let mut current_mode_buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetActivationMode(initial_mode_buffer.as_mut_ptr(), LENGTH as c_uint, current_mode_buffer.as_mut_ptr(), LENGTH as c_uint) };
        initial_activation_mode = utf16_to_string(&initial_mode_buffer);
        current_activation_mode = utf16_to_string(&current_mode_buffer);
    }
    #[cfg(not(windows))]
    {
        let mut initial_mode_buffer: [c_char; LENGTH] = [0; LENGTH];
        let mut current_mode_buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetActivationMode(initial_mode_buffer.as_mut_ptr(), LENGTH as c_uint, current_mode_buffer.as_mut_ptr(), LENGTH as c_uint) };
        initial_activation_mode = c_char_to_string(&initial_mode_buffer);
        current_activation_mode = c_char_to_string(&current_mode_buffer);
    }
    let activation_mode = ActivationMode {
        initial_mode: initial_activation_mode,
        current_mode: current_activation_mode,
    };
    println!("Activation Mode: {:?}", activation_mode);
    println!("Result: {}", status);
    if status == 0 {
        Ok(activation_mode)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_activation_meter_attribute_uses(name: &str) -> Result<u32, LexActivatorErrorCode> {
    let status: i32;
    let mut count: c_uint = 0;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { extern_functions::GetActivationMeterAttributeUses(c_name.as_ptr(), &mut count,) };
    }
    #[cfg(not(windows))]
    {
        // let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_name: CString  = string_to_cstring(name);
        status = unsafe { extern_functions::GetActivationMeterAttributeUses(c_name.as_ptr(), &mut count) };
    }
    println!("Activation Metadata: {}", count);
    println!("Result: {}", status);
    if status == 0 {
        Ok(count)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_server_sync_grace_period_expiry_date() -> Result<u32, LexActivatorErrorCode> {
    let status: i32;
    let mut expiry_date: c_uint = 0;
    status = unsafe { extern_functions::GetServerSyncGracePeriodExpiryDate(&mut expiry_date) };
    println!("Server Sync Grace Period Expiry Date: {}", expiry_date);
    println!("Result: {}", status);
    if status == 0 {
        Ok(expiry_date)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_trial_activation_metadata(key: &str) -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut trial_activation_metadata: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        let c_key = to_utf16(key);
        status = unsafe { extern_functions::GetTrialActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_activation_metadata = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        let c_key: CString  = string_to_cstring(key);
        status = unsafe { extern_functions::GetTrialActivationMetadata(c_key.as_ptr(), buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_activation_metadata = c_char_to_string(&buffer);
    }
    println!("Trial Activation Metadata: {}", trial_activation_metadata);
    println!("Result: {}", status);
    if status == 0 {
        Ok(trial_activation_metadata)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_trial_expiry_date() -> Result<u32, LexActivatorErrorCode> {
    let status: i32;
    let mut trial_expiry_date: c_uint = 0;
    status = unsafe { extern_functions::GetTrialExpiryDate(&mut trial_expiry_date) };
    println!("Trial Expiry Date: {}", trial_expiry_date);
    println!("Result: {}", status);
    if status == 0 {
        Ok(trial_expiry_date)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_trial_id() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut trial_id: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetTrialId(buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_id = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetTrialId(buffer.as_mut_ptr(), LENGTH as c_uint) };
        trial_id = c_char_to_string(&buffer);
    }
    println!("Trial ID: {}", trial_id);
    println!("Result: {}", status);
    if status == 0 {
        Ok(trial_id)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_local_trial_expiry_date() -> Result<u32, LexActivatorErrorCode> {
    let status: i32;
    let mut trial_expiry_date: c_uint = 0;
    status = unsafe { extern_functions::GetLocalTrialExpiryDate(&mut trial_expiry_date) };
    println!("Local Trial Expiry Date: {}", trial_expiry_date);
    println!("Result: {}", status);
    if status == 0 {
        Ok(trial_expiry_date)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

pub fn get_library_version() -> Result<String, LexActivatorErrorCode> {
    let status: i32;
    const LENGTH: usize = 256; // Set the appropriate buffer length
    let mut library_version: String;
    #[cfg(windows)]
    {
        let mut buffer: [u16; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLibraryVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        library_version = utf16_to_string(&buffer);
    }
    #[cfg(not(windows))]
    {
        let mut buffer: [c_char; LENGTH] = [0; LENGTH];
        status = unsafe { extern_functions::GetLibraryVersion(buffer.as_mut_ptr(), LENGTH as c_uint) };
        library_version = c_char_to_string(&buffer);
    }
    println!("Library Version: {}", library_version);
    println!("Result: {}", status);
    if status == 0 {
        Ok(library_version)
    } else {
        return Err(LexActivatorErrorCode::from(status));
    }
}

// ------------------ Action Functions ------------------

pub fn activate_license() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::ActivateLicense() };
    println!("Activate License inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn activate_license_offline(file_path: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { extern_functions::ActivateLicenseOffline(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path);
        status = unsafe { extern_functions::ActivateLicenseOffline(c_file_path.as_ptr()) };
    }
    println!("Activate License Offline inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn generate_offline_activation_request(file_path: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { extern_functions::GenerateOfflineActivationRequest(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path);
        status = unsafe { extern_functions::GenerateOfflineActivationRequest(c_file_path.as_ptr()) };
    }
    println!("Generate Offline Activation Request inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn deactivate_license() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::DeactivateLicense() };
    println!("Deactivate License inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn generate_offline_deactivation_request(file_path: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { extern_functions::GenerateOfflineDeactivationRequest(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path);
        status = unsafe { extern_functions::GenerateOfflineDeactivationRequest(c_file_path.as_ptr()) };
    }
    println!("Generate Offline Deactivation Request inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn is_license_genuine() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::IsLicenseGenuine() };
    println!("IsLicenseGenuine inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn is_license_valid() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::IsLicenseValid() };
    println!("IsLicenseValid inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn activate_trial() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::ActivateTrial() };
    println!("ActivateTrial inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn activate_trial_offline(file_path: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { extern_functions::ActivateTrialOffline(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path);
        status = unsafe { extern_functions::ActivateTrialOffline(c_file_path.as_ptr()) };
    }
    println!("ActivateTrialOffline inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn generate_offline_trial_activation_request(file_path: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_file_path = to_utf16(file_path);
        status = unsafe { extern_functions::GenerateOfflineTrialActivationRequest(c_file_path.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_file_path: CString  = string_to_cstring(file_path);
        status = unsafe { extern_functions::GenerateOfflineTrialActivationRequest(c_file_path.as_ptr()) };
    }
    println!("GenerateOfflineTrialActivationRequest inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn is_trial_genuine() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::IsTrialGenuine() };
    println!("IsTrialGenuine inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn activate_local_trial(trial_length: u32) -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let c_trial_length: c_uint = trial_length as c_uint;
    let status = unsafe { extern_functions::ActivateLocalTrial(c_trial_length) };
    println!("ActivateLocalTrial inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn is_local_trial_genuine() -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let status = unsafe { extern_functions::IsLocalTrialGenuine() };
    println!("IsLocalTrialGenuine inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn extend_local_trial(trial_extension_length: u32) -> Result<LexActivatorStatusCode, LexActivatorErrorCode> {
    let c_trial_extension_length: c_uint = trial_extension_length as c_uint;
    let status = unsafe { extern_functions::ExtendLocalTrial(c_trial_extension_length) };
    println!("ExtendLocalTrial inside: {}", status);
    match status {
        0 => Ok(LexActivatorStatusCode::from(status)), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn increment_activation_meter_attribute_uses(name: &str, increment: u32) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    let c_increment: c_uint = increment as c_uint;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { extern_functions::IncrementActivationMeterAttributeUses(c_name.as_ptr(), c_increment) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name);
        status = unsafe { extern_functions::IncrementActivationMeterAttributeUses(c_name.as_ptr(), c_increment) };
    }
    println!("IncrementActivationMeterAttributeUses inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn decrement_activation_meter_attribute_uses(name: &str, decrement: u32) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    let c_decrement: c_uint = decrement as c_uint;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { extern_functions::DecrementActivationMeterAttributeUses(c_name.as_ptr(), c_decrement) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name);
        status = unsafe { extern_functions::DecrementActivationMeterAttributeUses(c_name.as_ptr(), c_decrement) };
    }
    println!("DecrementActivationMeterAttributeUses inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn reset_activation_meter_attribute_uses(name: &str) -> Result<(), LexActivatorErrorCode> {
    let status: i32;
    #[cfg(windows)]
    {
        let c_name = to_utf16(name);
        status = unsafe { extern_functions::ResetActivationMeterAttributeUses(c_name.as_ptr()) };
    }
    #[cfg(not(windows))]
    {
        let c_name: CString  = string_to_cstring(name);
        status = unsafe { extern_functions::ResetActivationMeterAttributeUses(c_name.as_ptr()) };
    }
    println!("ResetActivationMeterAttributeUses inside: {}", status);
    match status {
        0 => Ok(()), // Include the desired status code
        _ => Err(LexActivatorErrorCode::from(status)),
    }
}

pub fn reset() -> Result<(), LexActivatorErrorCode> {
    let status = unsafe { extern_functions::Reset() };
    println!("Reset inside: {}", status);
    if status != 0 {
        return Err(LexActivatorErrorCode::from(status));
    } else {
        Ok(())
    }
}
