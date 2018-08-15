use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> Result<(u32, u32), Box<Error>> {
    let file = File::open("input/2017/05.txt")?;

    let list: Vec<i32> = BufReader::new(file)
        .lines()
        .flat_map(|line| line)
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    Ok((part1(&list), part2(&list)))
}

fn part1(list: &[i32]) -> u32 {
    let mut list = list.to_vec();

    let mut i = 0;
    let mut steps = 0;

    while let Some(n) = list.get_mut(i as usize) {
        steps += 1;
        i += *n;
        *n += 1;
    }

    steps
}

fn part2(list: &[i32]) -> u32 {
    let mut list = list.to_vec();

    let mut i = 0;
    let mut steps = 0;

    while let Some(n) = list.get_mut(i as usize) {
        steps += 1;
        i += *n;
        if *n >= 3 {
            *n -= 1;
        } else {
            *n += 1;
        }
    }

    steps
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (342669, 25136209));
}
