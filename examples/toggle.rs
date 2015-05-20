//! An example of using the raw types and bridge API
extern crate hue;
use hue::rest_api::light::{self, Light};
use std::default::Default;

fn main() {
  let mut bridge = hue::bridge::Bridge::new("192.168.1.10".to_string(), "newdeveloper".to_string());
  let mut state: light::State = Default::default();
  state.transitiontime = Some(0);
  state.bri = Some(255);
  state.hue = Some(0);
  state.sat = Some(0);
  for _ in 0..15 {
      println!("Turning light off!");
      state.on = Some(false);
      bridge.set_state("1", state);
      println!("Turning light on!");
      state.on = Some(true);
      bridge.set_state("1", state);
  }
}
