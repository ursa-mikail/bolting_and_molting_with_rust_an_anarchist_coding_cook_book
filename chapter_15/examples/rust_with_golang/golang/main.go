package main

// #cgo LDFLAGS: -L../rust/target/release -lrust_with_golang
// #include <stdlib.h>
// int add(int a, int b);
// char* to_uppercase(const char* input);
// void free_rust_string(char* s);
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	// Example 1: Call the add function
	result := C.add(5, 7)
	fmt.Printf("5 + 7 = %d\n", result)
	
	// Example 2: Call the to_uppercase function
	inputStr := "Hello, Rust from Go!"
	cstr := C.CString(inputStr)
	defer C.free(unsafe.Pointer(cstr))
	
	// Call the Rust function
	rustResult := C.to_uppercase(cstr)
	
	// Convert result back to Go string
	goResult := C.GoString(rustResult)
	
	// Free the memory allocated by Rust
	C.free_rust_string(rustResult)
	
	fmt.Printf("Original: %s\nUppercase: %s\n", inputStr, goResult)
}