use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;

lazy_static! {
    static ref COMPONENTS: Vec<Vec<(Port, u8)>> = input();
}

type Port = usize;
type Component = (Port, Port);

pub fn solve() -> Result<(u32, u32), Box<Error>> {
    Ok((part1(), part2()))
}

fn input() -> Vec<Vec<(Port, u8)>> {
    let file = File::open("input/2017/24.txt").expect("File not found.");
    let mut components = vec![HashMap::new(); 256];

    for line in BufReader::new(file).lines() {
        let (x, y) = parse(&line.unwrap());
        *components[x].entry(y).or_insert(0) += 1;
        *components[y].entry(x).or_insert(0) += 1;
    }

    components
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect()
}

fn part1() -> u32 {
    fn recur(port: Port, sum: usize, used: &HashMap<(Port, Port), u8>) -> usize {
        COMPONENTS[port]
            .iter()
            .filter(|&&(next, count)| count > *used.get(&(port, next)).unwrap_or(&0))
            .map(|&(next, _)| {
                let mut used = used.clone();
                *used.entry((port, next)).or_insert(0) += 1;
                *used.entry((next, port)).or_insert(0) += 1;
                recur(next, sum + port + next, &used)
            })
            .chain(once(sum))
            .max()
            .unwrap()
    }

    recur(0, 0, &HashMap::new()) as u32
}

fn part2() -> u32 {
    fn recur(port: Port, len: u32, sum: usize, used: &HashMap<(Port, Port), u8>) -> (u32, usize) {
        COMPONENTS[port]
            .iter()
            .filter(|&&(next, count)| count > *used.get(&(port, next)).unwrap_or(&0))
            .map(|&(next, _)| {
                let mut used = used.clone();
                *used.entry((port, next)).or_insert(0) += 1;
                *used.entry((next, port)).or_insert(0) += 1;
                recur(next, len + 1, sum + port + next, &used)
            })
            .chain(once((len, sum)))
            .max()
            .unwrap()
    }

    recur(0, 0, 0, &HashMap::new()).1 as u32
}

fn parse(text: &str) -> Component {
    let_scan!(text; (let x, "/", let y));
    (x, y)
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (1906, 1824));
}
