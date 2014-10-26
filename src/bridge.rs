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
    self.request(curl::http::handle::Get, path, None)
  }

  pub fn put<T: FromJson>(&mut self, path: &str, body: Option<Json>) -> Option<T> {
    self.request(curl::http::handle::Put, path, body)
  }

  /// Do a request to the bridge API.  Don't include the /api/username portion in path.
  pub fn request<T: FromJson>(&mut self, method: curl::http::handle::Method, path: &str, body: Option<Json>) -> Option<T> {
    let uri = format!("http://{:s}/api/{:s}/{:s}", self.host(), self.username(), path);
    let mut request = curl::http::handle::Request::new(&mut self.handle, method).uri(uri);
    let mut bs;
    if let Some(body) = body {
      bs = body.to_string();
      request = request.body(&bs);
    }
    let resp = request.exec();
    match resp {
      Ok(resp) => {
        if resp.get_code() != 200 { return None };
        // debug print:
        println!("{}", ::std::str::from_utf8(resp.get_body()));
        //
        super::json_helper::FromJson::from_json(&resp.get_body().to_json())
      },
      _ => None
    }
  }
}