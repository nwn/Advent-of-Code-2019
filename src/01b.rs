use std::io::{self, prelude::*};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let modules = lines.map(|module| module.parse::<i32>().unwrap());
    let fuel: i32 = modules.map(|mut mass| {
        let mut total = 0;
        while mass > 0 {
            let added = max(0, mass / 3 - 2);
            total += added;
            mass = added;
        }
        total
    }).sum();
    println!("{}", fuel);
}
