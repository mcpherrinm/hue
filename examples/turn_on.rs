//! An example of using the raw types and bridge API
extern crate hue;
use std::default::Default;
use hue::rest_api::light::Light;

fn main() {
  println!("Version: {}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));
  let mut bridge = hue::bridge::Bridge::new("192.168.1.10".to_string(), "newdeveloper".to_string());
  let mut state: hue::rest_api::light::State = Default::default();
  state.on = Some(true);
  bridge.set_state("1", state);
  bridge.set_state("2", state);
  bridge.set_state("3", state);
  bridge.set_state("4", state);
}
