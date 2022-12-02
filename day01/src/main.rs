pub fn main() {
    let mut elf_calories = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf_str| { 
            elf_str
                .split("\n")
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    elf_calories.sort_by(|a, b| b.cmp(a));

    let part1 = elf_calories[0];
    println!("part1: {part1}");

    let part2 = elf_calories[..3].into_iter().sum::<u32>();
    println!("part2: {part2}");
}
