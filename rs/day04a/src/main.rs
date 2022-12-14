
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", include_str!("../input.txt")
        .split("\n")
        .filter(|line_str| !line_str.is_empty())
        .map(|line_str| is_overlapping(line_str))
        .sum::<i32>(),
    );

    Ok(())
}

pub fn is_overlapping(line: &str) -> i32 {

    let pairs = line
        .split(",")
        .map(|each| { each
            .split('-')
            .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let mut overlap = 0;
    let a_start : i32 = pairs[0][0].parse().unwrap();
    let a_end : i32 = pairs[0][1].parse().unwrap();
    let b_start : i32 = pairs[1][0].parse().unwrap();
    let b_end : i32 = pairs[1][1].parse().unwrap();

    if a_start >= b_start && a_end <= b_end {
        overlap = 1;
    } else if b_start >= a_start && b_end <= a_end {
        overlap = 1;
    }

    return overlap;

}
