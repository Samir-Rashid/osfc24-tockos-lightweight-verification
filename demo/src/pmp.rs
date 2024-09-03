#![allow(unused)]
use flux_rs::*;

// low level properties
#[flux::refined_by(configured_0: bool, configured_1: bool)]
pub struct PmpRegions {
    #[flux::field(bool[configured_0])]
    configured_0: bool,
    #[flux::field(bool[configured_1])] // syntax same meaning as {v: v == configured_1} 
    configured_1: bool,
}

#[flux::sig(fn (pmp: &strg PmpRegions, num_regions: usize) 
    ensures pmp: PmpRegions{v: ((0 < num_regions) == v.configured_0) && ((1 < num_regions) == v.configured_1)})]
pub fn configure(pmp: &mut PmpRegions, num_regions: usize) {
    // pmp.configured_0 = false; // bug! must reset [num_regions, end]
    // pmp.configured_1 = false;

    if num_regions > 0 {
        pmp.configured_0 = true;
    }

    if num_regions > 1 {
        pmp.configured_1 = true;
    }
}

// high level properties
// unused & private function. This is a "static assert".
#[flux::sig(fn (pmp: &strg PmpRegions, 
    num_regions: usize{num_regions >= 0 && num_regions <= 2}) 
    -> bool[true] 
    ensures pmp: PmpRegions
)]
fn verify(pmp: &mut PmpRegions, num_regions: usize) -> bool {
    configure(pmp, num_regions);
    (bool_to_int(pmp.configured_0) + bool_to_int(pmp.configured_1)) == num_regions
}

#[flux::sig(fn (b: bool) -> usize{v: if b {1 == v} else {0 == v}})]
fn bool_to_int(b: bool) -> usize {
    if b {1} else {0}
}