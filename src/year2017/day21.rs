use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Grid = Vec<Vec<bool>>;
type RuleBook = HashMap<Grid, Grid>;

pub fn solve() -> Result<(usize, usize), Box<Error>> {
    let rules = input()?;

    Ok((part1(&rules), part2(&rules)))
}

fn input() -> Result<RuleBook, Box<Error>> {
    let file = File::open("input/2017/21.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .flat_map(|x| x)
        .map(|line| parse(&line))
        .collect())
}

fn part1(rules: &RuleBook) -> usize {
    let mut canvas = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..5 {
        enhance(&mut canvas, rules);
    }

    canvas.iter().flat_map(|r| r.iter().filter(|&&x| x)).count()
}

fn part2(rules: &RuleBook) -> usize {
    let mut canvas = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..18 {
        enhance(&mut canvas, rules);
    }

    canvas.iter().flat_map(|r| r.iter().filter(|&&x| x)).count()
}

fn enhance(canvas: &mut Grid, rules: &RuleBook) {
    let len = canvas.len();
    let skip = if len % 2 == 0 { 2 } else { 3 };
    let new_len = len + len / skip;
    let mut new_canvas = vec![vec![false; new_len]; new_len];
    for i in 0..(len / skip) {
        let r = i * skip;
        for j in 0..(len / skip) {
            let c = j * skip;
            let v: Grid = canvas[r..r + skip]
                .iter()
                .map(|row| row[c..c + skip].to_vec())
                .collect();
            copy(&mut new_canvas, find(v, rules), r + i, c + j);
        }
    }
    *canvas = new_canvas;
}

fn find(mut chunk: Grid, rules: &RuleBook) -> &Grid {
    for _ in 0..2 {
        for _ in 0..4 {
            if let Some(found) = rules.get(&chunk) {
                return found;
            }
            rotate(&mut chunk);
        }
        transpose(&mut chunk);
    }

    panic!("Rule not found");
}

fn rotate(chunk: &mut Grid) {
    let len = chunk.len();

    let new = (0..len)
        .map(|i| (0..len).map(|j| chunk[len - j - 1][i]).collect())
        .collect();

    *chunk = new;
}

fn transpose(chunk: &mut Grid) {
    let len = chunk.len();

    let new = (0..len)
        .map(|i| (0..len).map(|j| chunk[j][i]).collect())
        .collect();

    *chunk = new;
}

fn copy(dest: &mut Grid, source: &Grid, r: usize, c: usize) {
    let len = source.len();
    for (r_d, r_s) in dest[r..(r + len)].iter_mut().zip(source.iter()) {
        r_d[c..(c + len)].copy_from_slice(r_s);
    }
}

fn parse(text: &str) -> (Grid, Grid) {
    let v: Vec<Grid> = text
        .split(" => ")
        .map(|side| {
            side.split('/')
                .map(|line| line.bytes().map(|c| c == b'#').collect())
                .collect()
        })
        .collect();

    let mut iter = v.into_iter();

    (iter.next().unwrap(), iter.next().unwrap())
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (176, 2_368_161));
}
