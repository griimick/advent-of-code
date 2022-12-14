// proper way to propagate errors
use lazy_static::lazy_static;
use std::error::Error;
use regex::Regex;
use std::str;

lazy_static! {
    static ref MOVE_REG: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    static ref BOX_REG: Regex = Regex::new(r"\[(\w+)\]").unwrap();
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let strings =  include_str!("../input.txt") // iykyk :)
        .split("\n\n")
        .collect::<Vec<&str>>();
    let stack_lines = strings[0].split('\n').filter(|line| !line.is_empty());
    let moves_lines = strings[1].split('\n').filter(|line| !line.is_empty());
    let stacks_count: i32 = stack_lines.clone()
        .last().unwrap()
        .trim().split("   ")
        .last().unwrap()
        .parse::<i32>()?;
    let max_stack_height: i32 = stack_lines.clone().count() as i32 - 1; // except the indexes
    
    let stacks_str: Vec<Vec<&str>> = stack_lines
        .rev()
        .skip(1)
        .map(|line| {
            return line.as_bytes()
                .chunks(4) //`[x] `
                .map(|buf| str::from_utf8(buf).unwrap())
                .collect::<Vec<&str>>();
        })
        .collect();

    let height = max_stack_height as usize + 1;
    let width = stacks_count as usize + 1;

    let mut stack: Vec<Vec<&str>> = Vec::with_capacity(height);

    for _ in 0..height {
        stack.push(Vec::with_capacity(width));
    }

    for i in 1..height {
        for j in 1..width { 
            // println!("{}", stacks_str[i-1][j-1]);
            let cap = BOX_REG.captures(stacks_str[i-1][j-1]);
            if !cap.is_none() {
                let name = cap.unwrap().get(1).map_or("", |m| m.as_str());
                stack[j-1].push(name);
            }
        }
    }

    let moves: Vec<Vec<usize>> = moves_lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps =  MOVE_REG.captures(line.trim()).unwrap();
            let count = caps.get(1).map_or("", |m| m.as_str());
            let from = caps.get(2).map_or("", |m| m.as_str());
            let to = caps.get(3).map_or("", |m| m.as_str());

            return Vec::from([
                count.parse::<usize>().unwrap(), 
                from.parse::<usize>().unwrap(), 
                to.parse::<usize>().unwrap(), 
            ]);
        })
        .collect::<Vec<Vec<usize>>>();

    // println!("{:?}", stack);

    for mov in moves.iter() {
        let count = mov[0];
        let from = mov[1];
        let to = mov[2];
        // println!("{} {} {}", count, from, to);
        if stack[from - 1].len() >= count {
            let offset = stack[from - 1].len() - count;
            let mut u: Vec<&str> = stack[from - 1].drain(offset..).collect();
            stack[to - 1].append(&mut u);
            // println!("{:?}", stack);
        }
    }

    let mut ans: String = "".to_owned();

    for s in stack.iter() {
        if s.last().is_some() {
           ans.push_str(s.last().unwrap()); 
        }
    }

    println!("{:?}", ans);
        
    Ok(())
}
