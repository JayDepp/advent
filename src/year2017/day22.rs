use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    r: i32,
    c: i32,
}

enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn turn_around(&mut self) {
        *self = match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

use self::Dir::*;
use self::State::*;

type Grid1 = HashSet<Point>;
type Grid2 = HashMap<Point, State>;

pub fn solve() -> Result<(usize, usize), Box<Error>> {
    let grid1 = input()?;
    let grid2 = grid1.iter().map(|&p| (p, Infected)).collect();

    Ok((part1(grid1), part2(grid2)))
}

fn input() -> Result<Grid1, Box<Error>> {
    let file = File::open("input/2017/22.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .flat_map(|x| x)
        .zip(-12..=12)
        .flat_map(|(line, r)| {
            line.bytes()
                .zip(-12..=12)
                .filter_map(|(x, c)| {
                    if x == b'#' {
                        Some(Point { r, c })
                    } else {
                        None
                    }
                })
                .collect::<Grid1>()
        })
        .collect())
}

fn part1(mut grid: Grid1) -> usize {
    let mut p = Point { r: 0, c: 0 };
    let mut dir = North;
    let mut count = 0;
    for _ in 0..10000 {
        if grid.contains(&p) {
            dir.turn_right();
            grid.remove(&p);
        } else {
            dir.turn_left();
            grid.insert(p);
            count += 1;
        }
        match dir {
            North => p.r -= 1,
            South => p.r += 1,
            West => p.c -= 1,
            East => p.c += 1,
        }
    }
    count
}

fn part2(mut grid: Grid2) -> usize {
    let mut p = Point { r: 0, c: 0 };
    let mut dir = North;
    let mut count = 0;
    for _ in 0..10000000 {
        let node = grid.entry(p).or_insert(Clean);
        match node {
            Clean => {
                dir.turn_left();
                *node = Weakened;
            }
            Weakened => {
                count += 1;
                *node = Infected;
            }
            Infected => {
                dir.turn_right();
                *node = Flagged;
            }
            Flagged => {
                dir.turn_around();
                *node = Clean;
            }
        }
        match dir {
            North => p.r -= 1,
            South => p.r += 1,
            West => p.c -= 1,
            East => p.c += 1,
        }
    }
    count
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (5305, 2511424));
}
