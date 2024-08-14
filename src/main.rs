use rayon::prelude::*;

const BATCH_SIZE: usize = 32;

fn main() {
    let largest_paralysis_count: u8 = (0..1_000_000_000 / BATCH_SIZE)
        .into_par_iter()
        .map(|_| {
            let mut rng = fastrand::Rng::new();

            (0..BATCH_SIZE)
                .into_iter()
                .map(|_| {
                    // We need 231 four sided dice rolls. 58 × 4 = 232.
                    let mut rands = [0u8; 58];
                    rng.fill(&mut rands);

                    // This next part is heavily inspired by thorio (https://github.com/thorio)
                    // https://github.com/arhourigan/graveler/issues/4#issuecomment-2282775988
                    let mut ones = rands
                        .iter()
                        .take(57)
                        .map(|rand| {
                            let mut ones = 0;
                            ones += if rand & 0b00000011 == 0 { 1 } else { 0 };
                            ones += if rand & 0b00001100 == 0 { 1 } else { 0 };
                            ones += if rand & 0b00110000 == 0 { 1 } else { 0 };
                            ones += if rand & 0b11000000 == 0 { 1 } else { 0 };
                            ones
                        })
                        .sum();

                    let last_rand = rands.last().unwrap();
                    ones += if last_rand & 0b00000011 == 0 { 1 } else { 0 };
                    ones += if last_rand & 0b00001100 == 0 { 1 } else { 0 };
                    ones += if last_rand & 0b00110000 == 0 { 1 } else { 0 };

                    ones
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Largest number of paralyses: {}", largest_paralysis_count);
}
