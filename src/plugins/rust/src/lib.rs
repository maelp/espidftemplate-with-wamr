#[no_mangle]
pub extern "C" fn print_message() -> i32 {
    // This function will be imported from the host
    unsafe { 
        let _ = log_message("Hello from Rust WASM plugin!"); 
    }
    42
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Import the log_message function from the host
extern "C" {
    fn log_message(ptr: &str) -> i32;
}