use rayon::prelude::*;

const BATCH_SIZE: usize = 32;

fn main() {
    let largest_paralysis_count: u8 = (0..1_000_000_000 / BATCH_SIZE)
        .into_par_iter()
        .map(|_| {
            let mut counts = [0; BATCH_SIZE];
            let mut rng = fastrand::Rng::new();
            for _ in 0..231 {
                let mut rands = [0u8; BATCH_SIZE];
                rng.fill(&mut rands);

                for i in 0..BATCH_SIZE {
                    counts[i] += if rands[i] % 4 == 0 { 1 } else { 0 }
                }
            }
            counts.into_iter().max().unwrap()
        })
        .max()
        .unwrap();

    println!("Largest number of paralyses: {}", largest_paralysis_count);
}
