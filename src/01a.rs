use std::io::{self, prelude::*};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let modules = lines.map(|module| module.parse::<i32>().unwrap());
    let fuel: i32 = modules.map(|module| max(0, module / 3 - 2)).sum();
    println!("{}", fuel);
}
