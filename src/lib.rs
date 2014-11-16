#![feature(macro_rules)]
#![feature(if_let)]

extern crate serialize;
use rest_api::light::Light as RestLight;
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

impl Hue {
  pub fn new() -> Hue {
    // Need to replace with bridge discovery
    // and proper auth stuff.
    let host = "192.168.1.10".to_string();
    Hue { bridge: bridge::Bridge::new(host, "newdeveloper".to_string()) }
  }

  /// A light controller for the Nth light
  pub fn light<'a>(&'a mut self, index: uint) -> Option<Light<'a>> {
    None
  }

  /// Iterate over all lights
  pub fn lights<'a>(&'a mut self) -> LightIter<'a> {
    let lights = self.bridge.get_all().unwrap();
    LightIter{handle: self, lights: lights}
  }
}

pub struct LightIter<'a> {
  /// The Hue this was created for
  handle: &'a mut Hue,
  /// All the lights at the start of iteration
  lights: Vec<(String, rest_api::light::Attributes)>,
}

impl<'a> Iterator<Light<'a>> for LightIter<'a> {
  fn next(&mut self) -> Option<Light<'a>> {
    None
  }
}

pub struct Light<'a>;
