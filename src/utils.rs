use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<BufReader<File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
