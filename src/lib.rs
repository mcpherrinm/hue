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
pub struct Hue {
  bridge: bridge::Bridge

}

pub struct LightIter;
pub struct Light;

impl Hue {
  pub fn new() -> Hue {
    // Need to replace with bridge discovery
    // and proper auth stuff.
    let host = "192.168.1.10".to_string();
    Hue { bridge: bridge::Bridge::new(host, "newdeveloper".to_string()) }
  }

  /// A light controller for the Nth light
  pub fn light(index: uint) -> Option<Light> {
    None
  }

  /// Iterate over all lights
  pub fn light_iter() -> LightIter {
    LightIter
  }
}
