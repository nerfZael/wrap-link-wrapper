use crate::{imported::ArgsInvoke, WrapClientModule};

pub fn execute_invoke(path_parts: Vec<String>, args: Option<Vec<u8>>) -> Option<Vec<u8>> {
  let uri = path_parts[..path_parts.len()-1].join("/");
  let method = path_parts[path_parts.len()-1].to_string();

  let result = WrapClientModule::invoke(&ArgsInvoke {
      uri,
      method,
      args,
  });

  match result {
      Ok(result) => {
          if let Some(data) = result.data {
              return Some(data);
          }

          return None;
      }
      Err(_) => {
          return None;
      }
  }
} 
