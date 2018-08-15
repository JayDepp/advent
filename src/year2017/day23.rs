use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use scan_rules::{scanner::NonSpace, ScanError};

#[derive(Debug, Copy, Clone)]
enum Val {
    Reg(usize),
    Lit(i64),
}

#[derive(Debug, Copy, Clone)]
enum Ins {
    Set(usize, Val),
    Sub(usize, Val),
    Mul(usize, Val),
    Jnz(Val, Val),
}

pub fn solve() -> Result<(usize, usize), Box<Error>> {
    Ok((part1(&input()?), part2()))
}

fn input() -> Result<Vec<Ins>, Box<Error>> {
    let file = File::open("input/2017/23.txt")?;

    Ok(BufReader::new(file)
        .lines()
        .flat_map(|x| x)
        .map(|line| parse(&line))
        .collect::<Result<_, _>>()?)
}

fn part1(program: &[Ins]) -> usize {
    let mut regs = [0; 8];
    let mut i = 0;
    let mut count = 0;

    while (i as usize) < program.len() {
        match program[i as usize] {
            Ins::Set(x, y) => regs[x] = read(y, &regs),
            Ins::Sub(x, y) => regs[x] -= read(y, &regs),
            Ins::Mul(x, y) => {
                count += 1;
                regs[x] *= read(y, &regs)
            }
            Ins::Jnz(x, y) => {
                if read(x, &regs) != 0 {
                    i += read(y, &regs) - 1;
                }
            }
        }
        i += 1;
    }

    count
}

fn part2() -> usize {
    (0..=1000)
        .map(|x| x * 17 + 106500)
        .filter(|&x| !is_prime(x))
        .count()
}

fn is_prime(n: u32) -> bool {
    let sroot = (n as f32).sqrt() as u32 + 1;
    let mut i = 3;

    if n == 1 || (n > 2 && n % 2 == 0) {
        return false;
    }

    while i < sroot {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn read(val: Val, regs: &[i64]) -> i64 {
    match val {
        Val::Lit(num) => num,
        Val::Reg(reg) => regs[reg],
    }
}

fn parse(text: &str) -> Result<Ins, ScanError> {
    scan!(text;
        ("set", let x, let y: NonSpace<&str>) => Ins::Set(idx(x), val(y)?),
        ("sub", let x, let y: NonSpace<&str>) => Ins::Sub(idx(x), val(y)?),
        ("mul", let x, let y: NonSpace<&str>) => Ins::Mul(idx(x), val(y)?),
        ("jnz", let x: NonSpace<&str>, let y: NonSpace<&str>) => Ins::Jnz(val(x)?, val(y)?),
    )
}

fn idx(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn val(text: &str) -> Result<Val, ScanError> {
    scan!(text;
        (let num) => Val::Lit(num),
        (let reg) => Val::Reg(idx(reg)),
    )
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (3969, 917));
}
