//! # 09 async/await with tokio, the standard async runtime (axum, reqwest, sqlx all use it).
//!
//! - Async is for I/O-bound concurrency (network, files, timers): thousands of tasks on a few threads.
//!   CPU-bound work goes on threads (lesson 08) or `tokio::task::spawn_blocking`.
//! - A Future is LAZY: calling an async fn does nothing until you `.await` it.
//! - `join!` runs futures concurrently in one task. `spawn` creates an independent task (must be `'static + Send`).
//! - Never block inside async (std::thread::sleep, heavy loops). That stalls every task on the thread.
//!
//! cargo run  --example 09_async
//! cargo test --example 09_async
#![allow(unused)]

use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch(id: u32, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await; // stand-in for a network call
    format!("result {id}")
}

#[tokio::main]
async fn main() {
    let t = Instant::now();
    let a = fetch(1, 100).await;
    let b = fetch(2, 200).await;
    println!("sequential: {a}, {b} in {:?}", t.elapsed()); // ~300ms

    let t = Instant::now();
    let (a, b) = tokio::join!(fetch(1, 100), fetch(2, 200));
    println!("concurrent: {a}, {b} in {:?}", t.elapsed()); // ~200ms

    // Spawn many tasks and collect results as they finish
    let mut set = tokio::task::JoinSet::new();
    for i in 0..5 {
        set.spawn(fetch(i, 50));
    }
    while let Some(res) = set.join_next().await {
        println!("{}", res.unwrap());
    }

    // select!: the first branch to finish wins and the others are cancelled (dropped)
    tokio::select! {
        r = fetch(9, 500) => println!("{r}"),
        _ = sleep(Duration::from_millis(100)) => println!("timeout!"),
    }

    // Async channel: producer/consumer
    let (tx, mut rx) = tokio::sync::mpsc::channel(8);
    tokio::spawn(async move {
        for i in 0..3 {
            tx.send(i).await.unwrap();
        }
    }); // tx dropped here -> rx.recv() returns None -> loop ends
    while let Some(v) = rx.recv().await {
        println!("got {v}");
    }
}

// ======================= EXERCISES =======================

/// Run every (id, ms) job CONCURRENTLY and return results in input order.
/// Total time should be about max(ms), not the sum. Hint: spawn all first, then await the handles in order.
async fn fetch_all(jobs: Vec<(u32, u64)>) -> Vec<String> {
    todo!()
}

/// Some(result) if the fetch finishes within `limit_ms`, else None. Hint: `tokio::time::timeout`.
async fn fetch_with_timeout(id: u32, ms: u64, limit_ms: u64) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn concurrent() {
        let t = Instant::now();
        let r = fetch_all(vec![(1, 200), (2, 100), (3, 200)]).await;
        assert_eq!(r, ["result 1", "result 2", "result 3"]);
        assert!(t.elapsed() < Duration::from_millis(400), "not concurrent: {:?}", t.elapsed());
    }

    #[tokio::test]
    async fn timeouts() {
        assert_eq!(fetch_with_timeout(1, 10, 200).await, Some("result 1".into()));
        assert_eq!(fetch_with_timeout(2, 300, 50).await, None);
    }
}
