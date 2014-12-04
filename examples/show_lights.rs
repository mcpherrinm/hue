//! An example of using the raw types and bridge API
extern crate hue;
extern crate serialize;
use serialize::json::ToJson;
use hue::rest_api::light::Light;

fn main() {
  let mut bridge = hue::bridge::Bridge::new("192.168.1.10".to_string(),
                                            "newdeveloper".to_string());
  for light in bridge.get_all().unwrap().into_iter() {
    println!("{}: {}", light.0, light.1.name);
  }
}
