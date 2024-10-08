//! **Problem 61** - *Cyclical Figurate Numbers*

use crate::shared::structures::Problem;
use itertools::Itertools;
use std::collections::HashMap;

/// Get `Problem` struct.
pub fn get_problem() -> Problem {
    Problem::new(61, "Cyclical Figurate Numbers", solve)
}

const MIN_4_DIGIT: u16 = 1000;
const MAX_4_DIGIT: u16 = 9999;

fn solve() -> String {
    // generate all 4-digit numbers for each cyclical figurate number type

    let triangle = generate_numbers(
        || {
            let first_n = ((-1.0 + (1.0f64 - 4.0 * 1.0 * -2.0 * MIN_4_DIGIT as f64).sqrt()) / 2.0).ceil() as u16;
            let first_value = first_n * (first_n + 1) / 2;
            (first_n, first_value)
        },
        |n, value| value + n + 1,
    );
    let square = generate_numbers(
        || {
            let first_n = (MIN_4_DIGIT as f64).sqrt().ceil() as u16;
            let first_value = first_n * first_n;
            (first_n, first_value)
        },
        |n, value| value + 2 * n + 1,
    );
    let pentagonal = generate_numbers(
        || {
            let first_n = ((1.0 + (1.0f64 - 4.0 * 3.0 * -2.0 * MIN_4_DIGIT as f64).sqrt()) / 6.0).ceil() as u16;
            let first_value = first_n * (3 * first_n - 1) / 2;
            (first_n, first_value)
        },
        |n, value| value + 3 * n + 1,
    );
    let hexagonal = generate_numbers(
        || {
            let first_n = ((1.0 + (1.0f64 - 4.0 * 2.0 * -(MIN_4_DIGIT as f64)).sqrt()) / 4.0).ceil() as u16;
            let first_value = first_n * (2 * first_n - 1);
            (first_n, first_value)
        },
        |n, value| value + 4 * n + 1,
    );
    let heptagonal = generate_numbers(
        || {
            let first_n = ((3.0 + (9.0f64 - 4.0 * 5.0 * -2.0 * MIN_4_DIGIT as f64).sqrt()) / 10.0).ceil() as u16;
            let first_value = first_n * (5 * first_n - 3) / 2;
            (first_n, first_value)
        },
        |n, value| value + 5 * n + 1,
    );
    let octagonal = generate_numbers(
        || {
            let first_n = ((2.0 + (4.0f64 - 4.0 * 3.0 * -(MIN_4_DIGIT as f64)).sqrt()) / 6.0).ceil() as u16;
            let first_value = first_n * (3 * first_n - 2);
            (first_n, first_value)
        },
        |n, value| value + 6 * n + 1,
    );

    // we need to find 6 numbers, one from each type, that form a cyclical chain
    // therefore we can lock the first number to be a triangle number,
    // and try to find a chain of numbers that are cyclical

    // generate maps for each type of number, where the key is the first two digits of the number,
    // and the value is a list of numbers of that type that start with those two digits
    // this will allow us to quickly find the next number in the chain, if it exists

    let square_map = generate_map(&square);
    let pentagonal_map = generate_map(&pentagonal);
    let hexagonal_map = generate_map(&hexagonal);
    let heptagonal_map = generate_map(&heptagonal);
    let octagonal_map = generate_map(&octagonal);

    // generates all permutations, but some could be skipped (that have the same start)!!!
    let perms = [&square_map, &pentagonal_map, &hexagonal_map, &heptagonal_map, &octagonal_map]
        .into_iter()
        .permutations(5)
        .collect_vec();
    let mut stack = Vec::with_capacity(6);
    let mut iter_stack = Vec::with_capacity(5);

    'outer: for n in triangle {
        stack.push(n);

        for perm in perms.iter() {
            match perm[0].get(&(stack[0] % 100)) {
                Some(vec) => {
                    iter_stack.push(vec.iter());
                    stack.push(*iter_stack[0].next().unwrap());
                }
                None => continue,
            }
            while !iter_stack.is_empty() {
                if iter_stack.len() == 5 && stack.len() == 6 {
                    if stack.last().unwrap() % 100 == stack[0] / 100 {
                        break 'outer;
                    } else {
                        stack.pop();
                        if let Some(val) = iter_stack.last_mut().unwrap().next() {
                            stack.push(*val);
                        }
                    }
                } else if stack.len() == iter_stack.len() {
                    iter_stack.pop();
                    if let Some(last_item) = iter_stack.last_mut() {
                        stack.pop();
                        if let Some(val) = last_item.next() {
                            stack.push(*val);
                        }
                    }
                } else {
                    match perm[iter_stack.len()].get(&(stack.last().unwrap() % 100)) {
                        Some(vec) => {
                            iter_stack.push(vec.iter());
                            stack.push(*iter_stack.last_mut().unwrap().next().unwrap());
                        }
                        None => {
                            stack.pop();
                            if let Some(val) = iter_stack.last_mut().unwrap().next() {
                                stack.push(*val);
                            }
                        }
                    }
                }
            }
        }

        stack.pop();
    }

    // TODO: Write better recursive solution, this tries all permutations unnecessarily

    stack.into_iter().map(|n| n as u32).sum::<u32>().to_string()
}

/// Given a list of numbers, generate a map where the key is the first two digits of the number,
/// and the value is a list of numbers (from the original list) that start with those two digits.
fn generate_map(nums: &[u16]) -> HashMap<u16, Vec<u16>> {
    let mut map: HashMap<u16, Vec<u16>> = HashMap::new();
    for &num in nums {
        let key = num / 100;
        if let Some(vec) = map.get_mut(&key) {
            vec.push(num);
        } else {
            map.insert(key, vec![num]);
        }
    }
    map
}

/// Generate a list of 4 digit values based on the given functions.
/// # Arguments
/// * `first` - A function that returns the first n and corresponding first value.
/// * `next` - A function that returns the next value based on the current n and value.
fn generate_numbers<T, U>(first: T, next: U) -> Vec<u16>
where
    T: Fn() -> (u16, u16),
    U: Fn(u16, u16) -> u16,
{
    let mut numbers = Vec::new();

    let (n, mut value) = first();
    numbers.push(value);
    for n in n.. {
        value = next(n, value);
        if value > MAX_4_DIGIT {
            break;
        }
        numbers.push(value);
    }

    numbers
}
