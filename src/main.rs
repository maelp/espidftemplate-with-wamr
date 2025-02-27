use wamr_rust_sdk::{
    runtime::Runtime, module::Module, instance::Instance, function::Function,
    value::WasmValue, RuntimeError
};
use std::ffi::c_void;
use esp_idf_svc::{sys, log::EspLogger};
use log::*;
use std::thread;
use std::sync::mpsc;

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
        
    // Run WASM in a separate thread
    match run_wasm_in_thread() {
        Ok(result) => {
            info!("Thread result: {:?}", result);
        },
        Err(e) => {
            error!("Thread execution failed: {:?}", e);
            return Err(e);
        }
    }
    
    info!("----- WASM execution completed");
    Ok(())
}

// Run WASM in a separate thread and return the result
fn run_wasm_in_thread() -> Result<Vec<WasmValue>, RuntimeError> {
    // Create channel for thread communication
    let (tx, rx) = mpsc::channel();
    
    // Spawn thread for WASM execution
    info!("----- Spawning WASM execution thread");
    thread::spawn(move || {
        info!("----- WASM thread started");
        
        // Execute WASM and send result through channel
        match run_wasm() {
            Ok(result) => {
                info!("----- WASM execution successful in thread");
                tx.send(Ok(result)).unwrap();
            },
            Err(e) => {
                error!("----- WASM execution failed in thread: {:?}", e);
                tx.send(Err(e)).unwrap();
            }
        }
        
        info!("----- WASM thread completed");
    });
    
    // Wait for thread result with timeout
    info!("----- Waiting for thread result");
    match rx.recv() {
        Ok(result) => result,
        Err(e) => {
            error!("Failed to receive thread result: {:?}", e);
            Err(RuntimeError::InitializationFailure)
        }
    }
}

// Actually run the WASM module
fn run_wasm() -> Result<Vec<WasmValue>, RuntimeError> {
    info!("----- Configuring WAMR runtime");
    
    // Configure runtime with standard features
    let runtime = Runtime::builder()
        .run_as_interpreter() // Use interpreter mode for stability
        .use_system_allocator()
        .register_host_function("extra", extra as *mut c_void)
        .build()?;

    info!("----- WAMR runtime built successfully");

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
    info!("----- Free heap before WASM instantiation: {} bytes", free_heap);
    
    // Increase memory sizes
    info!("----- Attempting to create instance");
    let instance = match Instance::new_with_args(
        &runtime,
        &module,
        64 * 1024,
        64 * 1024
    ) {
        Ok(inst) => {
            info!("----- Successfully created instance");
            inst
        },
        Err(e) => {
            error!("Failed to create instance: {:?}", e);
            return Err(e);
        }
    };
    
    info!("----- Successfully instantiated WASM module");
    
    let function = match Function::find_export_func(&instance, "add") {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to find 'add' function: {:?}", e);
            return Err(e);
        }
    };
    
    info!("----- Found 'add' function, executing simple test");
    
    // Create parameters for the function call
    let params: Vec<WasmValue> = vec![WasmValue::I32(5), WasmValue::I32(7)];

    match function.call(&instance, &params) {
        Ok(result) => {
            info!("----- WASM function result: {:?}", result);
            Ok(result)
        },
        Err(e) => {
            error!("Function call failed: {:?}", e);
            Err(e)
        }
    }
}
