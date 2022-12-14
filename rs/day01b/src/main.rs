pub fn main() {
        let mut temp = include_str!("../input.txt")
        .split("\n\n")
        .into_iter()
        .map(|elf_str| { 
                elf_str
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .iter()
                    .filter(|s| !s.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>()
                    .iter()
                    .sum()
            })
        .collect::<Vec<u32>>();

    temp.sort();
    
    println!(
        "{:?}",
        temp.iter().rev().take(3).sum::<u32>()
    );
}
