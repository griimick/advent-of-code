use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut bucket: i32 = 0;
    let mut heap = BinaryHeap::new();

    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            heap.push(Reverse(bucket));
            bucket = 0;
            if heap.len() > 3 {
                heap.pop();
            }
        } else {
            let num: i32 = line?.parse().unwrap();
            bucket += num;
        }
    }

    heap.push(Reverse(bucket));

    let mut sum = 0;
    while !heap.is_empty()  {
        if let Some(Reverse(v)) = heap.pop() {
            sum += v;
        }
    }

    println!("{}", sum.to_string());

    Ok(())
}
