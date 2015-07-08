//! A direct translation of the Hue API into Rust
//! The mapping of these types and functions to the Hue REST Api should be
//! trivial.  Where possible, types are re-used for receiving and sending.
//! Enums are used in place of strings when there is a fixed set of values.
use serde;

/// Lots of APIs returned a success/error data type:
/// http://www.developers.meethue.com/documentation/error-messages
#[derive(Clone)]
pub struct Status {
  pub success: bool,
  pub value: String
}

impl super::json_helper::FromJson for Status {
  fn from_json(json: &serde::json::Value) -> Option<Status> {
    match json {
      &serde::json::Value::String(ref s) => {
          println!("value is {}", s);
          None
      }
      _ => None
    }
  }
}

pub mod light {
  use std::collections::btree_map::BTreeMap;
  use serde::json::Value;
  use super::Status;
  use json_helper::{self, ToJson, FromJson};

  /// The trait describing lights REST endpoints on the API.  Implemented by Bridge
  pub trait Light {
    /// GET /lights
    fn get_all(&mut self) -> Option<Vec<(String, Attributes)>>;

    /// GET /lights/<id>
    fn get_attributes(&mut self, id: &str) -> Option<Attributes>;

    /// PUT /lights/<id>/state
    fn set_state(&mut self, id: &str, state: State) -> Option<Status>;

    /// PUT /lights/<id>
    fn rename(&mut self, name: &str) -> Option<Status>;
  }

  /// All except transitiontime are returned when you read the state on a hue.
  /// Hue flux bulbs don't have color info.  The bridge is buggy if you set
  /// conflicting color mode options.
  /// Possibly this should be three structs, as some members are only get/set
  #[derive(Default, Copy, Clone)]
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
  macro_rules! maybe_insert_named {
    ($slf: ident, $object: ident, $field:ident, $json_name: expr) => (
      if let Some($field) = $slf.$field {
        $object.insert($json_name.to_string(), $field.to_json());
      }
    )
  }
  macro_rules! maybe_insert {
    ($slf: ident, $object: ident, $($field:ident),* ) => ({
      $( maybe_insert_named!($slf, $object, $field, stringify!($field)); )*
    })
  }

  impl json_helper::ToJson for State {
    fn to_json(&self) -> Value {
      let mut object = BTreeMap::new();
      maybe_insert!(self, object, on, bri, hue, sat, xy, ct, alert, effect);
      maybe_insert!(self, object, colormode, reachable, transitiontime);
      Value::Object(object)
    }
  }

  macro_rules! find_from_json {
    ($map: ident, $field: expr) => (
      $map.get($field).and_then(|x| json_helper::FromJson::from_json(x))
    )
  }

  impl json_helper::FromJson for State {
    fn from_json(object: &Value) -> Option<State> {
      match object {
        &Value::Object(ref map) =>
          Some(State {
            on: find_from_json!(map, "on"),
            bri: find_from_json!(map, "bri"),
            hue: find_from_json!(map,"hue"),
            sat: find_from_json!(map, "bri"),
            xy: find_from_json!(map, "xy"),
            ct: find_from_json!(map, "ct"),
            alert: find_from_json!(map, "alert"),
            effect: find_from_json!(map, "effect"),
            colormode: find_from_json!(map, "colormode"),
            reachable: find_from_json!(map, "reachable"),
            transitiontime: find_from_json!(map, "transitiontime"),
          }),
        _ => None,
      }
    }
  }

  #[derive(Copy, Clone)]
  pub enum ColorMode { HueSat, CieXy, ColorTemperature }
  impl json_helper::ToJson for ColorMode {
    fn to_json(&self) -> Value {
      Value::String(
        match *self {
          ColorMode::HueSat => "hs",
          ColorMode::CieXy => "xy",
          ColorMode::ColorTemperature => "ct"
        }.to_string())
    }
  }

  impl json_helper::FromJson for ColorMode {
    fn from_json(json: &Value) -> Option<ColorMode> {
      match json {
        &Value::String(ref s) => {
          match &s[..] {
            "hs" => Some(ColorMode::HueSat),
            "xy" => Some(ColorMode::CieXy),
            "ct" => Some(ColorMode::ColorTemperature),
            _ => None,
          }
        }
        _ => None
      }
    }
  }

  #[derive(Copy, Clone)]
  pub enum Alert { None, Select, LSelect }
  impl json_helper::ToJson for Alert {
    fn to_json(&self) -> Value {
      Value::String(
        match *self {
          Alert::None => "none",
          Alert::Select => "select",
          Alert::LSelect => "lselect"
        }.to_string())
    }
  }

  impl json_helper::FromJson for Alert {
    fn from_json(json: &Value) -> Option<Alert> {
      match json {
        &Value::String(ref s) => {
          match &s[..] {
            "none" => Some(Alert::None),
            "select" => Some(Alert::Select),
            "lselect" => Some(Alert::LSelect),
            _ => None,
          }
        }
        _ => None
      }
    }
  }

  #[derive(Copy, Clone)]
  pub enum Effect { None, ColorLoop }
  impl json_helper::ToJson for Effect {
    fn to_json(&self) -> Value {
      Value::String(
        match *self {
          Effect::None=> "none",
          Effect::ColorLoop => "colorloop",
        }.to_string())
    }
  }

  impl json_helper::FromJson for Effect {
    fn from_json(json: &Value) -> Option<Effect> {
      match json {
        &Value::String(ref s) => {
          match &s[..] {
            "none" => Some(Effect::None),
            "colorloop" => Some(Effect::ColorLoop),
            _ => None,
          }
        }
        _ => None
      }
    }
  }

  #[derive(Clone)]
  pub struct Attributes {
    pub state: State,
    pub type_: String,
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub pointsymbol: PointSymbol,
  }

  // No ToJson implementation for Attributes since we only recieve it.

  impl json_helper::FromJson for Attributes {
    fn from_json(json: &Value) -> Option<Attributes> {
      match json {
        &Value::Object(ref map) =>
          Some(Attributes{
          // These should return None instead of failing
            state: find_from_json!(map, "state").unwrap(),
            type_: find_from_json!(map, "type").unwrap(),
            name: find_from_json!(map, "name").unwrap(),
            modelid: find_from_json!(map, "modelid").unwrap(),
            swversion: find_from_json!(map, "swversion").unwrap(),
            pointsymbol: PointSymbol,
          }),
        _ => None,
      }
    }
  }

  /// Reserved for future use, apparently.
  #[derive(Clone, Copy)]
  pub struct PointSymbol;

  #[test]
  fn test_encode_state() {
    //! A wholly incomplete test
    let mut state = State{on: None, bri: None, hue: None, sat: None, xy: None, ct: None, alert: None, effect: None, colormode: None, reachable: None, transitiontime: None };
    assert_eq!(json::to_string(&state.to_json()).unwrap(), "{}".to_string());
    state.on = Some(true);
    assert_eq!(json::to_string(&state.to_json()).unwrap(),
               "{\"on\":true}".to_string());
    state.bri = Some(100);
    assert_eq!(json::to_string(&state.to_json()).unwrap(),
               "{\"bri\":100,\"on\":true}".to_string());
    state.bri = None;
    state.on = Some(false);
    assert_eq!(json::to_string(&state.to_json()).unwrap(),
               "{\"on\":false}".to_string());
  }
}
