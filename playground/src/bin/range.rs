use std::ops::RangeBounds;

fn main() {}

fn foo(_range: impl RangeBounds<u32> + Copy) {}
