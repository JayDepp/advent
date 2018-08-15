use std::collections::HashSet;

pub fn solve() -> (usize, usize) {
    let mut banks = [4u8, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let mut seen = HashSet::new();

    // Part 1
    while seen.insert(banks) {
        spread(&mut banks);
    }

    // Part 2
    let start = banks;
    let mut count = 0;
    loop {
        count += 1;
        spread(&mut banks);
        if banks == start {
            break;
        }
    }

    (seen.len(), count)
}

fn spread(banks: &mut [u8; 16]) {
    let (&blocks, mut i) = banks.iter().zip((0..16).rev()).max().unwrap();
    i = 15 - i;
    banks[i] = 0;

    for _ in 0..blocks {
        i += 1;
        if i == 16 {
            i = 0;
        }
        banks[i] += 1;
    }
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve(), (6681, 2392));
}
