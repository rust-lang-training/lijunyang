use hex;
use neon::prelude::*;
use ring::digest;
use std::fs::File;
use std::io::Read;
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn test01(mut cx: FunctionContext) -> JsResult<JsString> {
    let filename = cx.argument::<JsString>(0)?.value(&mut cx);
    let file = File::open(filename).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = [0; 1024 * 64];
    let mut content = String::from("");
    let mut ctx = ring::digest::Context::new(&ring::digest::SHA256);
    loop {
        match reader.read(&mut buffer) {
            Ok(n) if n > 0 => {
                // let s = String::from_utf8_lossy(&buffer[..n]);
                // content.push_str(&s);
                // println!("{}", s);

                ctx.update(&buffer[..n]);
            }
            Ok(_) => break,
            Err(e) => panic!("error reading file: {}", e),
        }
    }
    // let digest = ring::digest::digest(&ring::digest::SHA256, content.as_bytes());
    // println!("{:?}", digest);

    let res = ctx.finish();
    let hash_hex = hex::encode(res);

    // Ok(cx.string(hex::encode(digest)))
    Ok(cx.string(hash_hex))
}

fn request_std_token(mut cx: FunctionContext) -> JsResult<JsObject> {
    let bucket_name = cx
        .argument::<JsString>(0)?
        // .expect("bucketName must be string")
        .value(&mut cx);
    println!("bucket_name: {}", bucket_name);
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
    let object_key = cx.string(object_key);
    let ttl_seconds = cx.number(ttl_seconds);

    let obj = cx.empty_object();

    obj.set(&mut cx, "token", token)?;
    obj.set(&mut cx, "bucketName", bucket_name)?;
    obj.set(&mut cx, "objectKey", object_key)?;
    obj.set(&mut cx, "ttlSeconds", ttl_seconds)?;

    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("requestStdToken", request_std_token)?;
    cx.export_function("test01", test01)?;
    Ok(())
}
