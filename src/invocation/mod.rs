use eyre::Result;
use polywrap_wasm_rs::{Context, ReadDecoder, Read};
use crate::wrap::wrap_link_header::WrapLinkHeader;
use crate::wrap::wrap_link_json::deserialize_wrap_link_json;
use crate::wrap::wrap_link_file::deserialize_wrap_link_file;
use crate::wrap::wrap_link_response::{WrapLinkResponse, deserialize_wrap_link_response};

pub fn convert_invocation_result_to_response(result: Option<Vec<u8>>) -> Option<WrapLinkResponse> {
  if result == None {
      return None;
  }

  let result = result.unwrap();

  match read_wrap_link_result(result) {
      Ok(result) => match result {
          WrapLinkResult::String(result) => {
              return Some(WrapLinkResponse {
                  data: Some(result.into_bytes()),
                  headers: Some(vec![WrapLinkHeader {
                      name: "Content-Type".to_string(),
                      value: "text/html".to_string()
                  }])
              });
          },
          WrapLinkResult::Msgpack(result) => {
            return Some(WrapLinkResponse {
                data: Some(result),
                headers: Some(vec![WrapLinkHeader {
                    name: "Content-Type".to_string(),
                    value: "msgpack".to_string()
                }])
            });
          },
          WrapLinkResult::WrapLinkJson {
              _wrap_link_type,
              content,
          } => {
              return Some(WrapLinkResponse {
                  data: Some(content.into_bytes()),
                  headers: Some(vec![WrapLinkHeader {
                      name: "Content-Type".to_string(),
                      value: "application/json".to_string()
                  }])
              });
          },
          WrapLinkResult::WrapLinkFile {
              _wrap_link_type,
              content,
              content_type
          } => {
              return Some(WrapLinkResponse {
                  data: Some(content),
                  headers: Some(vec![WrapLinkHeader {
                      name: "Content-Type".to_string(),
                      value: content_type.to_string()
                  }])
              });
          },
          WrapLinkResult::WrapLinkResponse {
              data,
              headers,
          } => {
              return Some(WrapLinkResponse {
                  data: data,
                  headers: headers
              });
          }
      },
      Err(_) => None
  }
}

enum WrapLinkResult {
  String(String),
  Msgpack(Vec<u8>),
  WrapLinkJson {
      _wrap_link_type: String,
      content: String,
  },
  WrapLinkFile {
      _wrap_link_type: String,
      content: Vec<u8>,
      content_type: String,
  },
  WrapLinkResponse {
    headers: Option<Vec<WrapLinkHeader>>,
    data: Option<Vec<u8>>,
  }
}

fn read_wrap_link_result(result: Vec<u8>) -> Result<WrapLinkResult> {
  let mut context = Context::new();
  context.description = "Deserializing invocation result".to_string();
  let mut reader = ReadDecoder::new(&result, context);


  if reader.is_next_string()? {
      let str = reader.read_string()?;
      return Ok(WrapLinkResult::String(str));
  }

  if let Ok(result) = deserialize_wrap_link_json(&result) {
      return Ok(WrapLinkResult::WrapLinkJson {
          _wrap_link_type: result._wrap_link_type,
          content: result.content,
      });
  }

  if let Ok(result) = deserialize_wrap_link_file(&result) {
      return Ok(WrapLinkResult::WrapLinkFile {
          _wrap_link_type: result._wrap_link_type,
          content: result.content,
          content_type: result.content_type,
      });
  }

  if let Ok(result) = deserialize_wrap_link_response(&result) {
    return Ok(WrapLinkResult::WrapLinkResponse {
        headers: result.headers,
        data: result.data,
    });
}

  Ok(
    WrapLinkResult::Msgpack(result)
  )
}