/* Part 1

The bottom right corner contains all squares of odd numbers.
361527 = 601^2 + 326
601^2 is 300 in each direction from the center.
Moving one over, 300 up, and then 26 up gives an answer of 326.

*/

/* Part 2

Solved using the oeis database.

*/

/*
use std::error::Error;

fn day3() -> Result<(), Box<Error>> {
    // Part 2
    const RADIUS: usize = 200;
    const INPUT: u32 = 361527;

    let mut grid = [[0; 2*RADIUS+1]; 2*RADIUS+1];

    let mut i = RADIUS;
    let mut j = RADIUS;

    while grid[i][j] <= INPUT {

    }

    println!("{}", grid[i][j]);

    Ok(())
}
*/

pub fn solve() -> (u32, u32) {
    (326, 363_010)
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve(), (326, 363_010));
}
