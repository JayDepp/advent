use std::io::Read;
use std::error::Error;
use std::fs::File;

pub fn day1() -> Result<(), Box<Error>> {
    // Input
    let mut file = File::open("day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let bytes = contents.trim_right().as_bytes();

    // Part 1
    let mut sum = bytes.iter()
        .zip(&bytes[1..])
        .fold(0, |acc, (a, b)|
            if a == b {
                acc + (*a as char).to_digit(10).unwrap()
            } else {
                acc
            });
    
    if bytes[0] == bytes[bytes.len()-1] {
        sum += (bytes[0] as char).to_digit(10).unwrap();
    }

    println!("Part 1: {}", sum);

    // Part 2
    let mut sum = 0;
    let len = bytes.len();
    let skip = len / 2;
    for i in 0..len {
        if bytes[i] == bytes[(i + skip) % len] {
            sum += (bytes[i] as char).to_digit(10).unwrap();
        }
    }
    println!("Part 2: {}", sum);

    Ok(())
}