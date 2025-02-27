// Absolute minimal Rust WASM plugin for ESP32S3
// With explicit memory allocation for WebAssembly

// Manually define a static memory region for WASM
// This is needed because WebAssembly requires linear memory
#[no_mangle]
static mut MEMORY: [u8; 128] = [0; 128];

#[no_mangle]
pub extern "C" fn say_42() -> i32 {
    // Force use of memory to ensure it's included in WASM
    unsafe {
        MEMORY[0] = 42;
    }
    42  // Just return the number 42
}