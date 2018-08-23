use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub fn solve() -> Result<(u32, u32), Box<Error>> {
    let grid: Vec<Vec<u32>> = BufReader::new(File::open("input/2017/02.txt")?)
        .lines()
        .map(|line| line.expect("read error"))
        .map(|line| line.split('\t').map(|n| n.parse()).collect())
        .collect::<Result<_, _>>()?;

    let part1 = grid
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum();

    let part2 = grid.iter().map(|row| find_even_div(row)).sum();

    Ok((part1, part2))
}

fn find_even_div(row: &[u32]) -> u32 {
    row.iter()
        .cloned()
        .tuple_combinations()
        .filter_map(|(x, y)| {
            if x % y == 0 {
                Some(x / y)
            } else if y % x == 0 {
                Some(y / x)
            } else {
                None
            }
        }).next()
        .expect("No even div found.")
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (32121, 197));
}
