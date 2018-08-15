static STEP: usize = 329;

pub fn solve() -> (u32, u32) {
    (part1(), part2())
}

fn part1() -> u32 {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);

    let mut i = 0;

    for val in 1..=2017 {
        i = (i + STEP) % val + 1;
        buf.insert(i, val);
    }

    buf[i + 1] as u32
}

fn part2() -> u32 {
    let mut i = 0;
    let mut goal = 0;

    for val in 1..50000000 {
        i = (i + STEP) % val + 1;
        if i == 1 {
            goal = val;
        }
    }

    goal as u32
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve(), (725, 27361412));
}
