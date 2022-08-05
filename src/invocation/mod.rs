use eyre::Result;
use polywrap_wasm_rs::{Context, ReadDecoder, Read};
use crate::wrap::Header;
use crate::wrap::response::Response;
use crate::wrap::wrap_link_json::deserialize_wrap_link_json;
use crate::wrap::wrap_link_file::deserialize_wrap_link_file;

pub fn convert_invocation_result_to_response(result: Option<Vec<u8>>) -> Option<Response> {
  if result == None {
      return None;
  }

  let result = result.unwrap();

  match read_wrap_link_result(result) {
      Ok(result) => match result {
          WrapLinkResult::String(result) => {
              return Some(Response {
                  data: Some(result.into_bytes()),
                  headers: Some(vec![Header {
                      name: "Content-Type".to_string(),
                      value: "text/html".to_string()
                  }])
              });
          },
          WrapLinkResult::Msgpack(result) => {
            return Some(Response {
                data: Some(result),
                headers: Some(vec![Header {
                    name: "Content-Type".to_string(),
                    value: "msgpack".to_string()
                }])
            });
          },
          WrapLinkResult::WrapLinkJson {
              _wrap_link_type,
              content,
          } => {
              return Some(Response {
                  data: Some(content.into_bytes()),
                  headers: Some(vec![Header {
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
              return Some(Response {
                  data: Some(content),
                  headers: Some(vec![Header {
                      name: "Content-Type".to_string(),
                      value: content_type.to_string()
                  }])
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

  Ok(
    WrapLinkResult::Msgpack(result)
  )
}