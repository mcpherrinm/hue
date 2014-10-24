use super::json_helper::FromJson;
use serialize::json::Json;


pub struct Bridge;

enum HttpMethod {
  GET, POST, PUT
}

impl Bridge {
  fn request<T: FromJson>(&mut self, method: HttpMethod, path: String, body: Json) -> Option<T> {
    None
  }

  pub fn get<T: FromJson>(&mut self, path: String, body: Json) -> Option<T> {
    self.request(GET, path, body)
  }

  pub fn put<T: FromJson>(&mut self, path: String, body: Json) -> Option<T> {
    self.request(PUT, path, body)
  }

  pub fn post<T: FromJson>(&mut self, path: String, body: Json) -> Option<T> {
    self.request(POST, path, body)
  }
}
