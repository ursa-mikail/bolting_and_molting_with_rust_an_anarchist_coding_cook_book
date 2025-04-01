// use std::sync::Mutex;

// static mut COUNTER: i32 = 0;

pub unsafe fn dereference_raw_pointer() {
    let x: i32 = 42;
    let r: *const i32 = &x;

    unsafe {
        println!("r points to: {}", *r);
    }

    println!("🔥 What can go wrong? 🚨 Dereferencing Raw Pointers: DIY Segfaults. If x gets freed while r still exists, say hello to Undefined Behavior (UB).")
}

/*
pub unsafe fn modify_static_variable() {
    unsafe {
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}
*/

use std::sync::atomic::{AtomicI32, Ordering};

static COUNTER: AtomicI32 = AtomicI32::new(0);

pub fn modify_static_variable() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Counter: {}", COUNTER.load(Ordering::SeqCst));

    println!("🔥 What can go wrong? Mutable Static Variables: Global Mayhem: If multiple threads access COUNTER without synchronization, you just unlocked a race condition.")
}


unsafe fn do_dangerous_thing() {
    println!("Doing something unsafe!");
    println!("🔥 What can go wrong?  🧨 Calling Unsafe Functions: Invoking the Dark Arts: If this function interacts with raw pointers, FFI, or invalid memory, it can break everything.")
}

pub unsafe fn call_unsafe_function() {
    unsafe {
        do_dangerous_thing();
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

pub unsafe fn call_c_function() {
    unsafe {
        println!("Absolute value of -10 is: {}", abs(-10));
        println!("🔥 What can go wrong? 💣 Interfacing with C (FFI): Out In The Wild. If you call an FFI function incorrectly, expect a hard crash.")

    }
}

#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
}

pub unsafe fn use_union() {
    let u = MyUnion { i: 42 };
    unsafe {
        println!("Union holds: {}", u.i);
        println!("🔥 What can go wrong?  ⚠️ Playing Union Roulette. Accessing the wrong field results in unpredictable behavior.")

    }
}
