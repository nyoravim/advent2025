use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum AdventError {
    InvalidDay(u32),
    NoInputDir(PathBuf),
    NoInputFile(PathBuf),
}

impl Display for AdventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDay(day) => write!(f, "Invalid day index: {day}"),
            Self::NoInputDir(path) => write!(f, "No such input directory: {}", path.display()),
            Self::NoInputFile(path) => write!(f, "No such input data: {}", path.display()),
        }
    }
}

impl Error for AdventError {}

fn get_day_input_path(day: u32) -> Result<PathBuf, AdventError> {
    if day < 1 || day > 31 {
        return Err(AdventError::InvalidDay(day));
    }

    let dir = Path::new("./input");
    if !dir.is_dir() {
        return Err(AdventError::NoInputDir(dir.to_path_buf()));
    }

    let file = dir.join(Path::new(&format!("{day}.txt"))).to_path_buf();
    if file.is_file() {
        Ok(file)
    } else {
        Err(AdventError::NoInputFile(file))
    }
}

pub fn open_input(day: u32) -> Result<File, Box<dyn Error>> {
    let file_path = get_day_input_path(day)?;
    let file = File::open(file_path)?;

    Ok(file)
}
