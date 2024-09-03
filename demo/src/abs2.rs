#![allow(unused)]
use flux_rs::*;

#[sig(fn(x: i32) -> i32{v: v >= 0 && v >= x})]
fn abs(x: i32) -> i32 {
  if x >= 0 {
    x
  } else {
    -x
  }
}