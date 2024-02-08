use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn read_day_input_lines(day: u8) -> io::Result<io::Lines<io::BufReader<File>>>{
    read_lines(format!("/home/nathan/PycharmProjects/rust_of_code/puzzle_inputs/day_{day}_input.txt"))
}
