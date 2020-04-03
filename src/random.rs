use rand::Rng;
use rand_distr::{Distribution, Normal};

/// Return a random number in the [a, b) interval with equal probability.
pub fn range<T>(a: T, b: T) -> T
where
    T: rand::distributions::uniform::SampleUniform,
{
    rand::thread_rng().gen_range(a, b)
}

/// Roll a dice of n sides a certain number of times and return the accumulated result.
pub fn roll(n: u32, times: u32) -> u32 {
    if n == 0 || times == 0 {
        return 0;
    }
    let mut total = 0;
    for _ in 0..times {
        total += range(1, n + 1);
    }
    total
}

/// Return true with a 1 in n probability
pub fn one_in(n: u32) -> bool {
    if n < 2 {
        return true;
    }
    range(1, n + 1) == 1
}

/// Return a random number generated from a normal distribution with the
/// given mean and standard deviation
pub fn normal(mean: f64, stdev: f64) -> f64 {
    Normal::new(mean, stdev)
        .unwrap()
        .sample(&mut rand::thread_rng())
}

/// Return a reference to a random item from a slice, or None if the slice is empty
pub fn choice<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    Some(&slice[range(0, slice.len())])
}
