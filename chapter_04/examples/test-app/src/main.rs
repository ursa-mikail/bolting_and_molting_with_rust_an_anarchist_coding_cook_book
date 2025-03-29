// main.rs

mod libs;
mod utils;

use libs::p0::{xello, use_func, sum_vals, NAME};
use utils::utils::{stock_properties, sum_gains_loss, calculate_interest};

fn main() {
    println!("{}", utils::utils::hello());

    println!("hello 9");

    println!("Hello {}", xello());

    //let int_arr = vec![2, 3, 5, 7, 11];
    //let _int_arr = vec![2, 3, 5, 7, 11]; // _name: for future use

    println!("{:?}", std::any::type_name::<Vec<i32>>());
    println!("\n\n{}", NAME);

    use_func(sum_vals, 12, 21);

    let stocks = vec![
        Stock {
            initial_price: 1.93,
            current_price: 3.11,
            funds: 5000.00,
            symbol: String::from("XPP"),
        },
        Stock {
            initial_price: 100.00,
            current_price: 95.00,
            funds: 2000.00,
            symbol: String::from("TSLA"),
        },
        Stock {
            initial_price: 50.00,
            current_price: 60.00,
            funds: 500.00,
            symbol: String::from("MSFT"),
        },
    ];

    let mut total_funds = 0.0;
    let mut total_gains_array = Vec::new();

    for stock in stocks {
        let (total_gains, gain_loss) = stock_properties(
            stock.initial_price,
            stock.current_price,
            stock.funds,
            &stock.symbol,
        );
        total_gains_array.push(total_gains);
        total_funds += stock.funds;

        println!("Stock Symbol: {}", stock.symbol);
        println!("Initial Price: {:.2}", stock.initial_price);
        println!("Current Price: {:.2}", stock.current_price);
        println!("Funds Invested: {:.2}", stock.funds);
        println!("Total Gains: {:.2}", total_gains);
        if gain_loss {
            println!("Status: Gain\n");
        } else {
            println!("Status: Loss\n");
        }
    }

    let total_gains_loss = sum_gains_loss(&total_gains_array);
    println!("Overall Total Gains/Loss: {:.2}", total_gains_loss);

    let (interest, future_value) = calculate_interest(total_funds, 3.0, 0.5);
    println!(
        "Total Current Funds: ${:.2}, Interest: ${:.2}, Future Value: ${:.2}",
        total_funds, interest, future_value
    );
}

#[derive(Debug)]
struct Stock {
    initial_price: f64,
    current_price: f64,
    funds: f64,
    symbol: String,
}

/*
Hello from util
hello 9
Hello Xello from P0
"alloc::vec::Vec<i32>"


P0
Sum: 33
Stock Symbol: XPP
Initial Price: 1.93
Current Price: 3.11
Funds Invested: 5000.00
Total Gains: 3056.99
Status: Gain

Stock Symbol: TSLA
Initial Price: 100.00
Current Price: 95.00
Funds Invested: 2000.00
Total Gains: -100.00
Status: Loss

Stock Symbol: MSFT
Initial Price: 50.00
Current Price: 60.00
Funds Invested: 500.00
Total Gains: 100.00
Status: Gain

Overall Total Gains/Loss: 3056.99
Total Current Funds: $7500.00, Interest: $112.50, Future Value: $7612.50
*/