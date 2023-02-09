use node_bindgen::core::val::JsObject;
use node_bindgen::core::{JSValue, NjError};
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}

#[node_bindgen]
fn hello_plus_world(hello: String) -> String {
  hello.to_owned() + " world"
}

pub struct Rect {
  width: u32,
  height: u32,
}

impl JSValue<'_> for Rect {
  fn convert_to_rust(
    env: &node_bindgen::core::val::JsEnv,
    js_value: node_bindgen::sys::napi_value,
  ) -> Result<Self, node_bindgen::core::NjError> {
    let obj = env.convert_to_rust::<JsObject>(js_value)?;
    let width = obj
      .get_property("width")?
      .ok_or(NjError::Other("width is missing".to_owned()))?;
    let height = obj
      .get_property("height")?
      .ok_or(NjError::Other("height is missing".to_owned()))?;
    Ok(Rect {
      width: width.as_value::<u32>()?,
      height: height.as_value::<u32>()?,
    })
  }
}

#[node_bindgen]
fn area(rect: Rect) -> u32 {
  rect.width * rect.height
}
