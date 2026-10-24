use std::ops::RangeBounds;
use std::range::*;

fn main() {}

fn foo(range: impl RangeBounds<u32> + Copy) {}
