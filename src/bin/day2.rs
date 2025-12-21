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

fn sum_invalid(range: &IdRange, base: u64) -> u64 {
    let mut sum = 0;
    let mut num_invalid = 0;

    println!("Range [{},{}]", range.first, range.last);
    let mut factor = base;

    for id in range.first..=range.last {
        let mut current_factor = factor;
        loop {
            let lesser = id % current_factor;
            let greater = id / current_factor;

            match lesser.cmp(&greater) {
                Ordering::Less => (),
                Ordering::Greater => break,
                Ordering::Equal => {
                    let previous_factor = current_factor / base;
                    if lesser >= previous_factor {
                        println!("Invalid ID found: {id}");

                        // confusing: update the "factor" to the "current factor"
                        // ids only increase
                        // this is to improve efficiency and not do extra work
                        factor = current_factor;

                        sum += id;
                        num_invalid += 1;

                        break;
                    }

                    // otherwise, leading zero. we dont care
                }
            };

            current_factor *= base;
        }
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
