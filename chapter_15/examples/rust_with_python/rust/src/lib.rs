use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::{PyArray1, PyArray2, IntoPyArray};
use ndarray::Array2;

// Basic function example: Fibonacci calculation
#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// String processing example
#[pyfunction]
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// Numpy array processing example: vector addition
#[pyfunction]
fn add_vectors(py: Python, x: &PyArray1<f64>, y: &PyArray1<f64>) -> Py<PyArray1<f64>> {
    let x = unsafe { x.as_array() };
    let y = unsafe { y.as_array() };
    
    // Create a new array for the result
    let result = &x + &y;
    
    // Convert back to Python object
    result.into_pyarray(py).to_owned()
}

// Numpy array processing example: matrix multiplication
#[pyfunction]
fn matrix_multiply(py: Python, a: &PyArray2<f64>, b: &PyArray2<f64>) -> Py<PyArray2<f64>> {
    let a_array = unsafe { a.as_array() };
    let b_array = unsafe { b.as_array() };
    
    // Simple matrix multiplication
    let mut result = Array2::<f64>::zeros((a_array.shape()[0], b_array.shape()[1]));
    
    for i in 0..a_array.shape()[0] {
        for j in 0..b_array.shape()[1] {
            let mut sum = 0.0;
            for k in 0..a_array.shape()[1] {
                sum += a_array[[i, k]] * b_array[[k, j]];
            }
            result[[i, j]] = sum;
        }
    }
    
    // Convert back to Python object
    result.into_pyarray(py).to_owned()
}

// Custom struct example
#[pyclass]
struct Point {
    #[pyo3(get, set)]
    x: f64,
    
    #[pyo3(get, set)]
    y: f64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
    
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Point({}, {})", self.x, self.y))
    }
}

// Module definition - THIS IS THE KEY PART
#[pymodule]
fn rust_with_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;
    m.add_function(wrap_pyfunction!(add_vectors, m)?)?;
    m.add_function(wrap_pyfunction!(matrix_multiply, m)?)?;
    m.add_class::<Point>()?;
    
    Ok(())
}