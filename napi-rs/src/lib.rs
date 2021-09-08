#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

#[napi]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}
