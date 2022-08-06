pub mod wrap;
mod commands;
mod invocation;

use commands::*;
use polywrap_wasm_rs::{EncodeError, Context, WriteEncoder, Write};
use wrap::*;
use invocation::*;

pub fn encode_string(str: String) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: invoke".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_string(str, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_string<W: Write>(str: String, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("str", "String", "writing result");
    writer.write_string(&str)?;
    writer.context().pop();
    Ok(())
}

pub fn get(args: ArgsGet) -> Option<WrapLinkResponse> {
    let result = get_command_with_path_parts(&args);

    if result.is_none() {
        return None;
    }

    let (command, path_parts) = result.unwrap();

    match command.as_str() {
        "i" => convert_invocation_result_to_response(execute_invoke(path_parts, args.args)),
        "schema" => match get_schema(path_parts) {
            Some(schema) => Some(WrapLinkResponse {
                data: Some(format!("<pre>{}<pre>", schema).into_bytes()),
                headers: Some(vec![WrapLinkHeader {
                    name: "Content-Type".to_string(),
                    value: "text/html".to_string()
                }])
            }),
            None => None,
        },
        "manifest" => match get_manifest(path_parts) {
            Some(manifest) => Some(WrapLinkResponse {
                data: Some(format!("<pre>{}<pre>", serde_json::to_string_pretty(&manifest).unwrap()).into_bytes()),
                headers: Some(vec![WrapLinkHeader {
                    name: "Content-Type".to_string(),
                    value: "text/html".to_string()
                }])
            }),
            None => None,
        },
        _ => None
    }
}

fn get_command_with_path_parts(args: &ArgsGet) -> Option<(String, Vec<String>)> {
    let command_with_path = &args.path;
    let parts: Vec<&str> = command_with_path.split("/").collect();

    if parts.len() <= 1 {
        return None;
    }

    let command = parts[0];
    let path_parts = parts[1..]
        .into_iter()
        .map(|a| a.to_string())
        .collect();

    Some((command.to_string(), path_parts))
}

#[cfg(test)]
mod tests {
    use std::vec;

    pub use crate::wrap::*;

    use crate::{ArgsGet, get_command_with_path_parts, wrap_link_file::serialize_wrap_link_file, encode_string, invocation::convert_invocation_result_to_response, wrap::wrap_link_json::serialize_wrap_link_json };

    #[test]
    fn invoke() {
        let args = ArgsGet {
            path: "i/ens/test.eth".to_string(),
            args: None
        };
        
        let result = get_command_with_path_parts(&args);

        assert_eq!(result.is_none(), false);
    
        let (command, path_parts) = result.unwrap();
    
        assert_eq!(command, "i");
        assert_eq!(path_parts, vec!["ens", "test.eth"]);
    }

    #[test]
    fn get_schema() {
        let args = ArgsGet {
            path: "schema/ens/ethereum.polywrap.eth".to_string(),
            args: None
        };
        
        let result = get_command_with_path_parts(&args);

        assert_eq!(result.is_none(), false);
    
        let (command, path_parts) = result.unwrap();
    
        assert_eq!(command, "schema");
        assert_eq!(path_parts, vec!["ens", "ethereum.polywrap.eth"]);
    }

    #[test]
    fn get_manifest() {
        let args = ArgsGet {
            path: "manifest/ens/test.eth".to_string(),
            args: None
        };
        
        let result = get_command_with_path_parts(&args);

        assert_eq!(result.is_none(), false);
    
        let (command, path_parts) = result.unwrap();
    
        assert_eq!(command, "manifest");
        assert_eq!(path_parts, vec!["ens", "test.eth"]);
    }

    #[test]
    fn deserialize_wrap_link_json() {
        let result = WrapLinkJson {
            _wrap_link_type: "json".to_string(),
            content: "hello".to_string()
        };
      
        let bytes = serialize_wrap_link_json(&result).unwrap();

        let response = convert_invocation_result_to_response(Some(bytes));

        assert_eq!(response.is_some(), true);

        let response = response.unwrap();
        let headers = response.headers.unwrap();
        let header = &headers[0];

        assert_eq!(header.name, "Content-Type");
        assert_eq!(header.value, "application/json");
    }

    #[test]
    fn deserialize_wrap_link_file() {
        let result = WrapLinkFile {
            _wrap_link_type: "json".to_string(),
            content: "hello".to_string().into_bytes(),
            content_type: "text/plain".to_string()
        };
      
        let bytes = serialize_wrap_link_file(&result).unwrap();

        let response = convert_invocation_result_to_response(Some(bytes));

        assert_eq!(response.is_some(), true);

        let response = response.unwrap();
        let headers = response.headers.unwrap();
        let header = &headers[0];

        assert_eq!(header.name, "Content-Type");
        assert_eq!(header.value, "text/plain");
    }

    #[test]
    fn deserialize_string() {
        let result = "test".to_string();
      
        let bytes = encode_string(result.clone()).unwrap();

        let response = convert_invocation_result_to_response(Some(bytes));

        assert_eq!(response.is_some(), true);

        let response = response.unwrap();
        let response = response.data.unwrap();
        let content = std::str::from_utf8(&response).unwrap().to_string();

        assert_eq!(content, result);
    }
}