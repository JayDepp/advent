use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use scan_rules::scanner::Word;

struct Program<'a> {
    weight: u64,
    children: Vec<&'a str>,
}

pub fn solve() -> Result<(String, u64), Box<Error>> {
    let file = File::open("input/2017/07.txt")?;

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Read error."))
        .collect();

    let programs: HashMap<_, _> = lines.iter().map(parse_line).collect();

    let mut stack: HashSet<_> = programs.keys().collect();

    for p in programs.values() {
        for name in &p.children {
            stack.remove(name);
        }
    }

    assert_eq!(stack.len(), 1);
    let root = stack.iter().next().unwrap();

    Ok((root.to_string(), find_imbalance(root, &programs)))
}

fn parse_line(text: &String) -> (&str, Program) {
    let_scan!(text; (
        let name: Word<&str>,
        "(",
        let weight: u64,
        ")",
        [ "->" ]?,
        [ let children: Word<&str> ],*
    ));

    (name, Program { weight, children })
}

fn find_imbalance(root: &str, programs: &HashMap<&str, Program>) -> u64 {
    fn check(weights: &[(u64, u64)]) -> Result<(), u64> {
        let mut weights: Vec<_> = weights.iter().collect();
        weights.sort_by_key(|(_, w)| w);
        let a = weights[0];
        let b = weights[1];
        let c = weights[weights.len() - 1];
        if a.1 != b.1 {
            Err(a.0 + b.1 - a.1)
        } else if b.1 != c.1 {
            Err(c.0 - (c.1 - b.1))
        } else {
            Ok(())
        }
    }

    // TODO: Remove recursion
    fn recur(name: &str, map: &HashMap<&str, Program>) -> Result<(u64, u64), u64> {
        let node = &map[name];
        let weights = node.children
            .iter()
            .map(|child| recur(child, map))
            .collect::<Result<Vec<_>, _>>()?;

        if weights.is_empty() {
            return Ok((node.weight, node.weight));
        }

        check(&weights)?;

        let weight_above: u64 = weights.iter().map(|(_, w)| w).sum();

        Ok((node.weight, node.weight + weight_above))
    }

    recur(root, programs).expect_err("No imbalance found.")
}

#[cfg(test)]
#[test]
fn ans() {
    let (part1, part2) = solve().unwrap();
    assert_eq!(&part1, "vgzejbd");
    assert_eq!(part2, 1226);
}
