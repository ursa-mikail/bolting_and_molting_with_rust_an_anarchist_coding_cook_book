// src/lib.rs
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_double};

// A simple function that adds two integers
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

// A function that takes a string, converts it to uppercase, and returns it
#[no_mangle]
pub extern "C" fn to_uppercase(input: *const c_char) -> *mut c_char {
    // Convert C string to Rust string
    let c_str = unsafe {
        if input.is_null() {
            return std::ptr::null_mut();
        }
        CStr::from_ptr(input)
    };
    
    // Convert CStr to String and handle any UTF-8 errors
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    
    // Convert to uppercase
    let uppercase = rust_str.to_uppercase();
    
    // Convert back to C string
    let c_result = match CString::new(uppercase) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    };
    
    c_result
}

// Function to free memory allocated by Rust
#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
            // CString is dropped here, freeing the memory
        }
    }
}

// Simple Rust struct that we'll expose to C
#[repr(C)]
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

// Function that works with the struct
#[no_mangle]
pub extern "C" fn distance_from_origin(point: Point) -> c_double {
    let x = point.x as f64;
    let y = point.y as f64;
    (x * x + y * y).sqrt()
}