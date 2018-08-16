use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use scan_rules::{scanner::NonSpace, ScanError};

#[derive(Debug, Copy, Clone)]
enum Val {
    Reg(char),
    Lit(i64),
}

#[derive(Debug, Copy, Clone)]
enum Ins {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(char),
    Jgz(Val, Val),
}

pub fn solve() -> Result<(i64, u64), Box<Error>> {
    let program = input()?;

    Ok((part1(&program), part2(&program)))
}

fn input() -> Result<Vec<Ins>, Box<Error>> {
    let file = File::open("input/2017/18.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .flat_map(|x| x)
        .map(|line| parse(&line))
        .collect::<Result<_, _>>()?)
}

fn part1(program: &[Ins]) -> i64 {
    let mut regs = HashMap::new();
    let mut last_played = 0;

    let mut i = 0;

    loop {
        match program[i as usize] {
            Ins::Snd(x) => last_played = read(x, &regs),
            Ins::Set(x, y) => *regs.entry(x).or_insert(0) = read(y, &regs),
            Ins::Add(x, y) => *regs.entry(x).or_insert(0) += read(y, &regs),
            Ins::Mul(x, y) => *regs.entry(x).or_insert(0) *= read(y, &regs),
            Ins::Mod(x, y) => *regs.entry(x).or_insert(0) %= read(y, &regs),
            Ins::Rcv(x) => {
                if *regs.get(&x).unwrap_or(&0) != 0 {
                    return last_played;
                }
            }
            Ins::Jgz(x, y) => {
                if read(x, &regs) > 0 {
                    i += read(y, &regs) - 1;
                }
            }
        }
        i += 1;
    }
}

fn part2(program: &[Ins]) -> u64 {
    let (tx1, rc1) = channel();
    let (tx2, rc2) = channel();

    let chans = vec![(tx1, rc2), (tx2, rc1)];

    let handle = chans
        .into_iter()
        .enumerate()
        .map(|(id, (tx, rc))| {
            let program = program.to_vec();
            thread::spawn(move || {
                let mut pos = 0;
                let mut regs = HashMap::new();
                let mut count = 0;
                regs.insert('p', id as i64);
                loop {
                    match program[pos as usize] {
                        Ins::Snd(x) => {
                            count += 1;
                            if tx.send(read(x, &regs)).is_err() {
                                return count;
                            }
                        }
                        Ins::Set(x, y) => {
                            let y = read(y, &regs);
                            regs.insert(x, y);
                        }
                        Ins::Add(x, y) => *regs.entry(x).or_insert(0) += read(y, &regs),
                        Ins::Mul(x, y) => *regs.entry(x).or_insert(0) *= read(y, &regs),
                        Ins::Mod(x, y) => *regs.entry(x).or_insert(0) %= read(y, &regs),
                        Ins::Rcv(x) => match rc.recv_timeout(Duration::from_secs(5)) {
                            Ok(y) => {
                                regs.insert(x, y);
                            }
                            _ => return count,
                        },
                        Ins::Jgz(x, y) => {
                            if read(x, &regs) > 0 {
                                pos += read(y, &regs) - 1;
                            }
                        }
                    }
                    pos += 1;
                }
            })
        })
        .last()
        .unwrap();

    handle.join().unwrap()
}

fn read(val: Val, regs: &HashMap<char, i64>) -> i64 {
    match val {
        Val::Lit(num) => num,
        Val::Reg(reg) => *regs.get(&reg).unwrap_or(&0),
    }
}

fn parse(text: &str) -> Result<Ins, ScanError> {
    scan!(text;
        ("snd", let x: NonSpace<&str>) => Ins::Snd(val(x)?),
        ("set", let x, let y: NonSpace<&str>) => Ins::Set(x, val(y)?),
        ("add", let x, let y: NonSpace<&str>) => Ins::Add(x, val(y)?),
        ("mul", let x, let y: NonSpace<&str>) => Ins::Mul(x, val(y)?),
        ("mod", let x, let y: NonSpace<&str>) => Ins::Mod(x, val(y)?),
        ("rcv", let x) => Ins::Rcv(x),
        ("jgz", let x: NonSpace<&str>, let y: NonSpace<&str>) => Ins::Jgz(val(x)?, val(y)?),
    )
}

fn val(text: &str) -> Result<Val, ScanError> {
    scan!(text;
        (let num) => Val::Lit(num),
        (let reg) => Val::Reg(reg),
    )
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (1187, 5969));
}
