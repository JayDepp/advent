use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn day4() -> Result<(), Box<Error>> {
    // Input
    let file = File::open("day4.txt")?;
    
    let lines: Vec<_> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    
    let phrases: Vec<Vec<_>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();
    
    // Part 1
    let ans = phrases
        .iter()
        .filter(|words| {
            let len = words.len();
            let set: HashSet<_> = words.iter().collect();
            set.len() == len
        })
        .count();
    
    println!("{}", ans);
    
    // Part 2
    let ans = phrases
        .iter()
        .filter(|words| {
            let words = words.iter().map(|word| lettercount(word));
            let len = words.len();
            let set: HashSet<_> = words.collect();
            set.len() == len
        })
        .count();
    
    println!("{}", ans);
    
    Ok(())
}

fn lettercount(word: &str) -> [u8; 26] {
    let mut counter = [0; 26];

    for letter in word.bytes() {
        counter[letter as usize - 'a' as usize] += 1;
    }

    counter
}