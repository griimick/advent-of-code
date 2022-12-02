// proper way to propagate errors
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut elf_calories = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf_str| { 
            elf_str
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .map(|counts| counts.into_iter().sum())
        })
        .collect::<Result<Vec<u32>,_>>()?;

    elf_calories.sort_by(|a, b| b.cmp(a));

    let part1 = elf_calories[0];
    println!("part1: {part1}");

    let part2 = elf_calories[..3].into_iter().sum::<u32>();
    println!("part2: {part2}");

    Ok(())
}
