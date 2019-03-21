extern crate rand;

use sorting::heap_sort::{Comparable,Sortable};
use rand::prelude::*;

fn main() {
    println!("Hello, world!");

    let x = 99i32;
    let y = 99i32;

    println!("X: {}, y: {}, x.compare(y): {}", x, y, x.compare(&y));

    let test_size: usize = 10;
    let mut unsort: Vec<i32> = Vec::with_capacity(test_size);
    let mut rng = thread_rng();
    for _ in 0..test_size {
        unsort.push(rng.gen_range(0, 100));
    }

    println!("Unsorted Vec: {:?}", unsort);

    let sorted = match unsort.heap_sort() {
        Ok(val) => { val },
        Err(msg) => panic!("Uh-Oh! {}", msg),
    };

    println!("Sorted? Vec: {:?}", sorted);
}
