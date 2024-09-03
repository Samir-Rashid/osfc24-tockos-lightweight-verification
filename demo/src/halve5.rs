#![allow(unused)]
use flux_rs::*;

//       Precondition      Postcondition
#[sig(fn(x: i32{x > 0}) -> i32{v: v < x})]
pub fn halve(x: i32) -> i32 {
    x / 2
}

fn main() {
    halve(5);
    halve(-1);
}