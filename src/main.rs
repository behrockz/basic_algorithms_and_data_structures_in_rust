use std::time::{Instant};
use bitvec::order::{Msb0};
use bitvec::view::BitView;
use rand::RngCore;

mod sort_algorithms;
mod collections;
mod trees;
mod graphs;

fn main() {
    let mut rng = rand::thread_rng();

    const SIZE : usize = 4096;
    //let mut p = crate::collections::patricia::PatriciaTree::new();
    let mut p = crate::collections::loopy_patricia::LoopyPatriciaTree::new();
    let mut vector = Vec::new();
    let start = Instant::now();

    let insert_count = 1000000;
    for _i in 0..insert_count {
        let mut bytes: [u8; SIZE] = [0u8; SIZE];
        rng.fill_bytes(&mut bytes);
        let b = bytes.view_bits::<Msb0>().to_bitvec();
        p.insert(&b);
        vector.push(b);
    }

    let elapsed = start.elapsed();
    println!("{} inserts took {:?}", insert_count, elapsed);

    let start = Instant::now();
    for b in vector {
        assert!(p.search(&b));
    }
    let elapsed = start.elapsed();
    println!("{} search took {:?}", insert_count, elapsed);

    let failed_search_count = 1000;
    let start = Instant::now();
    for _i in 0..failed_search_count {
        let mut bytes: [u8; SIZE] = [0u8; SIZE];
        rng.fill_bytes(&mut bytes);
        let b = bytes.view_bits::<Msb0>().to_bitvec();
        assert!(!p.search(&b));
    }
    let elapsed = start.elapsed();
    println!("{} failed search took {:?}", failed_search_count, elapsed);
}