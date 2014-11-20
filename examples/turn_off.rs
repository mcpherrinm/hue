//! An example of using the raw types and bridge API
extern crate hue;
extern crate serialize;
use serialize::json::ToJson;
use hue::rest_api::light::Light;

fn main() {
  let mut bridge = hue::bridge::Bridge::new("192.168.1.10".to_string(), "newdeveloper".to_string());
  let mut state = hue::rest_api::light::State{on: None, bri: None, hue: None, sat: None, xy: None, ct: None, alert: None, effect: None, colormode: None, reachable: None, transitiontime: None };
  state.on = Some(false);
  bridge.set_state("1", state);
  bridge.set_state("2", state);
  bridge.set_state("3", state);
  bridge.set_state("4", state);
}
