use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut highest: i32 = 0;
    let mut bucket: i32 = 0;

    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            highest = cmp::max(highest, bucket);
            bucket = 0;
        } else {
            let num: i32 = line?.parse().unwrap();
            bucket += num;
        }
    }

    highest = cmp::max(highest, bucket);
    println!("{}", highest.to_string());

    Ok(())
}
