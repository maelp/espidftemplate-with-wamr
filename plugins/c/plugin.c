#include <stdint.h>
#include <stdlib.h>

// Import the log_message function from the host
extern int log_message(const char* message);

// Export functions to be called from the host
__attribute__((export_name("print_message")))
int32_t print_message() {
    log_message("Hello from C WASM plugin!");
    return 42;
}

__attribute__((export_name("add")))
int32_t add(int32_t a, int32_t b) {
    return a + b;
}