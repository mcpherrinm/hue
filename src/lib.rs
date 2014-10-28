#![feature(macro_rules)]
#![feature(if_let)]

extern crate serialize;
mod json_helper;

// rest_api and bridge are lower level APIs that follow the Philips Hue Api
// design closely.  They underly the wrappers in this file.
pub mod rest_api;
pub mod bridge;

/// A Hue API handle.
struct Hue {
  bridge: bridge::Bridge
}
