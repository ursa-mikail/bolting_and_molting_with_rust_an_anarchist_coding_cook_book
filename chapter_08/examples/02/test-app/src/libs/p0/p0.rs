pub mod p0 {
    pub static NAME: &str = "DefaultName";
    
    pub fn xello() -> String {
        "World".to_string()
    }
    
    pub fn sum_vals(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn use_func<F>(f: F, a: i32, b: i32)
    where
        F: Fn(i32, i32) -> i32,
    {
        let result = f(a, b);
        println!("Result: {}", result);
    }
}