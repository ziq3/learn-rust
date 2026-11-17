//! # 05 Closures & iterators: idiomatic Rust uses these instead of index loops, at no runtime cost.
//!
//! - `iter()` yields &T, `iter_mut()` yields &mut T, `into_iter()` yields T (consumes the collection).
//! - Adaptors (map, filter, ...) are LAZY. Nothing runs until a consumer (collect, sum, for, ...).
//! - Closure traits: `Fn` (reads captures), `FnMut` (mutates captures), `FnOnce` (consumes captures).
//!
//! cargo run  --example 05_iterators
//! cargo test --example 05_iterators
#![allow(unused)]

use std::collections::HashMap;

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    // Closures capture their environment.
    let factor = 10;
    let scale = |x: i32| x * factor;
    println!("scale: {} {}", scale(3), apply(scale, 4));

    let mut total = 0;
    let mut add = |x: i32| total += x; // FnMut: mutates a capture
    add(1);
    add(2);
    println!("total: {total}");

    // The everyday toolbox
    let evens_sq: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    let sum: i32 = nums.iter().sum();
    let (small, big): (Vec<i32>, Vec<i32>) = nums.iter().copied().partition(|&x| x < 4);
    let any_big = nums.iter().any(|&x| x > 5);
    let pos = nums.iter().position(|&x| x == 4);
    let labeled: Vec<String> = nums
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{i}:{x}"))
        .collect();
    let product = nums.iter().fold(1, |acc, x| acc * x);
    let zipped: Vec<(i32, char)> = nums.iter().copied().zip("abc".chars()).collect();
    let diffs: Vec<i32> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    let first_big_square = (1..).map(|x| x * x).find(|&x| x > 50); // infinite range works: lazy
    println!("{evens_sq:?} {sum} {small:?}/{big:?} {any_big} {pos:?}");
    println!("{labeled:?} {product} {zipped:?} {diffs:?} {first_big_square:?}");

    // HashMap + entry API: the standard counting pattern
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for w in "a b a c b a".split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }
    println!("{counts:?}");

    // `move` makes the closure own its captures (required for threads and async tasks)
    let name = String::from("ann");
    let greet = move || format!("hi {name}");
    // println!("{name}"); // ❌ moved into the closure
    println!("{}", greet());
}

// ======================= EXERCISES (try one iterator chain each) =======================

/// Sum of squares of the odd numbers.
fn sum_odd_squares(xs: &[i32]) -> i32 {
    todo!()
}

/// Word frequency, case-insensitive. Return the top `n`, sorted by count desc, then word asc.
/// Hints: `to_lowercase()`, HashMap entry, `into_iter().collect::<Vec<_>>()`, `sort_by(|a, b| ...)`, `.cmp()`, `.then()`.
fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_squares() {
        assert_eq!(sum_odd_squares(&[1, 2, 3, 4, 5]), 1 + 9 + 25);
        assert_eq!(sum_odd_squares(&[]), 0);
    }

    #[test]
    fn top() {
        let got = top_words("the cat the dog The cat", 2);
        assert_eq!(got, [("the".to_string(), 3), ("cat".to_string(), 2)]);
        let ties = top_words("b a", 5);
        assert_eq!(ties, [("a".to_string(), 1), ("b".to_string(), 1)]);
    }
}
