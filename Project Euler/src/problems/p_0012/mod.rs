//! **Problem 12** - *Highly Divisible Triangular Number*
use crate::shared::structures::Problem;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(
        12,
        "Highly Divisible Triangular Number",
        solve,
    )
}


use crate::shared::math::num_of_divisors;

fn solve() -> String {
    // triangular numbers are natural numbers generated by the formula:
    // n-th triangular number = 1 + 2 + 3 + 4 + ... + n = n(n+1)/2
    // every natural number can be written as a product of prime numbers
    // n = p1^a1 * p2^a2 * p3^a3 * ... * pn^an
    // let's define a function d(n) that returns the number of divisors of n
    // it is obvious that d(n) = (a1 + 1) * (a2 + 1) * (a3 + 1) * ... * (an + 1)
    // (for every prime factor pi we can choose how many times to include it, lowest being 0, highest being ai)
    // (now we just multiply all the choices together to get the total number of divisors)
    // let x be the n-th triangular number
    // x(n) = n(n+1)/2
    // d(x(n)) = d(n/2) * d(n+1) if n is even
    // d(x(n)) = d(n) * d((n+1)/2) if n is odd
    // this is because n and n+1 are coprime, so d(n(n+1)) = d(n) * d(n+1)
    // also:
    // d(x(n+1)) = d((n+1)/2) * d(n+2) if n+1 is even
    // d(x(n+1)) = d(n+1) * d((n+2)/2) if n is odd
    // we can see that for each number x, d(x(n)) contains two terms that need to be multiplied
    // but since n is alternating between even and odd, the first term of d(x(n+1)) is the second term of d(x(n))
    // we can reuse that term and only calculate the second term of d(x(n+1))

    // let's start from n = 5
    let mut n = 5;
    // since 5 is odd, we can calculate initial values of d_t1 and d_t2
    let mut d_t1: u64 = num_of_divisors(n);
    let mut d_t2: u64 = num_of_divisors((n + 1) / 2);

    while d_t1 * d_t2 <= 500 {
        n += 1;
        d_t1 = d_t2;
        if n % 2 == 0 {
            d_t2 = num_of_divisors(n + 1);
        } else {
            d_t2 = num_of_divisors((n + 1) / 2);
        }
    }

    (n * (n + 1) / 2).to_string()
}
