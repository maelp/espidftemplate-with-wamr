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
    sys::link_patches(); // This is crucial for ESP-IDF integration
    EspLogger::initialize_default();
    
    info!("----- Starting WAMR ESP32 example");

    info!("Configuring WAMR runtime");
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("extra", extra as *mut c_void)
        .build()?;
    
    info!("WAMR runtime built successfully");
    
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
    
    info!("----- Creating runtime with fixed memory");
    
    // Print available heap info
    let free_heap = unsafe { sys::esp_get_free_heap_size() };
    info!("Free heap before WASM instantiation: {} bytes", free_heap);
    
    let params: Vec<WasmValue> = vec![WasmValue::I32(9), WasmValue::I32(27)];

    // Try with a small memory size first
    info!("Attempting to create instance with 4kb bytes memory");
    let instance = match Instance::new(&runtime, &module, 1024 * 4) {
        Ok(inst) => {
            info!("Successfully created instance with 4kb bytes memory");
            inst
        },
        Err(e) => {
            error!("Failed to create instance with 512 bytes: {:?}", e);
            return Err(e);
        }
    };
    
    info!("Successfully instantiated WASM module");
    let function = Function::find_export_func(&instance, "add")?;
    
    let result = function.call(&instance, &params)?;
    log::info!("----- Result: {:?}", result);

    Ok(())
}
