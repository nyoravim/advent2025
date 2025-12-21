use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

use advent25::read_input;

struct IdRange {
    first: u64,
    last: u64,
}

impl IdRange {
    fn new(first: u64, last: u64) -> Result<Self, &'static str> {
        if first > last {
            Err("First cannot succeed last!")
        } else if first == 0 {
            Err("0 is not an ID!")
        } else {
            Ok(IdRange { first, last })
        }
    }
}

impl FromStr for IdRange {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(pos) = s.find("-") {
            let (first_str, remainder) = s.split_at(pos);
            let (_, last_str) = remainder.split_at(1);

            let range = Self::new(first_str.parse()?, last_str.parse()?)?;
            Ok(range)
        } else {
            Err("No range delimiter!".into())
        }
    }
}

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

    println!("Range [{},{}]", range.first, range.last);
    for id in range.first..=range.last {
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
