//buggy.rs
use std;
use std::map::hashmap;
use std::map;

fn main() {
    let buggy_map :hashmap<uint, &uint> =
      hashmap::<uint, &uint>();
    buggy_map.insert(42, ~1); //~ ERROR illegal borrow
    
    // but it is ok if we use a temporary
    let tmp = ~2;
    buggy_map.insert(43, tmp);
}
