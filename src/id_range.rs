use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct IdRange {
    first: u64,
    last: u64,
}

pub struct IdIterator {
    id: u64,
    end: u64,
}

impl IdRange {
    pub fn new(first: u64, last: u64) -> Result<Self, &'static str> {
        if first > last {
            Err("First cannot succeed last!")
        } else if first == 0 {
            Err("0 is not an ID!")
        } else {
            Ok(IdRange { first, last })
        }
    }

    pub fn bounds(&self) -> (u64, u64) {
        (self.first, self.last)
    }

    pub fn contains(&self, id: u64) -> bool {
        id >= self.first && id <= self.last
    }

    pub fn ids(&self) -> IdIterator {
        IdIterator {
            id: self.first,
            end: self.last,
        }
    }
}

impl Iterator for IdIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.id > self.end {
            None
        } else {
            let id = self.id;
            self.id += 1;

            Some(id)
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

impl Display for IdRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.first, self.last)
    }
}
