pub fn solve() -> (u16, String) {
    (part1(), part2())
}

fn twist(rope: &mut [u8], pos: usize, len: usize) {
    if pos + len > 256 {
        let over = pos + len - 256;
        let mut buf = rope[pos..].to_vec();
        buf.extend_from_slice(&rope[..over]);
        buf.reverse();
        let mid = 256 - pos;
        rope[pos..].copy_from_slice(&buf[..mid]);
        rope[..over].copy_from_slice(&buf[mid..])
    } else {
        rope[pos..pos + len].reverse();
    }
}

fn part1() -> u16 {
    let lens = [
        206, 63, 255, 131, 65, 80, 238, 157, 254, 24, 133, 2, 16, 0, 1, 3,
    ];
    let mut rope = [0u8; 256];
    for i in 1..256 {
        rope[i] = i as u8;
    }
    let mut i = 0;
    for (skip, &len) in lens.iter().enumerate() {
        twist(&mut rope, i, len);
        i = (i + len + skip) % 256;
    }

    rope[0] as u16 * rope[1] as u16
}

fn part2() -> String {
    let input = "206,63,255,131,65,80,238,157,254,24,133,2,16,0,1,3";
    knot_hash(input)
        .iter()
        .map(|dense| format!("{:02x}", dense))
        .fold(String::with_capacity(32), |x, y| x + &y)
}

pub(crate) fn knot_hash(input: &str) -> Vec<u8> {
    let lens: Vec<_> = input
        .as_bytes()
        .iter()
        .chain(&[17, 31, 73, 47, 23])
        .map(|&x| x as usize)
        .collect();

    let mut rope = [0u8; 256];
    for i in 1..256 {
        rope[i] = i as u8;
    }

    let mut i = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for &len in &lens {
            twist(&mut rope, i, len);
            i = (i + len + skip) % 256;
            skip += 1;
        }
    }

    rope.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |x, y| x ^ y))
        .collect()
}

#[cfg(test)]
#[test]
fn ans() {
    let (part1, part2) = solve();
    assert_eq!(part1, 9656);
    assert_eq!(&part2, "20b7b54c92bf73cf3e5631458a715149");
}
