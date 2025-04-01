mod utils;

fn main() {
    unsafe {
        utils::dereference_raw_pointer();
        utils::modify_static_variable();
        utils::modify_static_variable();

        utils::call_unsafe_function();
        utils::call_c_function();
        utils::use_union();
    }
}

/*

r points to: 42
🔥 What can go wrong? 🚨 Dereferencing Raw Pointers: DIY Segfaults. If x gets freed while r still exists, say hello to Undefined Behavior (UB).
Counter: 1
🔥 What can go wrong? Mutable Static Variables: Global Mayhem: If multiple threads access COUNTER without synchronization, you just unlocked a race condition.
Counter: 2
🔥 What can go wrong? Mutable Static Variables: Global Mayhem: If multiple threads access COUNTER without synchronization, you just unlocked a race condition.
Doing something unsafe!
🔥 What can go wrong?  🧨 Calling Unsafe Functions: Invoking the Dark Arts: If this function interacts with raw pointers, FFI, or invalid memory, it can break everything.
Absolute value of -10 is: 10
🔥 What can go wrong? 💣 Interfacing with C (FFI): Out In The Wild. If you call an FFI function incorrectly, expect a hard crash.
Union holds: 42
🔥 What can go wrong?  ⚠️ Playing Union Roulette. Accessing the wrong field results in unpredictable behavior.


*/