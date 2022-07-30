#![allow(unused)]

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use pyo3::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[pyfunction]
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[pyfunction]
fn test_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

use std::thread;
use std::time::Duration;

#[derive(Debug)]
#[pyclass]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}

#[pyfunction]
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Penny);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let mut v = Vec::new();

    v.push(String::from("bulo"));
    v.push(String::from("ji"));    

    let v = vec![1, 2, 3, 4, 5];

    let third = v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
        
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let result;
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

            
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    // if x.len() > y.len() {
    //     x
    // } else {
    //     y
    // }
    x
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust_app(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;
    m.add_function(wrap_pyfunction!(test_loop, m)?)?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_class::<UsState>()?;

    Ok(())
}
