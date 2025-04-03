import time
import numpy as np
import rust_with_python as rp

def test_fibonacci():
    """Test and compare Fibonacci implementation"""
    
    # Python implementation for comparison
    def py_fibonacci(n):
        if n <= 0:
            return 0
        elif n == 1:
            return 1
        else:
            return py_fibonacci(n - 1) + py_fibonacci(n - 2)
    
    # Test for a small number
    n = 30
    
    # Time the Python implementation
    start = time.time()
    py_result = py_fibonacci(n)
    py_time = time.time() - start
    
    # Time the Rust implementation
    start = time.time()
    rust_result = rp.fibonacci(n)
    rust_time = time.time() - start
    
    print(f"Fibonacci({n}):")
    print(f"  Python: {py_result} in {py_time:.6f} seconds")
    print(f"  Rust:   {rust_result} in {rust_time:.6f} seconds")
    print(f"  Speedup: {py_time/rust_time:.2f}x\n")

def test_string_ops():
    """Test string operations"""
    test_string = "Hello, Rust from Python!"
    
    result = rp.reverse_string(test_string)
    print(f"Original string: {test_string}")
    print(f"Reversed string: {result}\n")

def test_numpy_operations():
    """Test NumPy array operations"""
    # Create some test arrays
    x = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
    y = np.array([5.0, 4.0, 3.0, 2.0, 1.0])
    
    # Vector addition
    result = rp.add_vectors(x, y)
    print("Vector addition:")
    print(f"  x: {x}")
    print(f"  y: {y}")
    print(f"  x + y: {result}\n")
    
    # Matrix multiplication
    a = np.array([[1.0, 2.0], [3.0, 4.0]])
    b = np.array([[5.0, 6.0], [7.0, 8.0]])
    
    # NumPy result for comparison
    numpy_result = np.matmul(a, b)
    
    # Rust result
    rust_result = rp.matrix_multiply(a, b)
    
    print("Matrix multiplication:")
    print(f"  A:\n{a}")
    print(f"  B:\n{b}")
    print(f"  NumPy result:\n{numpy_result}")
    print(f"  Rust result:\n{rust_result}")
    print(f"  Equal: {np.allclose(numpy_result, rust_result)}\n")

def test_custom_class():
    """Test custom Rust class in Python"""
    # Create two points
    p1 = rp.Point(3.0, 4.0)
    p2 = rp.Point(7.0, 1.0)
    
    print(f"Point 1: {p1}")
    print(f"Point 2: {p2}")
    print(f"Distance from origin (p1): {p1.distance_from_origin()}")
    print(f"Distance between points: {p1.distance_to(p2)}")
    
    # Modify a point
    p1.x = 5.0
    p1.y = 12.0
    
    print(f"Modified Point 1: {p1}")
    print(f"New distance from origin: {p1.distance_from_origin()}\n")

if __name__ == "__main__":
    print("==== Testing Rust with Python Integration ====\n")
    
    test_fibonacci()
    test_string_ops()
    test_numpy_operations()
    test_custom_class()
    
    print("All tests completed!")