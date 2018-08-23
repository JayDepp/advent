use std::fs;
use std::io;

pub fn solve() -> Result<(u32, u32), io::Error> {
    let contents = fs::read_to_string("input/2017/01.txt")?;
    let bytes = contents.trim_right().as_bytes();

    Ok((part1(bytes), part2(bytes)))
}

fn part1(bytes: &[u8]) -> u32 {
    matched_sum(bytes, 1)
}

fn part2(bytes: &[u8]) -> u32 {
    matched_sum(bytes, bytes.len() / 2)
}

fn matched_sum(bytes: &[u8], offset: usize) -> u32 {
    zip_offset(bytes, offset)
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| (a as char).to_digit(10).unwrap())
        .sum()
}

fn zip_offset<'a>(bytes: &'a [u8], offset: usize) -> impl Iterator<Item = (u8, u8)> + 'a {
    bytes
        .iter()
        .zip(bytes[offset..].iter().chain(&bytes[..offset]))
        .map(|(&a, &b)| (a, b))
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (1253, 1278));
}
