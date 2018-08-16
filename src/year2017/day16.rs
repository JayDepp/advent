use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;

enum Step {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

pub fn solve() -> Result<(String, String), Box<Error>> {
    let steps = input()?;

    Ok((part1(&steps), part2(&steps)))
}

fn input() -> Result<Vec<Step>, Box<Error>> {
    let mut contents = String::new();
    File::open("input/2017/16.txt")?.read_to_string(&mut contents)?;

    contents.trim_right().split(',').map(parse).collect()
}

fn part1(steps: &[Step]) -> String {
    let mut stage = *b"abcdefghijklmnop";
    dance(steps, &mut stage);

    str::from_utf8(&stage).unwrap().to_owned()
}

fn part2(steps: &[Step]) -> String {
    let mut stage = *b"abcdefghijklmnop";

    let begin = stage;

    let mut cycled = 1;

    loop {
        dance(steps, &mut stage);
        if stage == begin {
            break;
        }
        cycled += 1;
    }

    for _ in 0..(1_000_000_000 % cycled) {
        dance(steps, &mut stage);
    }

    str::from_utf8(&stage).unwrap().to_owned()
}

fn parse(text: &str) -> Result<Step, Box<Error>> {
    Ok(match &text[..1] {
        "s" => Step::Spin(text[1..].parse()?),
        "x" => {
            let_scan!(&text[1..]; (let x, "/", let y));
            Step::Exchange(x, y)
        }
        "p" => {
            let bytes = text.as_bytes();
            Step::Partner(bytes[1], bytes[3])
        }
        _ => Err("Unmatched")?,
    })
}

fn dance(steps: &[Step], stage: &mut [u8; 16]) {
    for step in steps {
        match *step {
            Step::Spin(x) => stage.rotate_right(x),
            Step::Exchange(x, y) => stage.swap(x, y),
            Step::Partner(x, y) => {
                for p in stage.iter_mut() {
                    if *p == x {
                        *p = y;
                    } else if *p == y {
                        *p = x;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn ans() {
    let (part1, part2) = solve().unwrap();
    assert_eq!(&part1, "ebjpfdgmihonackl");
    assert_eq!(&part2, "abocefghijklmndp");
}
