//! A direct translation of the Hue API into Rust
//! The mapping of these types and functions to the Hue REST Api should be
//! trivial.  Where possible, types are re-used for receiving and sending.
//! Enums are used in place of strings when there is a fixed set of values.

pub mod light {
  use std::collections::TreeMap;
  use serialize::json;
  use serialize::json::{Json, ToJson};
  use super::super::json_helper::FromJson;

  /// All except transitiontime are returned when you read the state on a hue.
  /// Hue flux bulbs don't have color info.  The bridge is buggy if you set
  /// conflicting color mode options.  Reachable is always true.
  /// Possibly this should be three structs, as some members are only get/set
  pub struct State {
    pub on: Option<bool>,
    pub bri: Option<u8>,
    pub hue: Option<u16>,
    pub sat: Option<u8>,
    pub xy: Option<(f32, f32)>,
    pub ct: Option<u16>,
    pub alert: Option<Alert>,
    pub effect: Option<Effect>,
    pub colormode: Option<ColorMode>,
    pub reachable: Option<bool>,
    pub transitiontime: Option<u16>
  }

  // Macros to the rescue for verbose code!
  macro_rules! maybe_insert_named(
    ($slf: ident, $object: ident, $field:ident, $json_name: expr) => (
      if let Some($field) = $slf.$field {
        $object.insert($json_name.to_string(), $field.to_json());
      }
    )
  )
  macro_rules! maybe_insert(
    ($slf: ident, $object: ident, $($field:ident),* ) => ({
      $( maybe_insert_named!($slf, $object, $field, stringify!($field)) )*
    })
  )

  impl ToJson for State {
    fn to_json(&self) -> Json {
      let mut object = TreeMap::new();
      maybe_insert!(self, object, on, bri, hue, sat, xy, ct, alert, effect);
      maybe_insert!(self, object, colormode, reachable, transitiontime);
      json::Object(object)
    }
  }

  macro_rules! find_and(
    ($map: ident, $field: expr) => (
      $map.find(&$field.to_string()).and_then(|x| FromJson::from_json(x))
    )
  )

  impl State {
    pub fn from_json(object: Json) -> Option<State> {
      match object {
        json::Object(map) =>
          Some(State {
            on: find_and!(map, "on"),
            bri: find_and!(map, "bri"),
            hue: find_and!(map,"hue"),
            sat: find_and!(map, "bri"),
            // This is the worst line of Rust I have ever written:
            // The map *lookup* requires an allocation to get a &String, and
            // then the resulting Json object is turned into a string just so
            // it can be parsed again by the json library into the right type.
            xy: map.find(&"xy".to_string()).and_then(|x| json::decode(x.to_string().as_slice()).ok()),
            ct: find_and!(map, "ct"),
            alert: find_and!(map, "alert"),
            effect: find_and!(map, "effect"),
            colormode: find_and!(map, "colormode"),
            reachable: find_and!(map, "reachable"),
            transitiontime: find_and!(map, "transitiontime"),
          }),
        _ => None,
      }
    }
  }

  pub enum ColorMode { HueSat, CieXy, ColorTemperature }
  impl ToJson for ColorMode {
    fn to_json(&self) -> Json {
      json::String(
        match *self {
          HueSat => "hs",
          CieXy => "xy",
          ColorTemperature => "ct"
        }.to_string())
    }
  }

  impl FromJson for ColorMode {
    fn from_json(json: &Json) -> Option<ColorMode> {
      match json {
        &json::String(ref s) => {
          match s.as_slice() {
            "hs" => Some(HueSat),
            "xy" => Some(CieXy),
            "ct" => Some(ColorTemperature),
            _ => None,
          }
        }
        _ => None
      }
    }
  }

  pub enum Alert { NoAlert, Select, LSelect }
  impl ToJson for Alert {
    fn to_json(&self) -> json::Json {
      json::String(
        match *self {
          NoAlert => "none",
          Select => "select",
          LSelect => "lselect"
        }.to_string())
    }
  }

  impl FromJson for Alert {
    fn from_json(json: &Json) -> Option<Alert> {
      match json {
        &json::String(ref s) => {
          match s.as_slice() {
            "none" => Some(NoAlert),
            "select" => Some(Select),
            "lselect" => Some(LSelect),
            _ => None,
          }
        }
        _ => None
      }
    }
  }

  pub enum Effect { NoEffect, ColorLoop }
  impl ToJson for Effect {
    fn to_json(&self) -> json::Json {
      json::String(
        match *self {
          NoEffect => "none",
          ColorLoop => "colorloop",
        }.to_string())
    }
  }

  impl FromJson for Effect {
    fn from_json(json: &Json) -> Option<Effect> {
      match json {
        &json::String(ref s) => {
          match s.as_slice() {
            "none" => Some(NoEffect),
            "colorloop" => Some(ColorLoop),
            _ => None,
          }
        }
        _ => None
      }
    }
  }

  /// Returned from /api/<username>/lights/<id>
  pub struct Attributes {
    pub state: State,
    pub type_: String,
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub pointsymbol: PointSymbol,
  }

  // No ToJson implementation for Attributes since we only recieve it.

  /// Reserved for future use, apparently.
  pub struct PointSymbol;

  #[test]
  fn test_encode_state() {
    //! A wholly incomplete test
    let mut state = State{on: None, bri: None, hue: None, sat: None, xy: None, ct: None, alert: None, effect: None, colormode: None, reachable: None, transitiontime: None };
    assert_eq!(state.to_json().to_string(), "{}".to_string());
    state.on = Some(true);
    assert_eq!(state.to_json().to_string(),
               "{\"on\":true}".to_string());
    state.bri = Some(100);
    assert_eq!(state.to_json().to_string(),
               "{\"bri\":100,\"on\":true}".to_string());
    state.bri = None;
    state.on = Some(false);
    assert_eq!(state.to_json().to_string(),
               "{\"on\":false}".to_string());
  }



}



