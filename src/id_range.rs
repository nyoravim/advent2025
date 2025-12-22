use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
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

    pub fn len(&self) -> u64 {
        self.last - self.first + 1
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

    pub fn adjacent(&self, other: &IdRange) -> bool {
        self.last + 1 >= other.first && other.last + 1 >= self.first
    }

    pub fn or(&self, other: &IdRange) -> Option<IdRange> {
        if self.adjacent(other) {
            Some(IdRange {
                first: self.first.min(other.first),
                last: self.last.max(other.last),
            })
        } else {
            None
        }
    }

    pub fn merge(&mut self, other: &IdRange) -> bool {
        if self.adjacent(other) {
            self.first = self.first.min(other.first);
            self.last = self.last.max(other.last);

            true
        } else {
            false
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
