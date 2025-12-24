use std::error::Error;
use std::mem;

use advent25::{BitMap, read_input};
use bitvector::BitVector;

struct Manifold {
    splitters: BitMap,
    entrypoint: usize,
}

fn parse_manifold(input: &str) -> Result<Manifold, Box<dyn Error>> {
    let data: Vec<_> = input.trim().lines().map(|line| line.trim()).collect();

    let Some((header, body)) = data.split_first() else {
        return Err("No data provided!".into());
    };

    let Some(entrypoint) = header.chars().position(|c| c == 'S') else {
        return Err("No beam entrypoint specified in input!".into());
    };

    let width = header.len();
    let height = body.len();

    let mut splitters = BitMap::new(width, height);
    for y in 0..height {
        let row = &body[y];
        for (x, character) in row.char_indices() {
            if character != '^' {
                continue;
            }

            splitters.add(x, y);
        }
    }

    Ok(Manifold {
        splitters,
        entrypoint,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(7)?;
    let manifold = parse_manifold(&input)?;

    let (width, height) = manifold.splitters.size();
    let mut working = BitVector::new(width);

    let mut backbuffer = working.clone();
    backbuffer.insert(manifold.entrypoint);

    let mut split_count = 0;
    for y in 0..height {
        // iterating through beam positions in previous layer
        for x in &backbuffer {
            if manifold.splitters.exists(x, y) {
                // split

                if x == 0 || x == width - 1 {
                    return Err("Attempted to split off the side of the manifold!".into());
                }

                working.insert(x - 1);
                working.insert(x + 1);

                split_count += 1;
            } else {
                working.insert(x);
            }
        }

        mem::swap(&mut working, &mut backbuffer);
        working.clear();
    }

    println!("Beam split {split_count} time(s)");
    Ok(())
}
