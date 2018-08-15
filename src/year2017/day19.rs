use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

pub fn solve() -> Result<(String, u32), Box<Error>> {
    use self::Dir::*;

    let grid: Vec<Vec<u8>> = BufReader::new(File::open("input/2017/19.txt")?)
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    let mut x = grid[0].iter().position(|&c| c == b'|').ok_or("No start")?;
    let mut y = 0;
    let mut dir = South;
    let mut visited = String::new();
    let mut steps = 0;

    loop {
        steps += 1;
        match dir {
            North => y -= 1,
            South => y += 1,
            East => x += 1,
            West => x -= 1,
        }
        match grid[y][x] {
            b' ' => break,
            b'-' | b'|' => {}
            b'+' => match dir {
                North | South => {
                    if grid[y][x - 1] != b' ' {
                        dir = West;
                    } else if grid[y][x + 1] != b' ' {
                        dir = East;
                    } else {
                        Err(format!("Broken at r{} c{}", y, x))?
                    }
                }
                East | West => {
                    if grid[y - 1][x] != b' ' {
                        dir = North;
                    } else if grid[y + 1][x] != b' ' {
                        dir = South;
                    } else {
                        Err(format!("Broken at r{} c{}", y, x))?
                    }
                }
            },
            c => visited.push(c as char),
        }
    }

    Ok((visited, steps))
}

#[cfg(test)]
#[test]
fn ans() {
    let (part1, part2) = solve().unwrap();
    assert_eq!(&part1, "UICRNSDOK");
    assert_eq!(part2, 16064);
}
