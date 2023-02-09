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

fn area(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let rect: Handle<JsObject> = cx.argument(0)?;

  let width: Handle<JsNumber> = rect.get(&mut cx, "width")?;
  let w = width.value(&mut cx) as u32;

  let height: Handle<JsNumber> = rect.get(&mut cx, "height")?;
  let h = height.value(&mut cx) as u32;

  Ok(cx.number(w * h))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("sum", sum)?;
  cx.export_function("helloPlusWorld", hello_plus_world)?;
  cx.export_function("area", area)?;
  Ok(())
}
