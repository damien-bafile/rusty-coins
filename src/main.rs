use std::io::{self, Write};

#[derive(Copy, Clone, PartialEq, Eq)]
enum Values {
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

fn get_input() -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn check_input(input: &str) -> Option<u32> {
    // Try to parse input as unsigned int 32
    let val = input.trim().parse::<u32>().ok()?;

    match val {
        5 | 10 | 20 | 50 => Some(val),
        _ => None,
    }
}

fn total_value(values: Vec<Values>) -> u32 {
    values
        .iter()
        .filter(|val| {
            matches!(
                val,
                Values::Five | Values::Ten | Values::Twenty | Values::Fifty
            )
        })
        .map(|val| *val as u32)
        .sum()
}

fn main() {
    println!("Please enter a coin 5, 10, 20, 50. Type \"quit\" to exit.");
    io::stdout().flush().unwrap();

    let mut valuelist: Vec<Values> = Vec::new();
    loop {
        let inputstring = get_input();
        let value: Values = check_input(inputstring);
        match value {
            Values::Five | Values::Ten | Values::Twenty | Values::Fifty => valuelist.push(value),
            Values::Error => println!("Something went wrong!"),
            Values::Quit => {
                println!("Quitting...");
                break;
            }
        }
    }
    println!("Total value: {}", total_value(valuelist));
}
