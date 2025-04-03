fn main() {
    println!("Rust library successfully built!");
    println!("The library can now be used by Python code.");
}

/*
📦 Built wheel for CPython 3.11 to /var/folders/rx/yvl26n0d0w74m31czhj6q1g00000gn/T/.tmpMXhdnX/rust_with_python-0.1.0-cp311-cp311-macosx_11_0_arm64.whl
✏️ Setting installed package as editable
🛠 Installed rust_with_python-0.1.0
=== Build process completed ===
Running example Python script:
==== Testing Rust with Python Integration ====

Fibonacci(30):
  Python: 832040 in 0.110027 seconds
  Rust:   832040 in 0.002564 seconds
  Speedup: 42.91x

Original string: Hello, Rust from Python!
Reversed string: !nohtyP morf tsuR ,olleH

Vector addition:
  x: [1. 2. 3. 4. 5.]
  y: [5. 4. 3. 2. 1.]
  x + y: [6. 6. 6. 6. 6.]

Matrix multiplication:
  A:
[[1. 2.]
 [3. 4.]]
  B:
[[5. 6.]
 [7. 8.]]
  NumPy result:
[[19. 22.]
 [43. 50.]]
  Rust result:
[[19. 22.]
 [43. 50.]]
  Equal: True

Point 1: Point(3, 4)
Point 2: Point(7, 1)
Distance from origin (p1): 5.0
Distance between points: 5.0
Modified Point 1: Point(5, 12)
New distance from origin: 13.0

All tests completed!
*/