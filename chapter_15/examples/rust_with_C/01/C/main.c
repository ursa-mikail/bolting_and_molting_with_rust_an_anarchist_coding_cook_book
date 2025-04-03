#include <stdio.h>
#include <stdlib.h>
#include "../rust/include/rust_with_c.h"

int main() {
    printf("Starting Rust-C FFI example\n");
    
    // Example 1: Call the add function
    printf("Calling add function...\n");
    int sum = add(5, 7);
    printf("5 + 7 = %d\n", sum);
    
    // Example 2: Call the to_uppercase function
    printf("\nCalling to_uppercase function...\n");
    const char* input = "Hello, Rust from C!";
    char* uppercase = to_uppercase(input);
    
    if (uppercase != NULL) {
        printf("Original: %s\nUppercase: %s\n", input, uppercase);
        
        // Free the memory allocated by Rust
        printf("Freeing memory allocated by Rust...\n");
        free_rust_string(uppercase);
    } else {
        printf("Error converting string to uppercase\n");
    }
    
    // Example 3: Using a struct
    printf("\nCalling distance_from_origin function...\n");
    Point p = {3, 4};
    printf("Created Point with x=%d, y=%d\n", p.x, p.y);
    
    double distance = distance_from_origin(p);
    printf("Distance from origin to point (%d, %d): %.2f\n", p.x, p.y, distance);
    
    printf("\nAll FFI calls completed successfully!\n");
    return 0;
}