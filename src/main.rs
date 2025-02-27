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

    // Use the simpler direct approach without threading
    run_wasm()?;
    
    info!("WASM execution completed");
    Ok(())
}

// Let's remove unused code completely and focus on the core functionality

// Actually run the WASM module
fn run_wasm() -> Result<(), RuntimeError> {
    info!("Configuring WAMR runtime");
    
    // Configure runtime with standard features
    let runtime = Runtime::builder()
        .run_as_interpreter() // Use interpreter mode for stability
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
    
    // Increase memory sizes
    info!("Attempting to create instance");
    let instance = match Instance::new(
        &runtime,
        &module,
        2 * 1024
    ) {
        Ok(inst) => {
            info!("Successfully created instance");
            inst
        },
        Err(e) => {
            error!("Failed to create instance: {:?}", e);
            return Err(e);
        }
    };
    
    info!("Successfully instantiated WASM module");
    
    let function = match Function::find_export_func(&instance, "add") {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to find 'add' function: {:?}", e);
            return Err(e);
        }
    };
    
    info!("Found 'add' function, executing simple test");
    
    // Create parameters for the function call
    let params: Vec<WasmValue> = vec![WasmValue::I32(5), WasmValue::I32(7)];
    
    match function.call(&instance, &params) {
        Ok(result) => {
            info!("WASM function result: {:?}", result);
        },
        Err(e) => {
            error!("Function call failed: {:?}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
