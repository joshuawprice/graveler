// This is an attempt to use SIMD to speed things up, but it makes no difference
// as the compiler figured out how to use it by itself when I started batching it.

#![feature(portable_simd)]
use rand::random;
use rayon::prelude::*;
use std::simd::num::SimdInt;
use std::simd::{cmp::SimdPartialEq, u8x32};

fn main() {
    let largest_paralysis_count: u8 = (0..1_000_000_000 / 32)
        .into_par_iter()
        .map(|_| {
            let mut counts = u8x32::splat(0);
            for _ in 0..231 {
                let rands = u8x32::from_array(random());
                let mod_values = rands % u8x32::splat(4);
                let mask = mod_values.simd_eq(u8x32::splat(0));
                counts += mask.to_int().cast::<u8>();
            }

            counts.to_array().into_iter().max().unwrap()
        })
        .max()
        .unwrap();

    println!("Largest number of paralyses: {}", largest_paralysis_count);
}
