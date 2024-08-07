use std::path::Path;
use anyhow::Context;
use std::path::PathBuf;
use std::time::{Instant};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

wasmtime::component::bindgen!({
    path: "wit/read.wit",
    world: "discovery",
    async: true
});

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    println!("Ramping up ..");

    let parameter = "Joe".to_string();

    let my_component = Path::new("./reader.wasm").to_path_buf();

    let start = Instant::now();
    let (greeter, store) = init(my_component).await?;
    let duration = start.elapsed();
    println!("Initialization .. took {:?}", duration);

    let result = process(parameter, greeter, store).await?;
    println!("Getting back: '{}'", result);
    
    Ok(())
}

pub async fn init(path: PathBuf) -> wasmtime::Result<(Discovery, Store<i32>)> {
    let mut configuration = Config::new();
    configuration.wasm_component_model(true);
    configuration.async_support(true);

    let engine = Engine::new(&configuration).unwrap();
    let linker = Linker::<i32>::new(&engine);

    let mut store = Store::new(&engine, 0);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    let worker = Discovery::instantiate_async(&mut store, &component, &linker)
        .await
        .context("Failed to instantiate the example world")?;

    return Ok((worker, store));
}

pub async fn process(parameter: String, worker: Discovery, mut store: Store<i32>) -> wasmtime::Result<String> {
    worker
        .interface0
        .call_read(&mut store, &parameter)
        .await
        .context("Failed to call add function")        
}
