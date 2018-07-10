use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day5() -> Result<(), Box<Error>> {
    let file = File::open("day5.txt")?;
    
    let list: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&list));
    println!("Part 2: {}", part2(&list));
    
    Ok(())
}

fn part1(list: &[i32]) -> u64 {
    let mut list = list.to_vec();

    let len = list.len() as i32;
    let mut i = 0;
    let mut steps = 0;

    while i < len {
        steps += 1;
        let n = &mut list[i as usize];
        i += *n;
        if i < 0 { break; }
        *n += 1;
    }

    steps
}

fn part2(list: &[i32]) -> u64 {
    let mut list = list.to_vec();

    let len = list.len() as i32;
    let mut i = 0;
    let mut steps = 0;

    while i < len {
        steps += 1;
        let n = &mut list[i as usize];
        i += *n;
        if i < 0 { break; }
        if *n >= 3 {
            *n -= 1;
        } else {
            *n += 1;
        }
    }

    steps
}