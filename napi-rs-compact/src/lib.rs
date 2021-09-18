#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsNumber, JsObject, JsString, Result};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("sum", sum)?;
  exports.create_named_method("helloPlusWorld", hello_plus_world)?;
  Ok(())
}

#[js_function(2)]
fn sum(ctx: CallContext) -> Result<JsNumber> {
  let a = ctx.get::<JsNumber>(0)?.get_uint32()?;
  let b = ctx.get::<JsNumber>(1)?.get_uint32()?;
  ctx.env.create_uint32(a + b)
}

#[js_function(1)]
fn hello_plus_world(ctx: CallContext) -> Result<JsString> {
  let hello = ctx.get::<JsString>(0)?.into_utf8()?;
  ctx
    .env
    .create_string_from_std(hello.as_str()?.to_owned() + " world")
}
