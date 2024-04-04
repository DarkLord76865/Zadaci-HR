//! **Problem 21** - *Amicable Numbers*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(21, "Amicable Numbers", solve)
}

use crate::shared::math::sum_of_proper_divisors;

const MAX: usize = 10_000;

fn solve() -> String {
    // we need to check numbers 1 to 9999 (inclusive)
    // the sums of proper divisors will be stored for each number

    // we will use a vector to store the sums of proper divisors
    // index 0 will be ignored
    let mut sums = vec![0; MAX];

    // calculate the sums of proper divisors for each number
    for (i, store_pos) in sums.iter_mut().enumerate().skip(1) {
        *store_pos = sum_of_proper_divisors(i as u64);
    }

    // now it is checked whether the number is amicable, and if it is, it is added to the sum
    let mut sum = 0;
    for (i, sum_of_divisors) in sums.iter().enumerate().skip(1) {
        if (*sum_of_divisors < MAX as u64) && (sums[*sum_of_divisors as usize] == i as u64) && (i as u64 != *sum_of_divisors) {
            sum += i as u64;
        }
    }

    sum.to_string()
}
