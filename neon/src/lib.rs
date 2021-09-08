use neon::prelude::*;

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
  let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
  Ok(cx.number((a + b) as u32))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("sum", sum)?;
  Ok(())
}
