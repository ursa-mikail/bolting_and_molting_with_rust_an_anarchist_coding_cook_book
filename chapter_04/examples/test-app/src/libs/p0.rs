// libs/p0.rs

pub const NAME: &str = "P0";

pub fn xello() -> String {
    "Xello from P0".to_string()
}

pub fn use_func<F>(func: F, a: i32, b: i32)
where
    F: Fn(i32, i32),
{
    func(a, b);
}

pub fn sum_vals(a: i32, b: i32) {
    println!("Sum: {}", a + b);
}
