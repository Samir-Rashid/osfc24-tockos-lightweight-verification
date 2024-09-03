#![allow(unused)]
use flux_rs::*;

#[trusted]
fn abs(x: i32) -> i32 {
  if x >= 0 {
    x
  } else {
    -x
  }
}