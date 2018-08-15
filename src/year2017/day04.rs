use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

pub fn solve() -> Result<(usize, usize), Box<Error>> {
    let file = File::open("input/2017/04.txt")?;

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let phrases: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let part1 = phrases.iter().filter(|&words| is_unique(words)).count();

    let part2 = phrases
        .iter()
        .filter(|words| is_unique(words.iter().map(|word| lettercount(word))))
        .count();

    Ok((part1, part2))
}

fn lettercount(word: &str) -> [u8; 26] {
    let mut counter = [0; 26];

    for letter in word.bytes() {
        counter[(letter - b'a') as usize] += 1;
    }

    counter
}

fn is_unique<I, T>(iter: I) -> bool
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq,
{
    let mut seen = HashSet::new();

    for item in iter {
        if !seen.insert(item) {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (325, 119));
}
