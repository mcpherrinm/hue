extern crate serde;
extern crate hyper;
mod json_helper;

use rest_api::light::Light as RestLight;

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
  pub fn light(&mut self, index: usize) -> Option<OneLight> {
    self.lights().nth(index)
  }

  /// Iterate over all lights
  pub fn lights(&mut self) -> LightIter {
    let lights = self.bridge.get_all().unwrap();
    LightIter{handle: self, lights: lights.into_iter()}
  }
}

/// The Light trait describes some set of lights that you apply normal light
/// operations for.  Implemented on individual lights, the all lights object,
/// and groups. (Well, when I finish)
trait Light {
  //fn get_color() -> LightColor;
  //fn set_color(color: LightColor);

}

pub struct LightIter<'a> {
  /// The Hue this was created for
  handle: &'a mut Hue,
  /// All the lights at the start of iteration
  lights: ::std::vec::IntoIter<(String, rest_api::light::Attributes)>,
}

impl<'a> Iterator for LightIter<'a> {
  type Item = OneLight;
  fn next(&mut self) -> Option<OneLight> {
    match self.lights.next() {
      None => None,
      Some((id, attrs)) => {
        Some(OneLight{id: id, attrs: attrs})
      }
    }
  }
}

pub struct OneLight {
  id: String,
  attrs: rest_api::light::Attributes
}

// This is the main
impl Light for OneLight {

}
