extern crate rand;

use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

fn code() -> String {
    return ",>+<".to_string();
}

fn main() {
    let v: Vec<&str> = code().split("").collect();

    for i in &v {
        println!("{}", i);
    }
}
