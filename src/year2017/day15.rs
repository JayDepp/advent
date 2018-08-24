use std::iter::FusedIterator;

pub fn solve() -> (u32, u32) {
    (part1(), part2())
}

#[derive(Debug, Copy, Clone)]
struct Gen {
    factor: u64,
    prev: u64,
}

impl Iterator for Gen {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = (self.prev * self.factor) % 2_147_483_647;
        Some(self.prev)
    }
}

impl FusedIterator for Gen {}

fn part1() -> u32 {
    let gen_a = Gen {
        factor: 16807,
        prev: 783,
    };
    let gen_b = Gen {
        factor: 48271,
        prev: 325,
    };

    gen_a
        .take(40_000_000)
        .zip(gen_b)
        .filter(|&(a, b)| a % 65536 == b % 65536)
        .count() as u32
}

fn part2() -> u32 {
    let gen_a = Gen {
        factor: 16807,
        prev: 783,
    };
    let gen_b = Gen {
        factor: 48271,
        prev: 325,
    };

    gen_a
        .filter(|&a| a % 4 == 0)
        .take(5_000_000)
        .zip(gen_b.filter(|&b| b % 8 == 0))
        .filter(|&(a, b)| a % 65536 == b % 65536)
        .count() as u32
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve(), (650, 336));
}
