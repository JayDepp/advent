use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use scan_rules::scanner::{NonSpace, Word};

struct Instruction<'a> {
    reg: &'a str,
    delta: i64,
    cond_reg: &'a str,
    cond_cmp: &'a str,
    cond_amt: i64,
}

pub fn solve() -> Result<(i64, i64), Box<Error>> {
    let file = File::open("input/2017/08.txt")?;

    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut regs: HashMap<&str, i64> = HashMap::new();
    let mut max = 0;

    for line in &lines {
        let ins = parse_ins(line);
        let cond_val = regs.get(ins.cond_reg).cloned().unwrap_or_default();
        let cond = match ins.cond_cmp {
            "<" => cond_val < ins.cond_amt,
            ">" => cond_val > ins.cond_amt,
            "<=" => cond_val <= ins.cond_amt,
            ">=" => cond_val >= ins.cond_amt,
            "==" => cond_val == ins.cond_amt,
            "!=" => cond_val != ins.cond_amt,
            _ => panic!("Unrecognized comparator."),
        };
        if cond {
            let reg = regs.entry(ins.reg).or_default();
            *reg += ins.delta;
            max = cmp::max(*reg, max);
        }
    }

    Ok((*regs.values().max().unwrap(), max))
}

fn parse_ins(text: &str) -> Instruction {
    let_scan!(text; (
        let reg: Word<&str>,
        let inc_or_dec: Word<&str>,
        let amt: i64,
        "if",
        let cond_reg: Word<&str>,
        let cond_cmp: NonSpace<&str>,
        let cond_amt: i64
    ));

    let delta = match inc_or_dec {
        "inc" => amt,
        "dec" => -amt,
        _ => panic!("Unrecognized instruction"),
    };

    Instruction {
        reg,
        delta,
        cond_reg,
        cond_cmp,
        cond_amt,
    }
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (2971, 4254));
}
