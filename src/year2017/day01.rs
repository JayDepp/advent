use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn solve() -> Result<(u32, u32), Box<Error>> {
    let mut file = File::open("input/2017/01.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let bytes = contents.trim_right().as_bytes();

    Ok((part1(bytes), part2(bytes)))
}

fn part1(bytes: &[u8]) -> u32 {
    let mut sum = bytes.iter().zip(&bytes[1..]).fold(0, |acc, (a, b)| {
        if a == b {
            acc + (*a as char).to_digit(10).unwrap()
        } else {
            acc
        }
    });

    if bytes[0] == bytes[bytes.len() - 1] {
        sum += (bytes[0] as char).to_digit(10).unwrap();
    }

    sum
}

fn part2(bytes: &[u8]) -> u32 {
    let mut sum = 0;
    let len = bytes.len();
    let skip = len / 2;

    for i in 0..len {
        if bytes[i] == bytes[(i + skip) % len] {
            sum += (bytes[i] as char).to_digit(10).unwrap();
        }
    }

    sum
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (1253, 1278));
}
