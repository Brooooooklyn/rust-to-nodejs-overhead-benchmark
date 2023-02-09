#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsNumber, JsObject, JsString, Result};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("sum", sum)?;
  exports.create_named_method("helloPlusWorld", hello_plus_world)?;
  exports.create_named_method("area", area)?;
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

#[js_function(1)]
fn area(ctx: CallContext) -> Result<JsNumber> {
  let rect = ctx.get::<JsObject>(0)?;

  let width = rect.get_named_property_unchecked::<JsNumber>("width")?;
  let w = width.get_uint32()?;

  let height = rect.get_named_property_unchecked::<JsNumber>("height")?;
  let h = height.get_uint32()?;

  ctx.env.create_uint32(w * h)
}
