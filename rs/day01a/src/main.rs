pub fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
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
        .collect::<Vec<u32>>()
        .iter()
        .max()
        .unwrap(),
    );
}
