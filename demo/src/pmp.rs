#![allow(unused)]
use flux_rs::*;

flux_rs::defs! {
    fn configured_regions(r: PmpRegions) -> int {
        bool_to_int(r.configured_0) + bool_to_int(r.configured_1)
    }

    fn bool_to_int(b: bool) -> int {
        if b { 1 } else { 0 }
    }
}

// low level properties
#[flux::refined_by(configured_0: bool, configured_1: bool)]
pub struct PmpRegions {
    #[flux::field(bool[configured_0])]
    configured_0: bool,
    #[flux::field(bool[configured_1])]
    configured_1: bool,
}

// high level properties
#[flux::sig(fn (pmp: &strg PmpRegions, num_regions: usize{num_regions < 2}) 
    ensures pmp: PmpRegions{v: configured_regions(v) == num_regions })]
pub fn configure(pmp: &mut PmpRegions, num_regions: usize) {
    pmp.configured_0 = false; // bug! must reset [num_regions, end]
    pmp.configured_1 = false;

    if num_regions > 0 {
        pmp.configured_0 = true;
    }

    if num_regions > 1 {
        pmp.configured_1 = true;
    }
}
