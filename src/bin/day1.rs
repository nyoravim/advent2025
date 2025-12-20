use std::error::Error;
use std::io::Read;

use advent25::open_input;

const POSITION_COUNT: u32 = 100;

enum Direction {
    Left,
    Right,
}

struct Rotation {
    dir: Direction,
    measure: u32,
}

impl Rotation {
    fn to_increment(&self) -> i32 {
        let factor = match &self.dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        let measure = self.measure;
        factor * measure as i32
    }
}

struct AppliedRotation {
    new_pos: u32,
    zero_count: u32,
}

fn apply_rotation(pos: u32, rotation: &Rotation) -> AppliedRotation {
    let increment = rotation.to_increment();
    let uncorrected = pos as i32 + increment;

    let corrected = uncorrected.rem_euclid(POSITION_COUNT as i32);
    let zero_count = (uncorrected - corrected) / POSITION_COUNT as i32;

    AppliedRotation {
        new_pos: corrected as u32,
        zero_count: zero_count.abs() as u32,
    }
}

fn parse_rotation(data: &str) -> Result<Rotation, Box<dyn Error>> {
    let (dir, measure) = data.split_at(1);
    Ok(Rotation {
        dir: match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err("Invalid direction!".into()),
        },
        measure: measure.parse()?,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = open_input(1)?;

    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let mut pos = 50;
    let mut count = 0;

    println!("Starting dial at {pos}");
    for line in contents.lines().map(|l| l.trim()) {
        let rotation = parse_rotation(line)?;

        let applied = apply_rotation(pos, &rotation);
        pos = applied.new_pos;

        println!("Rotation {line} moved dial to {pos}");

        if applied.zero_count > 0 {
            count += applied.zero_count;
            println!("Incrementing count to {count}!");
        }
    }

    println!("Final count: {count}");
    Ok(())
}
