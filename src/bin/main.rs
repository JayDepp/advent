extern crate advent;
use advent::*;

/*
use std::error::Error;

fn day2() -> Result<(), Box<Error>> {
    Ok(())
}
*/

fn main() {
    day2().unwrap_or_else(|e| println!("{:?}", e));
}
