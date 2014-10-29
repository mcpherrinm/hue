#![feature(macro_rules)]
#![feature(if_let)]

extern crate serialize;
mod json_helper;

// rest_api and bridge are lower level APIs that follow the Philips Hue Api
// design closely.  They underly the wrappers in this file.
pub mod rest_api;
pub mod bridge;

// Should this be a trait Hue impld on bridge?
/// A Hue API handle.
struct Hue {
  bridge: bridge::Bridge

}

struct LightIter;
struct Light;

impl Hue {
  fn new() -> Hue {
    // Need to replace with bridge discovery
    // and proper auth stuff.
    let host = "192.168.1.10".to_string();
    Hue { bridge: bridge::Bridge::new(host, "newdeveloper".to_string()) }
  }

  /// A light controller for the Nth light
  fn light(index: uint) -> Option<Light> {
    None
  }

  /// Iterate over all lights
  fn light_iter() -> LightIter {
    LightIter
  }
}
