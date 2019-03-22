extern crate rand;

use sorting::heap_sort::{Comparable,Sortable};
use rand::prelude::*;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::thread;
use std::iter::{Iterator, IntoIterator};

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

    let test2_size: usize = 1_000_000;
    let mut unsorted2: Vec<i32> = Vec::with_capacity(test2_size);
    println!("Generating unsorted Vec: {} strong.", test2_size);
    for _ in 0..test2_size {
        unsorted2.push(rng.gen_range(0, std::i32::MAX));
    }

    println!("Sorting...");
    let pre_sort2 = Instant::now();
    let sorted2 = match unsorted2.heap_sort() {
        Ok(val) => { val },
        Err(msg) => panic!("Uh-Oh! {}", msg),
    };
    let sort2_dur = pre_sort2.elapsed();
    println!("Done sorting, how long did it take?");
    println!("{} ms", sort2_dur.as_millis());
    println!();
    print!("You don't want to see the result, do you? (y/N):  ");
    io::stdout().flush().expect("Something went wrong");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("WTF");
    if answer.trim().to_lowercase().eq(&String::from("y")) {
        print!("Remember: You asked for this");
        io::stdout().flush().unwrap();
        let one_sec = Duration::from_secs(1);
        for _ in 0..3 {
            thread::sleep(one_sec);
            print!(".");
            io::stdout().flush().unwrap();
        }
        thread::sleep(Duration::from_secs(3));
        println!();
        println!("{:?}", sorted2);
    } else {
        println!("A wise decision.");
    }
    println!("Sortation successful? => {}", is_sorted(&sorted2));
}

/// Iterate over a collection to check sorting.
/// returns true if sorted in ascending order, false if not.
fn is_sorted<'a, T, I>(col: &'a I) -> bool where T: Comparable, I: std::iter::IntoIterator<Item=T>,
                                                     I: Clone {
    let mut it = Box::new(col.clone().into_iter()) as Box<Iterator<Item=T>>;

    let mut prev = match it.next() {
        Some(val) => { val },
        None => return true,
    };

    let mut cur_val: T;
    loop {
        cur_val = match it.next() {
            Some(val) => val,
            None => break,
        };
        if cur_val.compare(&prev) < 0 { return false; } else { prev = cur_val; }
    }

    true
}