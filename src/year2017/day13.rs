use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> Result<(u64, u64), Box<Error>> {
    let scanners: Vec<_> = BufReader::new(File::open("input/2017/13.txt")?)
        .lines()
        .map(|line| parse(&line.unwrap()))
        .collect();

    Ok((part1(&scanners), part2(&scanners)))
}

fn part1(scanners: &[(u64, u64)]) -> u64 {
    scanners
        .iter()
        .filter(|&(depth, range)| depth % (2 * (range - 1)) == 0)
        .map(|(depth, range)| depth * range)
        .sum()
}

fn part2(scanners: &[(u64, u64)]) -> u64 {
    (0..)
        .find(|delay| {
            scanners
                .iter()
                .all(|&(depth, range)| (depth + delay) % (2 * (range - 1)) != 0)
        })
        .expect("No delay found")
}

fn parse(text: &str) -> (u64, u64) {
    let_scan!(text; (
        let depth: u64,
        ":",
        let range: u64
    ));

    (depth, range)
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (1704, 3_970_918));
}
