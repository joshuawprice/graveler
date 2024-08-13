// This was my first attempt, it's a bit simpler as it doesn't using batching, but also half the speed.

use rand::random;
use rayon::prelude::*;

fn main() {
    let largest_paralysis_count: u8 = (0..1_000_000_000)
        .into_par_iter()
        .map(|_| {
            (0..231)
                .map(|_| if random::<u8>() % 4 == 0 { 1 } else { 0 })
                .sum::<u8>()
        })
        .max()
        .unwrap();

    println!("Largest number of paralyses: {}", largest_paralysis_count);
}
