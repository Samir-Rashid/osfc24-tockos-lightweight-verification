#![allow(unused)]
use flux_rs::*;

#[sig(fn(x: i32) -> i32{v: v < x})]
pub fn halve(x: i32) -> i32 {
    x / 2
}
