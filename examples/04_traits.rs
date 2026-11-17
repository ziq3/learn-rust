//! # 04 Traits & generics: Rust's interfaces. There is no inheritance; you compose behavior.
//!
//! - Generics `fn f<T: Trait>(x: T)` = static dispatch: a copy per type, zero cost. Use this by default.
//! - `dyn Trait` = dynamic dispatch through a vtable. Use it for mixed-type collections or plugin-style code.
//! - Implement std traits (Display, From, Add, Iterator, ...) and std features work on your types.
//! - Common derives: Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord.
//!
//! cargo run  --example 04_traits
//! cargo test --example 04_traits
#![allow(unused)]

use std::fmt;
use std::ops::Add;

trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String {
        format!("{} makes a sound", self.name()) // default method, can be overridden
    }
}

struct Dog;
struct Cat {
    lives: u8,
}

impl Animal for Dog {
    fn name(&self) -> String {
        "Dog".into()
    }
    fn speak(&self) -> String {
        "Woof".into()
    }
}
impl Animal for Cat {
    fn name(&self) -> String {
        format!("Cat({} lives)", self.lives)
    }
}

fn intro<T: Animal>(a: &T) -> String {
    a.speak() // generic + trait bound
}
fn intro2(a: &impl Animal) -> String {
    a.speak() // same thing, shorter syntax
}
fn chorus(zoo: &[Box<dyn Animal>]) -> Vec<String> {
    zoo.iter().map(|a| a.speak()).collect() // trait objects: mixed types in one Vec
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Operator overloading is just a trait impl.
impl Add for Point {
    type Output = Point; // associated type
    fn add(self, o: Point) -> Point {
        Point {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
}

// Several bounds. Long bounds can go in a `where` clause.
fn largest<T>(xs: &[T]) -> Option<T>
where
    T: PartialOrd + Copy,
{
    let mut it = xs.iter().copied();
    let mut best = it.next()?;
    for x in it {
        if x > best {
            best = x;
        }
    }
    Some(best)
}

fn main() {
    println!("{} / {}", intro(&Dog), intro2(&Cat { lives: 9 }));
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat { lives: 3 })];
    println!("{:?}", chorus(&zoo));

    let p = Point { x: 1, y: 2 } + Point { x: 10, y: 20 };
    println!("{p} {p:?} {}", p.to_string()); // to_string() comes free with Display

    println!(
        "{:?} {:?} {:?}",
        largest(&[3, 9, 2]),
        largest(&[1.5, 0.2]),
        largest::<i32>(&[])
    );
}

// ======================= EXERCISES =======================

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}
struct Tweet {
    user: String,
    text: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        todo!() // "<title> by <author>"
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        todo!() // "@<user>: <text>"
    }
}

/// Summarize mixed items (trait objects).
fn summarize_all(items: &[&dyn Summary]) -> Vec<String> {
    todo!()
}

/// Generic: the item with the longest summary. Hint: `iter().max_by_key(...)`.
fn longest_summary<T: Summary>(items: &[T]) -> Option<&T> {
    todo!()
}

struct Celsius(f64);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!() // one decimal: "21.5°C". Hint: write!(f, "{:.1}...", ...)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn art() -> Article {
        Article {
            title: "Rust".into(),
            author: "Ferris".into(),
        }
    }
    fn tw() -> Tweet {
        Tweet {
            user: "ann".into(),
            text: "hi".into(),
        }
    }

    #[test]
    fn summaries() {
        let (a, t) = (art(), tw());
        let items: [&dyn Summary; 2] = [&a, &t];
        assert_eq!(summarize_all(&items), ["Rust by Ferris", "@ann: hi"]);
    }

    #[test]
    fn longest() {
        let tweets = [
            tw(),
            Tweet {
                user: "bob".into(),
                text: "a much longer tweet".into(),
            },
        ];
        assert_eq!(longest_summary(&tweets).unwrap().user, "bob");
        assert!(longest_summary::<Tweet>(&[]).is_none());
    }

    #[test]
    fn display() {
        assert_eq!(Celsius(21.54).to_string(), "21.5°C");
    }
}
