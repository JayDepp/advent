use std::cmp::max;
use std::error::Error;
use std::fs;

pub fn solve() -> Result<(u32, u32), Box<Error>> {
    let contents = fs::read_to_string("input/2017/11.txt")?;

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_dist: u32 = 0;

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

fn hex_dist(x: i32, y: i32) -> u32 {
    let x = x.abs() as u32;
    let y = y.abs() as u32;

    x + y.saturating_sub(x) / 2
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (761, 1542));
}
