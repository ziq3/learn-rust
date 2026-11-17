//! # 06 Lifetimes: you don't set them. You describe how references relate.
//!
//! A lifetime annotation `'a` tells the compiler "this output borrows from that input",
//! so it can prove no reference outlives its data. Most of the time the elision rules infer it.
//! You write them when:
//!   - a fn returns a reference and has several reference inputs (which one does it borrow from?)
//!   - a struct holds a reference
//!
//! Stuck fighting the borrow checker? Own the data (String, .clone(), Rc). That's a fine choice, not cheating.
//!
//! cargo run  --example 06_lifetimes
//! cargo test --example 06_lifetimes
#![allow(unused)]

// Output may borrow from a OR b, so both must live at least as long as 'a.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Output only borrows from `text`; `needle` can be short-lived.
fn after<'a>(text: &'a str, needle: &str) -> Option<&'a str> {
    text.find(needle).map(|i| &text[i + needle.len()..])
}

// A struct holding a reference can't outlive the data it points to.
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
    // Returns &'a str (tied to the INPUT), not tied to &mut self, so tokens outlive the parser.
    fn next_token(&mut self) -> Option<&'a str> {
        let rest = self.input[self.pos..].trim_start();
        if rest.is_empty() {
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest.find(' ').map_or(self.input.len(), |i| start + i);
        self.pos = end;
        Some(&self.input[start..end])
    }
}

fn main() {
    let s1 = String::from("long string");
    {
        let s2 = String::from("xyz");
        let result = longer(&s1, &s2);
        println!("{result}"); // ✅ s2 still alive
    }
    // Moving `result` out of that block would fail: it might point into s2, which is dropped.

    let needle = String::from("=");
    let value = after("key=value", &needle);
    drop(needle); // ✅ fine, `value` borrows only from the literal
    println!("{value:?}");

    let src = String::from("let x = 5");
    let mut p = Parser::new(&src);
    while let Some(tok) = p.next_token() {
        println!("token: {tok}");
    }

    // 'static = lives for the whole program (string literals, constants)
    let lit: &'static str = "baked into the binary";
    println!("{lit}");
}

// ======================= EXERCISES =======================

/// "key: value" -> Some(("key", "value")), trimmed, borrowed from `line`. No lifetimes needed here. Why?
fn split_kv(line: &str) -> Option<(&str, &str)> {
    todo!()
}

/// Holds words borrowed from some text.
struct Words<'a> {
    words: Vec<&'a str>,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Self {
        todo!() // split_whitespace
    }
    /// Note the return type: `&'a str`, not `&str`. See the test for why that matters.
    fn longest(&self) -> Option<&'a str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kv() {
        assert_eq!(split_kv("name: ann "), Some(("name", "ann")));
        assert_eq!(split_kv("nothing"), None);
    }

    #[test]
    fn words_outlive_struct() {
        let text = String::from("a bbb cc");
        let l = {
            let w = Words::new(&text);
            w.longest() // `w` is dropped here; the result borrows `text`, not `w`
        };
        assert_eq!(l, Some("bbb"));
    }
}
