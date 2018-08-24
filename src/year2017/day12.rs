use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> Result<(u64, u64), Box<Error>> {
    let pipes: Vec<_> = BufReader::new(File::open("input/2017/12.txt")?)
        .lines()
        .map(|line| parse(&line.unwrap()))
        .collect();

    Ok((part1(&pipes), part2(&pipes)))
}

fn part1(pipes: &[Vec<usize>]) -> u64 {
    let mut seen = HashSet::new();
    let mut stack = vec![0];
    let mut size = 0;

    while let Some(x) = stack.pop() {
        if seen.insert(x) {
            size += 1;
            stack.extend(&pipes[x]);
        }
    }

    size
}

fn part2(pipes: &[Vec<usize>]) -> u64 {
    let mut seen = [false; 2000];
    let mut stack: Vec<usize> = vec![];
    let mut groups = 0;
    let pos = 0;

    while let Some(pos) = find(&seen, pos) {
        groups += 1;
        seen[pos] = true;
        stack.extend(&pipes[pos]);
        while let Some(x) = stack.pop() {
            if !seen[x] {
                seen[x] = true;
                stack.extend(&pipes[x]);
            }
        }
    }

    groups
}

fn find(seen: &[bool], pos: usize) -> Option<usize> {
    seen[pos..]
        .iter()
        .enumerate()
        .find(|(_, &b)| !b)
        .map(|(i, _)| i)
}

fn parse(text: &str) -> Vec<usize> {
    let_scan!(text; (
        let _: usize,
        "<->",
        [ let pipes: usize ],*
    ));

    pipes
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (113, 202));
}
