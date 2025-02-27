use std::ffi::{c_void, CStr, CString};
use wamr_rust_sdk::{
    function::Function, instance::Instance, module::Module, runtime::Runtime, value::WasmValue,
    RuntimeError,
};

// Host function for WASM plugins to log messages
extern "C" fn log_message(message_ptr: *const i8) -> i32 {
    unsafe {
        if let Ok(c_str) = CStr::from_ptr(message_ptr).to_str() {
            log::info!("[WASM] {}", c_str);
        } else {
            log::error!("[WASM] Invalid UTF-8 string");
        }
    }
    0
}

// We need to embed the WASM binaries directly in the binary for ESP32
const RUST_PLUGIN: &[u8] = include_bytes!("../resources/plugins/rust_plugin.wasm");
const AS_PLUGIN: &[u8] = include_bytes!("../resources/plugins/as-plugin.wasm");
// The C plugin is optional - it will only be included if it exists
#[cfg(feature = "c_plugin")]
const C_PLUGIN: &[u8] = include_bytes!("../resources/plugins/c-plugin.wasm");

fn load_and_run_wasm_plugin(runtime: &Runtime, name: &str, wasm_bytes: &[u8]) -> Result<(), RuntimeError> {
    log::info!("Loading {} WASM plugin", name);
    
    // Load the WASM module from memory
    let module = Module::from_buffer(runtime, wasm_bytes)?;
    
    // Create an instance with 64KB of memory
    let instance = Instance::new(runtime, &module, 1024 * 64)?;
    
    // Find the print_message export function
    if let Ok(function) = Function::find_export_func(&instance, "print_message") {
        log::info!("Calling print_message function...");
        let result = function.call(&instance, &[])?;
        log::info!("print_message returned: {:?}", result);
    } else {
        log::warn!("print_message function not found");
    }
    
    // Find the add export function
    if let Ok(function) = Function::find_export_func(&instance, "add") {
        log::info!("Calling add function with params (5, 7)...");
        let params: Vec<WasmValue> = vec![WasmValue::I32(5), WasmValue::I32(7)];
        let result = function.call(&instance, &params)?;
        log::info!("add(5, 7) returned: {:?}", result);
    } else {
        log::warn!("add function not found");
    }
    
    Ok(())
}

fn main() -> Result<(), RuntimeError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting WAMR ESP32 example");
    
    // Create a WAMR runtime with system allocator and register the log_message function
    // For Rust plugins, we register without namespace
    // For AssemblyScript plugins, we need to register with "env" namespace
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("log_message", log_message as *mut c_void) // For Rust plugin
        .register_host_function("env.log_message", log_message as *mut c_void) // For AssemblyScript plugin
        .build()?;
    
    // Load and run the Rust plugin
    log::info!("\n\n=== Running Rust Plugin ===");
    match load_and_run_wasm_plugin(&runtime, "Rust", RUST_PLUGIN) {
        Ok(_) => log::info!("Rust plugin executed successfully"),
        Err(e) => log::error!("Failed to run Rust plugin: {:?}", e),
    }
    
    // Load and run the AssemblyScript plugin
    log::info!("\n\n=== Running AssemblyScript Plugin ===");
    match load_and_run_wasm_plugin(&runtime, "AssemblyScript", AS_PLUGIN) {
        Ok(_) => log::info!("AssemblyScript plugin executed successfully"),
        Err(e) => log::error!("Failed to run AssemblyScript plugin: {:?}", e),
    }
    
    // Load and run the C plugin if available
    #[cfg(feature = "c_plugin")]
    {
        log::info!("\n\n=== Running C Plugin ===");
        match load_and_run_wasm_plugin(&runtime, "C", C_PLUGIN) {
            Ok(_) => log::info!("C plugin executed successfully"),
            Err(e) => log::error!("Failed to run C plugin: {:?}", e),
        }
    }
    
    log::info!("WAMR ESP32 example completed");
    
    Ok(())
}
