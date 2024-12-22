use rayon::{iter::ParallelIterator as _, str::ParallelString};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

pub fn part1(input: &str) -> u64 {
    input
        .par_lines()
        .map(|line| line.parse().unwrap())
        .map(|mut num| {
            for _ in 0..2000 {
                num = next_secret_number(num);
            }
            num
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let slice: Arc<_> = (0..19usize.pow(4)).map(|_| AtomicU64::new(0)).collect();

    input
        .par_lines()
        .map(|line| (line.parse().unwrap(), slice.clone()))
        .for_each(|(mut num, slice)| {
            let mut sequence = [0u64; (19usize.pow(4) >> 6) + 1];
            let mut last_4 = 0;
            let mut old_price = num % 10;

            for i in 0..2000 {
                let new = next_secret_number(num);
                let new_price = new % 10;

                let diff = ((old_price + 9) - new_price) as usize;
                last_4 = (last_4 * 19 + diff) % 19usize.pow(4);

                if i >= 3 {
                    let div = last_4 >> 6;
                    let rem = last_4 & 63;
                    let flag = 1 << rem;

                    if sequence[div] & flag == 0 {
                        slice[last_4].fetch_add(new_price, Ordering::Relaxed);
                        sequence[div] |= flag;
                    }
                }

                num = new;
                old_price = new_price;
            }
        });

    slice
        .iter()
        .map(|s| s.load(Ordering::Relaxed))
        .max()
        .unwrap()
}

fn next_secret_number(mut n: u64) -> u64 {
    n = prune(mix(n, n << 6));
    n = prune(mix(n >> 5, n));
    n = prune(mix(n << 11, n));
    n
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(n: u64) -> u64 {
    n & 16_777_215
}
