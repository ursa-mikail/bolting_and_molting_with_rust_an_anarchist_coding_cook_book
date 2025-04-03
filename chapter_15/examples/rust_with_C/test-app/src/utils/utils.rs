use std::ffi::{CString, c_char, c_int};

// Declare C functions inside an `unsafe` block
unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
    fn puts(s: *const c_char);
}

// Wrapper function for safe Rust calls
pub fn call_c_functions() {
    unsafe {
        let num = -42;
        println!("Absolute value of {} is {}", num, abs(num));

        let message = CString::new("Hello from Rust!").expect("CString::new failed");
        puts(message.as_ptr());
    }
}
