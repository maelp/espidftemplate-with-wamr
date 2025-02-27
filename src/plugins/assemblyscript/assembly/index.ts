// Import the log_message function from the host
// Note: In AssemblyScript, we use the "env" namespace for imports
@external("env", "log_message")
declare function log_message(message: string): i32;

// Export functions to be called from the host
export function print_message(): i32 {
  log_message("Hello from AssemblyScript WASM plugin!");
  return 42;
}

export function add(a: i32, b: i32): i32 {
  return a + b;
}