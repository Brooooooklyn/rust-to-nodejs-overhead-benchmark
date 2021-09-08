#[macro_use]
extern crate napi_derive;

use napi::{CallContext, JsNumber, JsObject, Result};

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("sum", sum)?;
  Ok(())
}

#[js_function(2)]
fn sum(ctx: CallContext) -> Result<JsNumber> {
  let a = ctx.get::<JsNumber>(0)?.get_uint32()?;
  let b = ctx.get::<JsNumber>(1)?.get_uint32()?;
  ctx.env.create_uint32(a + b)
}
