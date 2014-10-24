/// Because honestly the existing parser is pants-on-head.
extern crate serialize;
use serialize::json;
use serialize::json::Json;

pub trait FromJson {
  fn from_json(j: &Json) -> Option<Self>;
}

impl FromJson for bool {
  fn from_json(jb: &Json) -> Option<bool> {
    match jb {
      &json::Boolean(b) => Some(b),
      _ => None
    }
  }
}
impl FromJson for u8 {
  fn from_json(j: &Json) -> Option<u8> {
    match j {
      &json::U64(n) => Some(n as u8),
      _ => None
    }
  }
}
impl FromJson for u16 {
  fn from_json(j: &Json) -> Option<u16> {
    match j {
      &json::U64(n) => Some(n as u16),
      _ => None
    }
  }
}
impl FromJson for f32 {
  fn from_json(j: &Json) -> Option<f32> {
    match j {
      &json::F64(n) => Some(n as f32),
      _ => None
    }
  }
}

impl FromJson for String {
  fn from_json(j: &Json) -> Option<String> {
    // This is really crummy that I need to copy.
    match j {
      &json::String(ref s) => Some(s.clone()),
      _ => None
    }
  }
}

#[test]
fn test() {
  let b: Option<bool> = FromJson::from_json(&json::U64(1));
  assert_eq!(None, b);
}
