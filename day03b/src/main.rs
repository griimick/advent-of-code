// proper way to propagate errors
use std::error::Error;
use std::collections::HashSet;

fn get_priorities(group: &[&str]) -> i32{
    
    let (shortest, shorter, short) = (group[0], group[1], group[2]);

    let set1: HashSet<char> = shortest.chars().collect();
    let set2: HashSet<char> = shorter.chars().collect();

    let mut wrongs = short.chars().filter(|c| set1.contains(&c) && set2.contains(&c) ).collect::<Vec<char>>();
    wrongs.sort();
    wrongs.dedup();

    wrongs.iter().map(|&c| get_score(c)).sum()
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", include_str!("../input.txt")
        .split("\n")
        .filter(|line_str| !line_str.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| { 
            get_priorities(group)
        })
        .into_iter()
        .sum::<i32>(),
    );

    Ok(())
}

pub fn get_score(c: char) -> i32 {
    if c <= 'Z' {
        c as i32 - 'A' as i32 + 27
    } else {
        c as i32 - 'a' as i32 + 1
    }
}
