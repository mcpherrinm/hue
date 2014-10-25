extern crate curl;

use serialize::json::{Json, ToJson};
use super::json_helper::FromJson;

pub struct Bridge {
  host: String,
  username: String,
  handle: curl::http::Handle,
}

impl Bridge {
  pub fn new(host: String, username: String) -> Bridge {
    Bridge{ host: host, username: username, handle: curl::http::handle() }
  }


  fn host(&self) -> &str { self.host.as_slice() }
  fn username(&self) -> &str { self.username.as_slice() }

  pub fn get<T: FromJson>(&mut self, path: &str) -> Option<T> {
    let url = format!("http://{:s}/api/{:s}/{:s}", self.host(), self.username(), path);
    let resp = self.handle.get(url).exec();
    match resp {
      Ok(resp) => {
        if resp.get_code() != 200 { return None };
        super::json_helper::FromJson::from_json(&resp.get_body().to_json())
      },
      _ => None
    }
  }

  /// Do a POST request to the bridge API.  Don't include the /api/username portion.
  pub fn post<T: FromJson>(&mut self, path: &str, body: Option<Json>) -> Option<T> {
    let url = format!("http://{:s}/api/{:s}/{:s}", self.host(), self.username(), path);
    let bs = if body.is_some() {
      body.to_string()
    } else {
      "".to_string()
    };
    let resp = self.handle.post(url, bs.as_slice()).exec();
    match resp {
      Ok(resp) => {
        if resp.get_code() != 200 { return None };
        super::json_helper::FromJson::from_json(&resp.get_body().to_json())
      },
      _ => None
    }
  }
}
