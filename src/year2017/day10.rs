use std::error::Error;
use std::fs;

pub fn solve() -> Result<(u16, String), Box<Error>> {
    let contents = fs::read_to_string("input/2017/10.txt")?;
    let bytes = scan!(&contents; ([let b: u8],*) => b)?;
    Ok((part1(&bytes), part2(contents.trim_right())))
}

pub fn twist(rope: &mut [u8; 256], pos: u8, len: u8) {
    if let (over, true) = pos.overflowing_add(len) {
        let over = over as usize;
        let pos = pos as usize;
        let mut buf = rope[pos..].to_vec();
        buf.extend_from_slice(&rope[..over]);
        buf.reverse();
        let (left, right) = buf.split_at(256 - pos);
        rope[pos..].copy_from_slice(left);
        rope[..over].copy_from_slice(right)
    } else {
        rope[pos as usize..(pos + len) as usize].reverse();
    }
}

fn part1(lengths: &[u8]) -> u16 {
    let mut rope = [0u8; 256];
    rope.iter_mut().enumerate().for_each(|(i, p)| *p = i as u8);

    let mut i = 0;
    for (skip, &len) in lengths.iter().enumerate() {
        twist(&mut rope, i, len);
        i = i.wrapping_add(len).wrapping_add(skip as u8);
    }

    u16::from(rope[0]) * u16::from(rope[1])
}

fn part2(input: &str) -> String {
    knot_hash(input)
        .iter()
        .map(|dense| format!("{:02x}", dense))
        .fold(String::with_capacity(32), |x, y| x + &y)
}

pub(crate) fn knot_hash(input: &str) -> Vec<u8> {
    let lens: Vec<_> = input
        .bytes()
        .chain([17, 31, 73, 47, 23].iter().cloned())
        .collect();

    let mut rope = [0u8; 256];
    rope.iter_mut().enumerate().for_each(|(i, p)| *p = i as u8);

    let mut i = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for &len in &lens {
            twist(&mut rope, i, len);
            i = i.wrapping_add(len).wrapping_add(skip);
            skip = skip.wrapping_add(1);
        }
    }

    rope.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |x, y| x ^ y))
        .collect()
}

#[cfg(test)]
#[test]
fn ans() {
    let (part1, part2) = solve().unwrap();
    assert_eq!(part1, 9656);
    assert_eq!(&part2, "20b7b54c92bf73cf3e5631458a715149");
}
