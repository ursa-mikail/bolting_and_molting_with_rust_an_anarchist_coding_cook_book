#ifndef RUST_WITH_C_H
#define RUST_WITH_C_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Simple function that adds two integers
int add(int a, int b);

// String manipulation function
char* to_uppercase(const char* input);

// Function to free strings allocated by Rust
void free_rust_string(char* s);

// Struct definition matching Rust's Point
typedef struct {
    int x;
    int y;
} Point;

// Function that works with the struct
double distance_from_origin(Point point);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // RUST_WITH_C_H
