use std::error::Error;

use advent25::{read_input, IdRange};

fn find_pattern(id: u64, factor: u64) -> Option<u64> {
    let mut value = id % factor;
    let mut remainder = id / factor;

    loop {
        // no more data to process
        if remainder == 0 {
            return Some(value);
        }

        // advance in id and get next value
        let next = remainder % factor;

        // if the next value does not equal, then there is no pattern
        if value != next {
            return None;
        }

        // advance
        value = next;
        remainder /= factor;
    }
}

fn is_invalid_id(id: u64, base: u64) -> bool {
    let mut factor = base;
    loop {
        if factor > id {
            return false; // no point
        }

        if let Some(pattern) = find_pattern(id, factor) {
            let previous_factor = factor / base;
            if pattern >= previous_factor {
                // no leading zero, we're good
                return true;
            }
        }

        factor *= base;
    }
}

fn sum_invalid(range: &IdRange, base: u64) -> u64 {
    let mut sum = 0;
    let mut num_invalid = 0;

    println!("Range {range}");
    for id in range.ids() {
        if !is_invalid_id(id, base) {
            continue;
        }

        println!("Invalid ID: {id}");

        sum += id;
        num_invalid += 1;
    }

    println!("Sum for range: {sum}");
    println!("{num_invalid} invalid ID(s)");

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(2)?;

    let ranges = input
        .trim()
        .split(",")
        .map(|r| r.parse())
        .collect::<Result<Vec<IdRange>, _>>()?;

    let mut accumulated: u64 = 0;
    for range in ranges {
        accumulated += sum_invalid(&range, 10);
    }

    println!("Accumulated invalid IDs: {accumulated}");
    Ok(())
}
