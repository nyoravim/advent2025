use std::error::Error;

use advent25::read_input;

fn parse_joltage(c: char) -> Result<u8, &'static str> {
    match c.to_digit(10) {
        Some(d) => Ok(d as u8),
        None => Err("Invalid digit!"),
    }
}

fn parse_bank(line: &str) -> Result<Vec<u8>, &'static str> {
    line.trim().chars().map(|c| parse_joltage(c)).collect()
}

struct MaxValue<'a, T> {
    value: &'a T,
    index: usize,
}

fn is_greater<T: Ord>(lhs: &T, rhs: &Option<MaxValue<T>>) -> bool {
    if let Some(max) = rhs {
        lhs > &max.value
    } else {
        true
    }
}

fn find_max<'a, T: Ord>(data: &'a [T]) -> Option<MaxValue<'a, T>> {
    let mut result = None;
    for i in 0..data.len() {
        let value = &data[i];
        if is_greater(value, &result) {
            result = Some(MaxValue {
                value: value,
                index: i,
            });
        }
    }

    result
}

fn find_max_joltage(line: &str, digits: usize) -> Result<u64, Box<dyn Error>> {
    let mut joltages: &[u8] = &parse_bank(line)?;

    let mut sum = 0;
    for i in 0..digits {
        let num_available = joltages.len() + i - (digits - 1);
        let available_batteries = &joltages[0..num_available];

        println!("Num available: {num_available}");
        let Some(max) = find_max(available_batteries) else {
            return Err("Not enough batteries in bank!".into());
        };

        sum *= 10;
        sum += *max.value as u64;

        joltages = &joltages[(max.index + 1)..];
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(3)?;

    let mut sum = 0;
    for line in input.trim().lines() {
        println!("{line}");
        let max_joltage = find_max_joltage(line, 12)?;

        println!("Max joltage: {max_joltage}");
        sum += max_joltage as u64;
    }

    println!("Sum: {sum}");
    Ok(())
}
