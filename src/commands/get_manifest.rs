use polywrap_wasm_rs::{
    JSON,
};

use crate::{WrapClientModule, imported::ArgsGetManifest};

pub fn get_manifest(path_parts: Vec<String>) -> Option<JSON::Value> {
    let uri = path_parts.join("/");

    let result = WrapClientModule::get_manifest(&ArgsGetManifest {
        uri,
    });

    match result {
        Ok(result) => {
            if let Some(manifest) = result {
                return Some(manifest);
            }

            return None;
        }
        Err(_) => {
            return None;
        }
    }
} 
