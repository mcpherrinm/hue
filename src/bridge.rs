use hyper;

use super::json_helper::{ToJson, FromJson};
use super::rest_api;
use serde;
use serde::json::{self, value, Value};
use std::io::{Read, Write};

pub struct Bridge {
  host: String,
  username: String,
}

impl Bridge {
  pub fn new(host: String, username: String) -> Bridge {
    Bridge{ host: host, username: username }
  }


  fn host(&self) -> &str { &self.host[..] }
  fn username(&self) -> &str { &self.username[..] }

  pub fn get<T: FromJson>(&mut self, path: &str) -> Option<T> {
    self.request(hyper::Get, path, None)
  }

  pub fn put<T: FromJson>(&mut self, path: &str, body: Value) -> Option<T> {
    self.request(hyper::method::Method::Put, path, Some(body))
  }

  /// Do a request to the bridge API.  Don't include the /api/username portion in path.
  pub fn request<T: FromJson>(&mut self, method: hyper::method::Method, path: &str, body: Option<Value>) -> Option<T> {
    let url = hyper::Url::parse(&format!("http://{}/api/{}{}", self.host(), self.username(), path)[..]).unwrap();
    let mut request = hyper::client::request::Request::new(method, url).unwrap().start().unwrap();
    if let Some(body) = body {
      json::to_string(&body).ok().and_then(|b| request.write(b.as_bytes()).ok());
    }
    let mut resp = request.send();
    match resp {
      Ok(mut resp) => {
        if resp.status != hyper::status::StatusCode::Ok { return None }
        let mut body: String = "".to_string();
        if !resp.read_to_string(&mut body).is_ok() { return None }
        json::from_str(&body[..]).ok().and_then(|v| super::json_helper::FromJson::from_json(&v))
      },
      _ => None
    }
  }
}

impl rest_api::light::Light for Bridge {
  fn get_all(&mut self) -> Option<Vec<(String, rest_api::light::Attributes)>> {
    // TODO: Deserialize "/lights" properly
    //self.get("/lights");
    Some(vec![])
  }

  fn get_attributes(&mut self, id: &str) -> Option<rest_api::light::Attributes> {
    self.get(&format!("/lights/{}", id)[..])
  }

  fn set_state(&mut self, id: &str, state: rest_api::light::State) -> Option<rest_api::Status> {
    self.put(&format!("/lights/{}/state", id)[..], state.to_json())
  }

  fn rename(&mut self, name: &str) -> Option<rest_api::Status> {
    None
  }

}
