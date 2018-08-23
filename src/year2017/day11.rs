use std::cmp::max;
use std::error::Error;
use std::fs;

pub fn solve() -> Result<(u64, u64), Box<Error>> {
    let contents = fs::read_to_string("input/2017/11.txt")?;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut max_dist: u64 = 0;
    for dir in contents.trim_right().split(',') {
        match dir {
            "n" => {
                y += 2;
            }
            "s" => {
                y -= 2;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            "sw" => {
                x -= 1;
                y -= 1;
            }
            "ne" => {
                x += 1;
                y += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            _ => Err(format!("Bad direction ({})", dir))?,
        }
        max_dist = max(max_dist, hex_dist(x, y));
    }

    Ok((hex_dist(x, y), max_dist))
}

fn hex_dist(x: i64, y: i64) -> u64 {
    let x = x.abs() as u64;
    let y = y.abs() as u64;

    if y > x {
        x + (y - x) / 2
    } else {
        x
    }
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (761, 1542));
}
