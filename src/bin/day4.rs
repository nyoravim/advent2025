use std::error::Error;
use std::num::NonZeroUsize;

use advent25::read_input;

use bitvector::BitVector;

struct Map {
    data: BitVector,
    rows: NonZeroUsize,
    columns: NonZeroUsize,
}

fn parse_map(input: &str) -> Result<Map, &'static str> {
    let data: Vec<_> = input.trim().lines().map(|l| l.trim()).collect();

    if data.is_empty() || data[0].is_empty() {
        Err("No data!")
    } else {
        let rows = data.len();
        let columns = data[0].len();
        let mut map = BitVector::new(columns * rows);

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

                map.insert(x + y * columns);
            }
        }

        Ok(Map {
            data: map,
            rows: NonZeroUsize::new(rows).unwrap(),
            columns: NonZeroUsize::new(columns).unwrap(),
        })
    }
}

struct MapIter {
    index: usize,
    rows: usize,
    columns: usize,
}

impl Map {
    fn index_from_pos(&self, x: usize, y: usize) -> Option<usize> {
        let columns = self.columns.get();
        let rows = self.rows.get();

        if x >= columns || y >= rows {
            None
        } else {
            Some(x + y * columns)
        }
    }

    fn exists(&self, x: usize, y: usize) -> bool {
        self.index_from_pos(x, y)
            .map_or(false, |i| self.data.contains(i))
    }

    fn coordinates(&self) -> MapIter {
        MapIter {
            index: 0,
            rows: self.rows.get(),
            columns: self.columns.get(),
        }
    }
}

impl Iterator for MapIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.rows * self.columns;
        if self.index >= count {
            None
        } else {
            let x = self.index % self.columns;
            let y = self.index / self.columns;

            self.index += 1;
            Some((x, y))
        }
    }
}

fn exists(map: &Map, x: usize, y: usize, delta: &(isize, isize)) -> bool {
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

fn num_adjacent(map: &Map, x: usize, y: usize) -> usize {
    (0..9)
        .filter_map(|i| parse_delta_index(i))
        .filter(|delta| exists(map, x, y, delta))
        .count()
}

fn accessible(map: &Map, x: usize, y: usize) -> bool {
    if map.exists(x, y) {
        let adj = num_adjacent(map, x, y);
        adj < 4
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input(4)?;
    let map = parse_map(&input)?;

    let num_accessible = map.coordinates().filter(|(x, y)| accessible(&map, *x, *y)).count();
    println!("Num accessible: {num_accessible}");

    Ok(())
}
