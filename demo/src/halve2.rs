#![allow(unused)]
use flux_rs::*;

#[sig(fn(x: i32) -> i32)]
pub fn halve(x: i32) -> i32 {
    x / 2
}
