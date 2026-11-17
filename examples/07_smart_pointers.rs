//! # 07 Smart pointers: pick one based on what you need.
//!
//! | Need                                          | Use                          |
//! |-----------------------------------------------|------------------------------|
//! | heap allocation, recursive type, trait object | `Box<T>`                     |
//! | several owners, one thread                    | `Rc<T>` (reference counted)  |
//! | several owners, across threads                | `Arc<T>` (atomic Rc)         |
//! | mutate through a shared ref, one thread       | `RefCell<T>` (checked at RUNTIME, panics on violation) |
//! | mutate through a shared ref, across threads   | `Mutex<T>` / `RwLock<T>`     |
//! | shared + mutable                              | `Rc<RefCell<T>>` / `Arc<Mutex<T>>` |
//! | parent<->child back-references (avoid cycles) | `Weak<T>`                    |
//!
//! Rule of thumb: plain ownership first. Reach for these only when the compiler says the design needs it.
//!
//! cargo run  --example 07_smart_pointers
//! cargo test --example 07_smart_pointers
#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

// Recursive type: without Box the size would be infinite.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{list:?}");

    // Rc: shared ownership. clone() is cheap (just increments a counter).
    let config = Rc::new(String::from("db=postgres"));
    let a = Rc::clone(&config);
    let b = Rc::clone(&config);
    println!("{a} owners={}", Rc::strong_count(&config)); // 3
    drop(b);
    println!("owners={}", Rc::strong_count(&config)); // 2

    // RefCell: interior mutability. borrow() / borrow_mut() are checked at runtime.
    let log = Rc::new(RefCell::new(Vec::<String>::new()));
    let writer = Rc::clone(&log);
    writer.borrow_mut().push("hello".into());
    writer.borrow_mut().push("world".into());
    println!("{:?}", log.borrow());
    // let r = log.borrow(); let w = log.borrow_mut(); // 💥 panics: already borrowed
}

// ======================= EXERCISES =======================

/// Sum a Box-based list recursively. Hint: match on `list`, `Cons(v, rest)`.
fn sum(list: &List) -> i32 {
    todo!()
}

type Ledger = Rc<RefCell<Vec<String>>>;

/// Several accounts share one ledger.
struct Account {
    name: String,
    balance: i64,
    ledger: Ledger,
}

impl Account {
    fn new(name: &str, ledger: &Ledger) -> Self {
        Self {
            name: name.into(),
            balance: 0,
            ledger: Rc::clone(ledger),
        }
    }
    /// Add to balance AND push "<name>+<amount>" into the shared ledger.
    fn deposit(&mut self, amount: i64) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_sum() {
        let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        assert_eq!(sum(&l), 6);
        assert_eq!(sum(&Nil), 0);
    }

    #[test]
    fn shared_ledger() {
        let ledger: Ledger = Rc::new(RefCell::new(vec![]));
        let mut ann = Account::new("ann", &ledger);
        let mut bob = Account::new("bob", &ledger);
        ann.deposit(10);
        bob.deposit(5);
        ann.deposit(1);
        assert_eq!(ann.balance, 11);
        assert_eq!(*ledger.borrow(), ["ann+10", "bob+5", "ann+1"]);
        assert_eq!(Rc::strong_count(&ledger), 3);
    }
}
