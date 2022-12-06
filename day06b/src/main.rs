use std::error::Error;
use std::collections::HashMap;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", include_str!("../input.txt") // iykyk :)
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| get_marker_index(line))
        .collect::<Vec<i32>>(),
    );

    Ok(())
}

pub fn get_marker_index(line: &str) -> i32 {
    let mut counter: i32 = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut i: usize = 0;

    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        if map.get(&c).is_some() {
            i = map.get(&c).unwrap() + 1;
            counter = 0;
            map.clear();
            continue;
        }
        map.insert(c, i);
        counter += 1;
        if counter == 14 {
            return i as i32 + 1;
        }
        i += 1;
    }
    if counter == 14 {
        return line.len() as i32;
    } else {
        return -1;

    }
}
