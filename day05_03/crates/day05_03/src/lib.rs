use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn request_sts_token(mut cx: FunctionContext) -> JsResult<JsObject> {
    let bucket_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let object_key = cx.argument::<JsString>(1)?.value(&mut cx);
    let ttl_seconds = match cx.argument_opt(2) {
        Some(hv) if hv.is_a::<JsNumber, _>(&mut cx) => {
            let nv = hv
                .downcast::<JsNumber, _>(&mut cx)
                .unwrap_or(cx.number(3600));
            let n = nv.value(&mut cx);
            n as u32
        }
        _ => 3600u32,
    };

    let token = cx.string("this is a dummy token");
    let bucket_name = cx.string(bucket_name);
    let object_Key = cx.string(object_key);
    let ttl_seconds = cx.number(ttl_seconds);
    let obj = cx.empty_object();
    obj.set(&mut cx, "token", token)?;
    obj.set(&mut cx, "bucketName", bucket_name)?;
    obj.set(&mut cx, "objectKey", object_Key)?;
    obj.set(&mut cx, "ttlSeconds", ttl_seconds)?;
    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("requestStsToken", request_sts_token)?;
    Ok(())
}
