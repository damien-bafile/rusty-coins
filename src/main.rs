use std::io;

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn check_quit(input: &str) -> bool {
    input.trim().eq_ignore_ascii_case("quit")
}

enum CheckInputError {
    NotANumber,
    OutOfRange,
    NotDivisibleBy5,
}

fn check_input(input: &str) -> Result<u32, CheckInputError> {
    let trimmed = input.trim();
    let val = trimmed
        .parse::<u32>()
        .map_err(|_| CheckInputError::NotANumber)?;
    if !(5..=95).contains(&val) {
        return Err(CheckInputError::OutOfRange);
    }
    if val % 5 != 0 {
        return Err(CheckInputError::NotDivisibleBy5);
    }
    Ok(val)
}

fn calculate_coins(mut value: u32) -> Vec<u32> {
    const COINTYPES: [u32; 4] = [50, 20, 10, 5];
    COINTYPES
        .iter()
        .map(|&coin| {
            let count = value / coin;
            value -= count * coin;
            count
        })
        .collect()
}

fn main() {
    println!("Please enter a coin >=5c || <= 95c. Type \"quit\" to exit.");

    loop {
        let input = get_input();
        if check_quit(&input) {
            println!("Exiting the program.");
            break;
        }
        match check_input(&input) {
            Ok(value) => {
                let coins = calculate_coins(value);
                println!("You entered: {value}");
                println!(
                    "Coins needed: 50c: {}, 20c: {}, 10c: {}, 5c: {}",
                    coins[0], coins[1], coins[2], coins[3]
                );
            }
            Err(e) => match e {
                CheckInputError::NotANumber => {
                    println!("Error: Input is not a number.");
                }
                CheckInputError::OutOfRange => {
                    println!("Error: Input must be between 5c and 95c.");
                }
                CheckInputError::NotDivisibleBy5 => {
                    println!("Error: Input must be divisible by 5.");
                }
            },
        }
    }
}
