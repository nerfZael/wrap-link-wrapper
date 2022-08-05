use JSON::json;
use polywrap_wasm_rs::{
    JSON,
};

use crate::{WrapClientModule, imported::ArgsGetSchema};

pub fn get_schema(path_parts: Vec<String>) -> Option<String> {
    let uri = path_parts.join("/");

    let result = WrapClientModule::get_schema(&ArgsGetSchema {
        uri,
    });

    match result {
        Ok(result) => {
            if let Some(schema) = result {
                return Some(schema);
            }

            return None;
        }
        Err(_) => {
            return None;
        }
    }
} 
