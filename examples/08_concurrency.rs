//! # 08 Concurrency: data races are compile errors.
//!
//! - `Send`: safe to MOVE to another thread. `Sync`: safe to SHARE `&T` across threads.
//!   The compiler derives these automatically. Rc is neither, so spawning with an Rc won't compile.
//! - Share state: `Arc<Mutex<T>>`. Or skip sharing and pass messages over channels.
//! - Use `thread::scope` (stable since 1.63) when threads only need to borrow local data.
//! - Real projects: `rayon` makes data parallelism a one-line change (`.par_iter()`).
//!
//! cargo run  --example 08_concurrency
//! cargo test --example 08_concurrency
#![allow(unused)]

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    // 1. spawn + join. `move` hands ownership of `data` to the thread.
    let data = vec![1, 2, 3];
    let h = thread::spawn(move || data.iter().sum::<i32>());
    println!("sum = {}", h.join().unwrap());

    // 2. Scoped threads borrow local data. No Arc or move needed; all threads join at scope end.
    let nums: Vec<i32> = (1..=100).collect();
    let (left, right) = nums.split_at(50);
    let (a, b) = thread::scope(|s| {
        let ha = s.spawn(|| left.iter().sum::<i32>());
        let hb = s.spawn(|| right.iter().sum::<i32>());
        (ha.join().unwrap(), hb.join().unwrap())
    });
    println!("scoped: {a} + {b} = {}", a + b);

    // 3. Shared mutable state
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    *c.lock().unwrap() += 1; // lock released when the guard drops (end of statement)
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("counter = {}", counter.lock().unwrap()); // 8000, always

    // 4. Message passing
    let (tx, rx) = mpsc::channel();
    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || tx.send(format!("worker {id} done")).unwrap());
    }
    drop(tx); // drop the original sender so the receiver loop ends
    for msg in rx {
        println!("{msg}");
    }
}

// ======================= EXERCISES =======================

/// Split `xs` into `chunks` pieces and sum each piece on its own thread (thread::scope).
/// Hints: `xs.chunks(size)`, `size = xs.len().div_ceil(chunks).max(1)`.
fn parallel_sum(xs: &[u64], chunks: usize) -> u64 {
    todo!()
}

/// Spawn `n` threads; each pushes its index into a shared Vec. Return the Vec sorted.
/// Hint: Arc<Mutex<Vec<usize>>>; after joining, lock and clone, or `Arc::try_unwrap`.
fn collect_ids(n: usize) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn psum() {
        let xs: Vec<u64> = (1..=1000).collect();
        assert_eq!(parallel_sum(&xs, 4), 500500);
        assert_eq!(parallel_sum(&xs, 7), 500500);
        assert_eq!(parallel_sum(&[], 3), 0);
    }

    #[test]
    fn ids() {
        assert_eq!(collect_ids(5), [0, 1, 2, 3, 4]);
    }
}
