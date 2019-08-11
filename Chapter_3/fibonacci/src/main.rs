use std::io;

fn main() {
    loop {
        println!("Enter the Fibonacci sequence term index:");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: u128 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 0 {
            println!("Result: 0");
        } else if input == 1 || input == 2 {
            println!("Result: 1");
        } else if input > 2 {
            let result = fibonacci(input);
            println!("Result: {}", result);
        } else {
            println!("Invalid input given!");
        }
    }
}

fn fibonacci(n: u128) -> u128 {
    let mut term_0: u128 = 1;
    let mut term_1: u128 = 1;
    let mut result: u128 = 0;
    let mut iteration = 0;

    while iteration < n - 2 {
        result = term_0 + term_1;
        term_0 = term_1;
        term_1 = result;
        iteration += 1;
    }

    result
}
