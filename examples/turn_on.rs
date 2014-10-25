//! An example of using the raw types and bridge API
extern crate hue;
extern crate serialize;
use serialize::json::ToJson;

fn main() {
  let mut bridge = hue::bridge::Bridge::new("192.168.1.10".to_string(), "newdeveloper".to_string());
  let mut state = hue::api::light::State{on: None, bri: None, hue: None, sat: None, xy: None, ct: None, alert: None, effect: None, colormode: None, reachable: None, transitiontime: None };
  state.on = Some(true);
  bridge.put::<bool>("lights/1/state", Some(state.to_json()));
}
