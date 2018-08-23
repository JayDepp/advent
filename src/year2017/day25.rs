use std::collections::HashMap;
use std::fs;

use scan_rules::scanner::Word;

struct Input {
    begin_state: char,
    steps: u32,
    blueprint: HashMap<char, (Rule, Rule)>,
}

lazy_static! {
    static ref INPUT: Input = input();
}

struct Rule {
    write: bool,
    offset: i32,
    next: char,
}

pub fn solve() -> usize {
    part1()
}

fn input() -> Input {
    let contents = fs::read_to_string("input/2017/25.txt").expect("Failed to read input");

    let mut iter = contents.split("\n\n");

    let_scan!(iter.next().unwrap(); (
        "Begin in state", let begin_state, ".",
        "Perform a diagnostic checksum after", let steps, "steps."
    ));

    let blueprint = iter
        .map(|rule| {
            let_scan!(rule; (
            "In state", let state: char, ":",
            [ "If the current value is", let _: u32, ":",
                "- Write the value", let write: u8, ".",
                "- Move one slot to the", let dir: Word<&str>, ".",
                "- Continue with state", let next: char, "."
            ]+
        ));
            (
                state,
                (
                    Rule {
                        write: write[0] == 1,
                        offset: if dir[0] == "left" { -1 } else { 1 },
                        next: next[0],
                    },
                    Rule {
                        write: write[1] == 1,
                        offset: if dir[1] == "left" { -1 } else { 1 },
                        next: next[1],
                    },
                ),
            )
        }).collect();

    Input {
        begin_state,
        steps,
        blueprint,
    }
}

fn part1() -> usize {
    let mut tape = HashMap::new();
    let mut state = INPUT.begin_state;
    let mut i = 0;

    for _ in 0..INPUT.steps {
        let cursor = tape.entry(i).or_insert(false);
        let rule = if *cursor {
            &INPUT.blueprint[&state].1
        } else {
            &INPUT.blueprint[&state].0
        };
        *cursor = rule.write;
        state = rule.next;
        i += rule.offset;
    }

    tape.values().filter(|&&x| x).count()
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve(), 4387);
}
