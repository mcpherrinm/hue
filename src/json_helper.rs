use serde::json::Value;

pub trait FromJson {
  fn from_json(j: &Value) -> Option<Self>;
}

pub trait ToJson {
  fn to_json(&self) -> Value;
}

impl FromJson for bool {
  fn from_json(jb: &Value) -> Option<bool> {
    match jb {
      &Value::Bool(b) => Some(b),
      _ => None
    }
  }
}

impl ToJson for bool {
    fn to_json(&self) -> Value {
        Value::Bool(*self)
    }
}

impl FromJson for u8 {
  fn from_json(j: &Value) -> Option<u8> {
    match j {
      &Value::U64(n) => Some(n as u8),
      _ => None
    }
  }
}

impl ToJson for u8 {
    fn to_json(&self) -> Value {
        Value::U64(*self as u64)
    }
}

impl FromJson for u16 {
  fn from_json(j: &Value) -> Option<u16> {
    match j {
      &Value::U64(n) => Some(n as u16),
      _ => None
    }
  }
}

impl ToJson for u16 {
    fn to_json(&self) -> Value {
        Value::U64(*self as u64)
    }
}

impl FromJson for f32 {
  fn from_json(j: &Value) -> Option<f32> {
    match j {
      &Value::F64(n) => Some(n as f32),
      _ => None
    }
  }
}

impl ToJson for f32 {
    fn to_json(&self) -> Value {
        Value::F64(*self as f64)
    }
}

impl FromJson for String {
  fn from_json(j: &Value) -> Option<String> {
    // This is really crummy that I need to copy.
    match j {
      &Value::String(ref s) => Some(s.clone()),
      _ => None
    }
  }
}

impl ToJson for String {
    fn to_json(&self) -> Value {
        Value::String(self.clone())
    }
}

impl<T, U> FromJson for (T, U) where T: FromJson, U: FromJson {
  fn from_json(j: &Value) -> Option<(T, U)> {
    match j {
      &Value::Array(ref vec) => {
          if vec.len() != 2 { return None };
          let (f1, f2) = (FromJson::from_json(&vec[0]),
                          FromJson::from_json(&vec[1]));
          if f1.is_some() && f2.is_some() {
            return Some((f1.unwrap(), f2.unwrap()));
          }
          None
        },
      _ => None
    }
  }
}

impl<T, U> ToJson for (T, U) where T: ToJson, U: ToJson {
    fn to_json(&self) -> Value {
        Value::Array(vec![self.0.to_json(), self.1.to_json()])
    }
}


#[test]
fn test() {
  let b: Option<bool> = FromJson::from_json(&Value::U64(1));
  assert_eq!(None, b);
}
