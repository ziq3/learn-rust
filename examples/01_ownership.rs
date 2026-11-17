//! # 01 Ownership & borrowing: the core idea of Rust. Learn this well and the rest gets easier.
//!
//! Three rules:
//! 1. Every value has exactly ONE owner.
//! 2. When the owner goes out of scope, the value is dropped (freed). No GC, no manual free.
//! 3. You can borrow: EITHER many `&T` (shared, read-only) OR one `&mut T` (exclusive). Never both at once.
//!
//! "Copy" types (i32, f64, bool, char, &T) are copied instead of moved.
//!
//! cargo run  --example 01_ownership
//! cargo test --example 01_ownership
#![allow(unused)]

fn main() {
    // --- Move: assigning or passing a heap value transfers ownership ---
    let a = String::from("hello");
    // `a` is no longer valid after this line:
    let b = a;
    // println!("{a}"); // ❌ borrow of moved value
    println!("b = {b}");

    // --- Clone: explicit deep copy (allocates, so Rust makes you write it) ---
    let c = b.clone();
    println!("b = {b}, c = {c}");

    // --- Borrow: lend a reference, keep ownership ---
    println!("{c} has length {}", length(&c));

    // --- Mutable borrow ---
    let mut s = String::from("hi");
    shout(&mut s);
    println!("{s}");

    // --- Why the borrow rule matters ---
    let mut v = vec![1, 2, 3];
    // shared borrow into v's buffer:
    let first = &v[0];
    // v.push(4); // ❌ push may reallocate, which would leave `first` dangling
    println!("first = {first}"); // last use of `first` -> borrow ends here
    v.push(4); // ✅ fine now
    println!("{v:?}");

    // --- Taking ownership ---
    let owned = String::from("bye");
    consume(owned);
    // consume(owned); // ❌ already moved

    // --- String vs &str ---
    // String: owned, growable heap buffer. &str: borrowed view into string data.
    // Take `&str` in parameters: accepts &String AND literals.
    println!("{}", first_word("hello world"));
}

fn length(s: &str) -> usize {
    s.len()
}
fn shout(s: &mut String) {
    s.push_str("!!!");
}
fn consume(s: String) {
    println!("consumed {s}");
} // s dropped here
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

// ======================= EXERCISES: replace todo!() until tests pass =======================

/// Return the longest word (first one on ties). Return a slice of `s`, don't allocate.
fn longest_word(s: &str) -> &str {
    let mut best = "";
    for w in s.split_whitespace() {
        if best.len() < w.len() {
            best = w;
        }
    }
    best
}

/// Append "!" to every string, in place.
fn excite(words: &mut Vec<String>) {
    for w in words {
        w.push_str("!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest() {
        assert_eq!(longest_word("a bbb cc ddd"), "bbb");
        assert_eq!(longest_word(""), "");
    }

    #[test]
    fn excite_in_place() {
        let mut v = vec!["hi".to_string(), "yo".to_string()];
        excite(&mut v);
        assert_eq!(v, ["hi!", "yo!"]);
    }
}
