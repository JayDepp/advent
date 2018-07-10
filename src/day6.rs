use std::error::Error;
use std::collections::HashSet;

pub fn day6() -> Result<(), Box<Error>> {
    let mut banks = [4u8, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let mut seen = HashSet::new();

    // Part 1
    while seen.insert(banks) {
        let (&blocks, mut i) = banks.iter().zip((0..16).rev()).max().unwrap();
        let mut blocks = blocks;
        i = 15 - i;
        banks[i] = 0;

        while blocks > 0 {
            i += 1;
            if i == 16 {
                i = 0;
            }
            banks[i] += 1;
            blocks -= 1;
        }
    }

    println!("{}", seen.len());

    // Part 2
    let start = banks;
    let mut count = 0;
    loop {
        count += 1;
        let (&blocks, mut i) = banks.iter().zip((0..16).rev()).max().unwrap();
        let mut blocks = blocks;
        i = 15 - i;
        banks[i] = 0;

        while blocks > 0 {
            i += 1;
            if i == 16 {
                i = 0;
            }
            banks[i] += 1;
            blocks -= 1;
        }

        if banks == start { break; }
    }

    println!("{}", count);

    Ok(())
}
