use std::{fmt, env, error, num, result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Formatter;
use std::num::ParseIntError;

const TARGET: i64 = 2020;

#[derive(fmt::Debug)]
struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Error {
        Error{message}
    }
}

type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error{}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::new(format!("io error:{}", e))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::new(format!("parse int error:{}", e))
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lines_to_ints(lines: impl Iterator<Item=io::Result<String>>) -> Result<Vec<i64>> {
    let mut result = Vec::new();
    for line in lines {
        result.push(line?.parse::<i64>()?);
    }
    Ok(result)
}

fn find_result(ints: &mut Vec<i64>) -> Result<i64> {
    ints.sort_unstable();
    for i in ints.iter() {
        let rem = TARGET - i;
        if ints.binary_search(&rem).is_ok() {
            return Ok(i * rem);
        }
    }
    Err(Error::new("no result".to_string()))
}


fn main() -> Result<()>{
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let mut ints = lines_to_ints(read_lines(&args[1])?)?;
        let result = find_result(&mut ints)?;
        println!("The result is {}", result);
        Ok(())
    } else {
        Err(Error::new("filename argument required".to_string()))
    }

}


