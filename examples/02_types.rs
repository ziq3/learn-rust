//! # 02 Structs, enums, pattern matching: design types so invalid states can't exist.
//!
//! - struct = AND (has a name AND an age). enum = OR (Cash OR Card OR Crypto), and each variant can carry data.
//! - There is no null. "Maybe absent" is `Option<T>`: `Some(x)` or `None`.
//! - `match` must cover every case. Add a variant and the compiler lists every place to update.
//!
//! cargo run  --example 02_types
//! cargo test --example 02_types
#![allow(unused)]

#[derive(Debug, Clone, PartialEq)] // derive = auto-implement common traits
struct User {
    name: String,
    age: u32,
}

impl User {
    // A "constructor" is just an associated fn named `new` by convention.
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
    fn is_adult(&self) -> bool {
        self.age >= 18 // &self = read-only borrow
    }
    fn birthday(&mut self) {
        self.age += 1; // &mut self = mutable borrow
    }
}

#[derive(Debug)]
enum Payment {
    Cash,
    Card { last4: String },
    Crypto(String, f64),
}

fn describe(p: &Payment) -> String {
    match p {
        Payment::Cash => "cash".to_string(),
        Payment::Card { last4 } => format!("card ****{last4}"),
        Payment::Crypto(coin, amt) if *amt > 1.0 => format!("big {coin} payment"), // match guard
        Payment::Crypto(coin, _) => coin.clone(),
    }
}

fn age_of(users: &[User], name: &str) -> Option<u32> {
    for u in users {
        if u.name == name {
            return Some(u.age);
        }
    }
    None
}

fn main() {
    let mut u = User::new("ann", 17);
    u.birthday();
    println!("{u:?} adult={}", u.is_adult());

    let payments = [
        Payment::Cash,
        Payment::Card {
            last4: "4242".into(),
        },
        Payment::Crypto("BTC".into(), 2.0),
    ];
    for p in &payments {
        println!("{}", describe(p));
    }

    let users = vec![u.clone(), User::new("bob", 30)];

    // Three ways to unpack an Option:
    match age_of(&users, "bob") {
        Some(age) => println!("bob is {age}"),
        None => println!("no bob"),
    }
    if let Some(age) = age_of(&users, "ann") {
        println!("ann is {age}");
    }
    let Some(age) = age_of(&users, "zed") else {
        println!("zed not found, early return");
        return;
    };
    println!("unreachable: {age}");
}

// ======================= EXERCISES =======================

#[derive(Debug, PartialEq)]
enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } => w * h,
            Shape::Square(s) => s * s,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Say(String),
}

/// "quit" -> Quit, "move 3 -4" -> Move{3,-4}, "say hello world" -> Say("hello world"), anything else -> None.
/// Hints: `split_once(' ')`, `str::parse::<i32>().ok()?`, matching on tuples `match (a, b) { ... }`.
fn parse(input: &str) -> Option<Command> {
    if input == "quit" {
        return Some(Command::Quit);
    }
    let (cmd, rest) = input.split_once(' ')?;
    if cmd == "move" {
        let (a, b) = rest.split_once(' ')?;
        let x = a.parse::<i32>().ok()?;
        let y = b.parse::<i32>().ok()?;
        return Some(Command::Move { x, y });
    }
    if cmd == "say" {
        return Some(Command::Say(rest.to_string()));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas() {
        assert!((Shape::Circle { r: 1.0 }.area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(Shape::Rect { w: 2.0, h: 3.0 }.area(), 6.0);
        assert_eq!(Shape::Square(2.0).area(), 4.0);
    }

    #[test]
    fn commands() {
        assert_eq!(parse("quit"), Some(Command::Quit));
        assert_eq!(parse("move 3 -4"), Some(Command::Move { x: 3, y: -4 }));
        assert_eq!(
            parse("say hello world"),
            Some(Command::Say("hello world".into()))
        );
        assert_eq!(parse("move x 1"), None);
        assert_eq!(parse("dance"), None);
    }
}
