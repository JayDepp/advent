use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2() -> Result<(), Box<Error>> {
    // Input
    let file = File::open("day2.txt")?;
    
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    // Parse
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line|
            line.split('\t')
                .map(|n| n.parse().unwrap())
                .collect()
        )
        .collect();
    
    // Part 1
    let ans: u32 = grid
        .iter()
        .map(|row|
            row.iter().max().unwrap() - row.iter().min().unwrap()
        )
        .sum();
    
    println!("Part 1: {}", ans);

    // Part 2
    let ans: u32 = grid
        .iter()
        .map(|row| find_even_div(row))
        .sum();

    println!("Part 2: {}", ans);

    Ok(())
}

fn find_even_div(row: &[u32]) -> u32 {
    for (i, x) in row[..row.len()-1].iter().enumerate() {
        for y in row[i+1..].iter() {
            if x % y == 0 {
                return x / y;
            } else if y % x == 0 {
                return y / x;
            }
        }
    }
    panic!();
}