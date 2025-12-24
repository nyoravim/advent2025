use std::error::Error;

use advent25::{BitMap, read_input};

fn parse_map(input: &str) -> Result<BitMap, &'static str> {
    let data: Vec<_> = input.trim().lines().map(|l| l.trim()).collect();

    if data.is_empty() || data[0].is_empty() {
        Err("No data!")
    } else {
        let rows = data.len();
        let columns = data[0].len();
        let mut map = BitMap::new(columns, rows);

        for y in 0..rows {
            let row: Vec<char> = data[y].chars().collect();
            let row_length = row.len();

            if row_length != columns {
                return Err("Column mismatch!");
            }

            for x in 0..row_length {
                if row[x] != '@' {
                    continue;
                }

                map.add(x, y);
            }
        }

        Ok(map)
    }
}

fn exists(map: &BitMap, x: usize, y: usize, delta: &(isize, isize)) -> bool {
    let (dx, dy) = delta;
    let Some(x_f) = x.checked_add_signed(*dx) else {
        return false;
    };

    let Some(y_f) = y.checked_add_signed(*dy) else {
        return false;
    };

    map.exists(x_f, y_f)
}

fn parse_delta_index(i: usize) -> Option<(isize, isize)> {
    let x = i % 3;
    let y = i / 3;

    let dx = (x as isize) - 1;
    let dy = (y as isize) - 1;

    if dx == 0 && dy == 0 {
        None
    } else {
        Some((dx, dy))
    }
}

fn num_adjacent(map: &BitMap, x: usize, y: usize) -> usize {
    (0..9)
        .filter_map(|i| parse_delta_index(i))
        .filter(|delta| exists(map, x, y, delta))
        .count()
}

fn accessible(map: &BitMap, x: usize, y: usize) -> bool {
    if map.exists(x, y) {
        let adj = num_adjacent(map, x, y);
        adj < 4
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(4)?;
    let mut map = parse_map(&input)?;

    let mut iteration = 0;
    let mut removed_count = 0;

    loop {
        iteration += 1;
        println!("Iteration #{iteration}");

        let accessible_rolls: Vec<_> = map
            .coordinates()
            .filter(|(x, y)| accessible(&map, *x, *y))
            .collect();

        let num_accessible = accessible_rolls.len();
        println!("Num accessible: {num_accessible}");

        if accessible_rolls.is_empty() {
            println!("Done");
            break;
        }

        for (x, y) in accessible_rolls {
            if !map.remove(x, y) {
                return Err("Something goofy happened - failed to remove roll".into());
            }
        }

        removed_count += num_accessible;
        println!("All accessible rolls removed!");
    }

    println!("Total removed: {removed_count}");
    println!("Num remaining: {}", map.num_entities());

    Ok(())
}
