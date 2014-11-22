extern crate curl;

use serialize::json::{Json, ToJson};
use super::json_helper::FromJson;
use super::rest_api;

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

  pub fn put<T: FromJson>(&mut self, path: &str, body: Json) -> Option<T> {
    self.request(curl::http::handle::Put, path, Some(body))
  }

  /// Do a request to the bridge API.  Don't include the /api/username portion in path.
  pub fn request<T: FromJson>(&mut self, method: curl::http::handle::Method, path: &str, body: Option<Json>) -> Option<T> {
    let uri = format!("http://{}/api/{}{}", self.host(), self.username(), path);
    println!("Url: {}", uri);
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
        println!("Response: {}", ::std::str::from_utf8(resp.get_body()));
        //
        super::json_helper::FromJson::from_json(&resp.get_body().to_json())
      },
      _ => None
    }
  }
}

impl rest_api::light::Light for Bridge {
  fn get_all(&mut self) -> Option<Vec<(String, rest_api::light::Attributes)>> {
    // TODO: Deserialize "/lights" properly
    Some(vec![])
  }

  fn get_attributes(&mut self, id: &str) -> Option<rest_api::light::Attributes> {
    self.get(format!("/lights/{}", id).as_slice())
  }

  fn set_state(&mut self, id: &str, state: rest_api::light::State) -> Option<rest_api::Status> {
    self.put(format!("/lights/{}/state", id).as_slice(), state.to_json())
  }

  fn rename(&mut self, name: &str) -> Option<rest_api::Status> {
    None
  }

}
