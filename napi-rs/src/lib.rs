use napi_derive::napi;

#[napi]
pub fn sum(a: u32, b: u32) -> u32 {
  a + b
}

#[napi]
pub fn hello_plus_world(hello: String) -> String {
  hello + " world"
}

#[napi(object)]
pub struct Rect {
  pub width: u32,
  pub height: u32,
}

#[napi]
pub fn area(rect: Rect) -> u32 {
  rect.width * rect.height
}
