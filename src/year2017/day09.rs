use std::error::Error;
use std::fs::File;
use std::io::Read;

pub fn solve() -> Result<(u64, u64), Box<Error>> {
    let mut contents = String::new();
    File::open("input/2017/09.txt")?.read_to_string(&mut contents)?;
    let mut iter = contents.trim_right().bytes();

    let mut score = 0;
    let mut depth = 0;
    let mut garbage = 0;

    while let Some(ch) = iter.next() {
        match ch {
            b'{' => {
                depth += 1;
                score += depth;
            }
            b'}' => {
                depth -= 1;
            }
            b'<' => {
                while let Some(ch) = iter.next() {
                    match ch {
                        b'!' => {
                            iter.next().ok_or("Trailing escape.")?;
                        }
                        b'>' => break,
                        _ => garbage += 1,
                    };
                }
            }
            b',' => {}
            _ => Err(format!("Bad input ({})", ch as char))?,
        }
    }

    Ok((score, garbage))
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve().unwrap(), (14212, 6569));
}
