use std::fs;
use std::io;

use bitvec::BitVec;

use super::day10::knot_hash;

pub fn solve() -> Result<(u32, u32), io::Error> {
    let contents = fs::read_to_string("input/2017/14.txt")?;
    let key = contents.trim_right();
    Ok((part1(key), part2(key)))
}

fn part1(key: &str) -> u32 {
    (0..128)
        .map(|row| {
            knot_hash(&format!("{}-{}", key, row))
                .iter()
                .map(|x| x.count_ones())
                .sum::<u32>()
        }).sum()
}

fn part2(key: &str) -> u32 {
    let mut grid: Vec<BitVec> = (0..128)
        .map(|row| knot_hash(&format!("{}-{}", key, row)).into())
        .collect();

    let mut regions = 0;
    let mut stack = vec![];
    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] {
                regions += 1;
                stack.extend(neighbors(i, j));
                while let Some((r, c)) = stack.pop() {
                    if grid[r][c] {
                        grid[r].set(c, false);
                        stack.extend(neighbors(r, c));
                    }
                }
            }
        }
    }

    regions
}

fn neighbors(r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut n = Vec::with_capacity(4);
    if r != 0 {
        n.push((r - 1, c));
    }
    if r < 127 {
        n.push((r + 1, c));
    }
    if c != 0 {
        n.push((r, c - 1));
    }
    if c < 127 {
        n.push((r, c + 1));
    }
    n
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (8204, 1089));
}
