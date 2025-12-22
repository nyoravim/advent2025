use std::cmp::Ordering;
use std::error::Error;

use advent25::{IdRange, read_input};

fn index_of<T, F: Fn(&T) -> bool>(data: &[T], func: F) -> Option<usize> {
    for i in 0..data.len() {
        if func(&data[i]) {
            return Some(i);
        }
    }

    None
}

struct InputData {
    ranges: Vec<IdRange>,
    available: Vec<u64>,
}

fn parse_input(input: &str) -> Result<InputData, Box<dyn Error>> {
    let lines: Vec<_> = input.trim().lines().collect();

    let Some(index) = index_of(&lines, |l| l.is_empty()) else {
        return Err("No blank line delimiter!".into());
    };

    let (ranges, remainder) = lines.split_at(index);
    let (_, ids) = remainder.split_first().unwrap();

    Ok(InputData {
        ranges: ranges
            .iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?,
        available: ids
            .iter()
            .map(|d| d.parse())
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn is_ingredient_fresh(id: u64, ranges: &[IdRange]) -> bool {
    ranges.iter().any(|range| range.contains(id))
}

fn merge_ranges(ranges: &[IdRange]) -> Option<Vec<IdRange>> {
    let mut globs: Vec<IdRange> = Vec::new();
    for range in ranges {
        if !globs.iter_mut().any(|g| g.merge(range)) {
            globs.push(range.clone());
        }
    }

    match globs.len().cmp(&ranges.len()) {
        Ordering::Less => Some(globs),
        Ordering::Equal => None,
        Ordering::Greater => panic!("dude how the fuck")
    }
}

fn total_possible_fresh(ranges: &[IdRange]) -> u64 {
    let mut globs = Vec::from_iter(ranges.iter().cloned());
    loop {
        let Some(merged) = merge_ranges(&globs) else {
            break;
        };

        globs = merged;
    }

    globs.iter().map(|glob| glob.len()).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(5)?;
    let data = parse_input(&input)?;

    let num_fresh = data
        .available
        .iter()
        .filter(|id| is_ingredient_fresh(**id, &data.ranges))
        .count();

    println!("{num_fresh} fresh ingredients");

    let total_fresh = total_possible_fresh(&data.ranges);
    println!("{total_fresh} total possible fresh ingredients");

    Ok(())
}
