use wamr_rust_sdk::{
    runtime::Runtime, module::Module, instance::Instance, function::Function,
    value::WasmValue, RuntimeError
};
use std::ffi::c_void;
use esp_idf_svc::{sys, log::EspLogger};
use log::*;

extern "C" fn extra() -> i32 {
    100
}

// Use the Rust-compiled plugin
const RUST_PLUGIN: &[u8] = include_bytes!("../resources/plugins/rust_plugin.wasm");

fn main() -> Result<(), RuntimeError> {
    // Initialize ESP-IDF
    // sys::link_patches();
    EspLogger::initialize_default();
    
    info!("----- Starting WAMR ESP32 example");

    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("extra", extra as *mut c_void)
        .build()?;
    
    log::info!("----- Parsing WASM module!");
    let module = match Module::from_vec(&runtime, RUST_PLUGIN.to_vec(), "rust_plugin") {
        Ok(m) => {
            log::info!("----- WASM module parsed successfully");
            m
        },
        Err(e) => {
            log::error!("----- Failed to parse WASM module: {:?}", e);
            return Err(e);
        }
    };
    
    log::info!("----- Creating runtime with fixed memory");
    let params: Vec<WasmValue> = vec![WasmValue::I32(9), WasmValue::I32(27)];

    let instance = Instance::new(&runtime, &module, 1024 * 64)?;
    let function = Function::find_export_func(&instance, "add")?;
    
    let result = function.call(&instance, &params)?;
    log::info!("----- Result: {:?}", result);

    Ok(())
}
