// proper way to propagate errors
use std::error::Error;
use std::collections::HashSet;

fn get_priorities(a: &str, b: &str) -> i32{
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };

    let set: HashSet<char> = shorter.chars().collect();
    let mut wrongs = longer.chars().filter(|c| set.contains(&c)).collect::<Vec<char>>();
    wrongs.sort();
    wrongs.dedup();

    wrongs.iter().map(|&c| get_score(c)).sum()
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", include_str!("../input.txt")
        .split("\n")
        .filter(|line_str| !line_str.is_empty())
        .map(|line_str| { 
            let (part1, part2) = line_str
            .split_at(line_str.len() / 2);
            get_priorities(part1, part2)
        })
        .into_iter()
        .sum::<i32>(),
    );

    Ok(())
}

pub fn get_score(c: char) -> i32 {
    let score: i32;
    if c <= 'Z' {
        score = c as i32 - 'A' as i32 + 27; 
    } else {
        score = c as i32 - 'a' as i32 + 1;
    }
    return score;
}
