use bitvec::order::Lsb0;
use bitvec::view::BitView;
use rand::RngCore;
use bitvec::prelude::*;

mod sort_algorithms;
mod collections;
mod trees;
mod graphs;

fn main() {
    let mut rng = rand::thread_rng();

    const SIZE : usize = 4096;
    let mut p = crate::collections::patricia::PatriciaTree::new();
    let mut vector = Vec::new();

    for _i in 0..100 {
        let mut bytes: [u8; SIZE] = [0u8; SIZE];
        rng.fill_bytes(&mut bytes);
        let b = bytes.view_bits::<Lsb0>().to_bitvec();
        p.insert(&b);
        assert!(p.search(&b));
        vector.push(b);
    }
    //
    // for b in vector {
    //     assert!(p.search(&b));
    // }

    // for _i in 0..100 {
    //     let mut bytes: [u8; SIZE] = [0u8; SIZE];
    //     rng.fill_bytes(&mut bytes);
    //     let b = bytes.view_bits::<Lsb0>().to_bitvec();
    //     assert!(!p.search(&b));
    // }
}