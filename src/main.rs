use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
    RuntimeError,
};

// We've simplified our plugins to not need any host functions, 
// but keeping this commented for reference
/*
extern "C" fn log_message(message_ptr: *const i8) -> i32 {
    unsafe {
        if message_ptr.is_null() {
            log::error!("[WASM] Null message pointer");
            return -1;
        }
        
        // Simple C string handling - find null terminator and convert to str
        let mut len = 0;
        while *message_ptr.add(len) != 0 && len < 100 {
            len += 1;
        }
        
        let slice = std::slice::from_raw_parts(message_ptr as *const u8, len);
        match std::str::from_utf8(slice) {
            Ok(s) => log::info!("[WASM] {}", s),
            Err(_) => log::error!("[WASM] Invalid UTF-8 string"),
        }
    }
    0
}
*/

// Use the Rust-compiled plugin
const RUST_PLUGIN: &[u8] = include_bytes!("../resources/plugins/rust_plugin.wasm");

fn load_and_run_wasm_plugin(runtime: &Runtime, name: &str, wasm_bytes: &[u8]) -> Result<(), RuntimeError> {
    log::info!("Loading {} WASM plugin ({} bytes)", name, wasm_bytes.len());
    
    // Step 1: Load the WASM module from memory
    log::info!("Parsing WASM module binary...");
    let module = match Module::from_vec(runtime, wasm_bytes.to_vec(), name) {
        Ok(m) => {
            log::info!("WASM module parsed successfully");
            m
        },
        Err(e) => {
            log::error!("Failed to parse WASM module: {:?}", e);
            return Err(e);
        }
    };
    
    // Try with zero additional memory - use only what's in the module
    log::info!("Creating WASM instance with only built-in memory");
    let instance = match Instance::new(runtime, &module, 0) {
        Ok(i) => {
            log::info!("WASM instance created successfully");
            i
        },
        Err(e) => {
            log::error!("Failed to create WASM instance: {:?}", e);
            return Err(e);
        }
    };
    
    // Step 3: Find and call the single function from our minimal Rust plugin
    if let Ok(function) = Function::find_export_func(&instance, "say_42") {
        log::info!("Found 'say_42' function, calling it...");
        let empty_params: Vec<WasmValue> = vec![];
        match function.call(&instance, &empty_params) {
            Ok(results) => {
                log::info!("Success! say_42() returned: {:?}", results);
                // The function returns a Vec<WasmValue>
                if let Some(result) = results.first() {
                    match result {
                        WasmValue::I32(val) => log::info!("The answer is: {}", val),
                        _ => log::info!("Unexpected return type")
                    }
                } else {
                    log::info!("No return value");
                }
            },
            Err(e) => {
                log::error!("Failed to call 'say_42' function: {:?}", e);
            }
        }
    } else {
        log::warn!("'say_42' function not found");
    }
    
    log::info!("WASM execution completed successfully");
    Ok(())
}

fn main() -> Result<(), RuntimeError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting WAMR ESP32 example");
    
    // Configure and create a minimal WAMR runtime
    log::info!("Creating runtime with fixed memory");
    
    // Configure with fixed memory, no system allocator
    let runtime = match Runtime::builder()
        // Don't use system allocator which might be causing the issue
        // .use_system_allocator()
        .build() {
        Ok(r) => {
            log::info!("Runtime built successfully");
            r
        },
        Err(e) => {
            log::error!("Failed to build runtime: {:?}", e);
            return Err(e);
        }
    };
    
    log::info!("WAMR runtime built successfully");
    
    // Run the Rust WASM plugin
    log::info!("\n\n=== Running Rust WASM Plugin ===");
    
    // Let's be really explicit about the plugin size for debugging
    log::info!("Plugin size: {} bytes", RUST_PLUGIN.len());
    
    // Try various memory sizes to see what works
    match load_and_run_wasm_plugin(&runtime, "Rust", RUST_PLUGIN) {
        Ok(_) => log::info!("Rust WASM plugin executed successfully"),
        Err(e) => {
            log::error!("Failed with 0 bytes of additional memory: {:?}", e);
            
            // Try again with 128 bytes
            log::info!("Retrying with 128 bytes of additional memory...");
            let retry_fn = |rt: &Runtime| -> Result<(), RuntimeError> {
                log::info!("Loading Rust WASM plugin (retry)");
                let module = Module::from_vec(rt, RUST_PLUGIN.to_vec(), "Rust-retry")?;
                let instance = Instance::new(rt, &module, 128)?;
                
                if let Ok(function) = Function::find_export_func(&instance, "say_42") {
                    let empty_params: Vec<WasmValue> = vec![];
                    let results = function.call(&instance, &empty_params)?;
                    log::info!("Results: {:?}", results);
                }
                
                Ok(())
            };
            
            if let Err(e2) = retry_fn(&runtime) {
                log::error!("Retry also failed: {:?}", e2);
            } else {
                log::info!("Retry succeeded with 128 bytes of memory!");
            }
        },
    }
    
    log::info!("WAMR ESP32 example completed");
    
    Ok(())
}
