extern crate hyper;

use serialize::json::{Json, ToJson};
use super::json_helper::FromJson;
use super::rest_api;
use std::io::{Reader, Writer};

pub struct Bridge {
  host: String,
  username: String,
}

impl Bridge {
  pub fn new(host: String, username: String) -> Bridge {
    Bridge{ host: host, username: username }
  }


  fn host(&self) -> &str { self.host.as_slice() }
  fn username(&self) -> &str { self.username.as_slice() }

  pub fn get<T: FromJson>(&mut self, path: &str) -> Option<T> {
    self.request(hyper::Get, path, None)
  }

  pub fn put<T: FromJson>(&mut self, path: &str, body: Json) -> Option<T> {
    self.request(hyper::method::Method::Put, path, Some(body))
  }

  /// Do a request to the bridge API.  Don't include the /api/username portion in path.
  pub fn request<T: FromJson>(&mut self, method: hyper::method::Method, path: &str, body: Option<Json>) -> Option<T> {
    let url = hyper::Url::parse(format!("http://{}/api/{}{}", self.host(), self.username(), path).as_slice()).unwrap();
    let mut request = hyper::client::request::Request::new(method, url).unwrap().start().unwrap();
    if let Some(body) = body {
      request.write_str(body.to_string().as_slice());
    }
    let mut resp = request.send();
    match resp {
      Ok(mut resp) => {
        if resp.status != hyper::status::Ok { return None };
        let body = resp.read_to_string().unwrap();
        // debug print:
        println!("Response: {}", body);
        //
        super::json_helper::FromJson::from_json(&body.to_json())
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
    self.get(format!("/lights/{}", id).as_slice())
  }

  fn set_state(&mut self, id: &str, state: rest_api::light::State) -> Option<rest_api::Status> {
    self.put(format!("/lights/{}/state", id).as_slice(), state.to_json())
  }

  fn rename(&mut self, name: &str) -> Option<rest_api::Status> {
    None
  }

}
