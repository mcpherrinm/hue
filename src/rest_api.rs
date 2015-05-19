//! A direct translation of the Hue API into Rust
//! The mapping of these types and functions to the Hue REST Api should be
//! trivial.  Where possible, types are re-used for receiving and sending.
//! Enums are used in place of strings when there is a fixed set of values.
use serde;
use super::json_helper::FromJson;

/// Lots of APIs returned a success/error data type:
/// http://www.developers.meethue.com/documentation/error-messages
pub struct Status {
  pub success: bool,
  pub value: String
}

impl super::json_helper::FromJson for Status {
  fn from_json(json: &serde::json::Value) -> Option<Status> {
    match json {
      &serde::json::Value::String(ref s) => {
          None
      }
      _ => None
    }
  }
}

pub mod light {
  use std::collections::btree_map::BTreeMap;
  use serde::json;
  use super::Status;

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
  #[derive(Default)]
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

/*
  impl ToJson for State {
    fn to_json(&self) -> Json {
      let mut object = BTreeMap::new();
      maybe_insert!(self, object, on, bri, hue, sat, xy, ct, alert, effect);
      maybe_insert!(self, object, colormode, reachable, transitiontime);
      json::Object(object)
    }
  }*/

  macro_rules! find_from_json {
    ($map: ident, $field: expr) => (
      $map.get($field).and_then(|x| super::super::json_helper::FromJson::from_json(x))
    )
  }

  impl super::super::json_helper::FromJson for State {
    fn from_json(object: &::serde::json::Value) -> Option<State> {
      match object {
        &::serde::json::Value::Object(ref map) =>
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

  pub enum ColorMode { HueSat, CieXy, ColorTemperature }
  /*impl ToJson for ColorMode {
    fn to_json(&self) -> Json {
      json::String(
        match *self {
          ColorMode::HueSat => "hs",
          ColorMode::CieXy => "xy",
          ColorMode::ColorTemperature => "ct"
        }.to_string())
    }
  }*/

  impl super::super::json_helper::FromJson for ColorMode {
    fn from_json(json: &::serde::json::Value) -> Option<ColorMode> {
      match json {
        &::serde::json::Value::String(ref s) => {
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

  pub enum Alert { None, Select, LSelect }
  /*impl ToJson for Alert {
    fn to_json(&self) -> json::Json {
      json::String(
        match *self {
          Alert::None => "none",
          Alert::Select => "select",
          Alert::LSelect => "lselect"
        }.to_string())
    }
  }*/

  impl super::super::json_helper::FromJson for Alert {
    fn from_json(json: &::serde::json::Value) -> Option<Alert> {
      match json {
        &::serde::json::Value::String(ref s) => {
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

  pub enum Effect { None, ColorLoop }
  /*
  impl ToJson for Effect {
    fn to_json(&self) -> json::Json {
      json::String(
        match *self {
          Effect::None=> "none",
          Effect::ColorLoop => "colorloop",
        }.to_string())
    }
  }*/

  impl super::super::json_helper::FromJson for Effect {
    fn from_json(json: &::serde::json::Value) -> Option<Effect> {
      match json {
        &::serde::json::Value::String(ref s) => {
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

  pub struct Attributes {
    pub state: State,
    pub type_: String,
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub pointsymbol: PointSymbol,
  }

  // No ToJson implementation for Attributes since we only recieve it.

  impl super::super::json_helper::FromJson for Attributes {
    fn from_json(json: &::serde::json::Value) -> Option<Attributes> {
      match json {
        &::serde::json::Value::Object(ref map) =>
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
