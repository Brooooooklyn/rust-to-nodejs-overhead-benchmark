use neon::prelude::*;

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
  let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
  Ok(cx.number((a + b) as u32))
}

fn hello_plus_world(mut cx: FunctionContext) -> JsResult<JsString> {
  let hello = cx.argument::<JsString>(0)?.value(&mut cx);
  Ok(cx.string(hello + " world"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("sum", sum)?;
  cx.export_function("helloPlusWorld", hello_plus_world)?;
  Ok(())
}
