# lexactivator

lexactivator is a rust wrapper for cryptlex's licensing SDK that lets you implement any type of licensing model such as node-locked, hosted floating licenses, trials and much more. This SDK offers support for online and offline activations.

## Usage

In your Cargo.toml:

```toml
[dependencies]
lexactivator = { version = "3.0.0"}
```

Simple example usage:

```rust

use lexactivator::*;

fn main() {
    let product_data: String = String::from("PASTE_THE CONTENT_OF_PRODUCT.DAT_FILE");
    let product_id: String = String::from("PRODUCT_ID");
    let license_key: String = String::from("LICENSE_KEY");
    let mut result: Result<(), LexActivatorError> = lexactivator::set_product_data(product_data);
    match result {
    Ok(()) => {
        // Product data set successfully
        println!("Product data set successfully.");
    }
    Err(error) => {
        // Error occurred while setting product data
        println!("Error while setting product data: {:?}", error);
    }
    }
    result = lexactivator::set_product_id(product_id, PermissionFlags::LA_USER);
    match result {
    Ok(()) => {
        // Product Id set successfully
        println!("Product id set successfully.");
    }
    Err(error) => {
        // Error occurred while setting product id
        println!("Error while setting product id: {:?}", error);
    }
    }
    result = lexactivator::set_license_key(license_key);
    match result {
    Ok(()) => {
        // License Key set successfully
        println!("License key set successfully.");
    }
    Err(error) => {
        // Error occurred while setting license key
        println!("Error while setting license key: {:?}", error);
    }
    }
    let activation_result: Result<LexActivatorStatus, LexActivatorError> = lexactivator::activate_license();
    match activation_result {
        Ok(LexActivatorStatus::LA_OK) => {
            println!("License activated successfully");
        }
        Ok(_) => {
            // Other success cases if needed
        }
        
        Err(error) => {
            println!("License activation failed: {:?}", error);
        }
    }

}

## License

This project is licensed under 

* [MIT License](https://opensource.org/licenses/MIT)
