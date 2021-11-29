use napi_derive::napi;

#[napi]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}

#[napi]
fn hello_plus_world(hello: String) -> String {
  hello + " world"
}
