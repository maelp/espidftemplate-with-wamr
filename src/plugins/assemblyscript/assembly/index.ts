// Extremely minimal AssemblyScript plugin for ESP32S3

// Export functions to be called from the host
export function print_message(): i32 {
  // Simply return a value without any imports
  return 42;
}

export function add(a: i32, b: i32): i32 {
  // Simple addition function
  return a + b;
}