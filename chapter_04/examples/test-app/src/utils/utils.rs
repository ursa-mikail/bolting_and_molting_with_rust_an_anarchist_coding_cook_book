pub fn hello() -> String {
    "Hello from util".to_string()
}

// Calculate the stock properties
pub fn stock_properties(initial_price: f64, current_price: f64, funds: f64, _symbol: &str) -> (f64, bool) {
    let total_units_purchased = funds / initial_price;
    let total_gains = (current_price - initial_price) * total_units_purchased;
    let gain_loss = total_gains > 0.0;
    (total_gains, gain_loss)
}

// Variadic function to calculate the sum of gains or losses
pub fn sum_gains_loss(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

pub fn calculate_interest(principal: f64, rate: f64, years: f64) -> (f64, f64) {
    let interest = principal * (rate / 100.0) * years;
    let future_value = principal + interest;
    (interest, future_value)
}
