use std::env;
use deno_core::op;
use deno_core::Extension;
use std::rc::Rc;
use deno_core::error::AnyError;

#[op]
async fn op_read_file(path:String)->Result<String,AnyError>{
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path:String,contents:String)->Result<(),AnyError>{
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path:String)->Result<(),AnyError>{
    std::fs::remove_file(path)?;
    Ok(())
}

#[op]
fn op_create_file(path:String)->Result<(),AnyError>{
    std::fs::File::create(path)?;
    Ok(())
}

async fn run_js(file_path:&str)->Result<(),AnyError>{
    let main_module = deno_core::resolve_path(file_path,env::current_dir()?.as_path())?;
    let run_js_extension = Extension::builder("runjs").ops(vec![
        op_read_file::decl(),
        op_write_file::decl(),
        op_remove_file::decl(),
        op_create_file::decl(),
    ]).build();
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions:vec![run_js_extension],
        ..Default::default()
    });
    let file_contents = deno_core::FastString::Static(include_str!("./runtime.js"));
    js_runtime.execute_script("[runjs:runtime.js]", file_contents).unwrap();
    let mod_id = js_runtime.load_main_module(&main_module,None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    if let Err(error) = runtime.block_on(run_js("/Users/Z009TZW/Documents/my_files/my-own-js-runtime/example.js")){
        eprintln!("error : {}",error);
    }

}
