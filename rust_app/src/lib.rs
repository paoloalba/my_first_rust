#![allow(unused)]

mod module_1;

pub use crate::module_1::sub_module_1;

use pyo3::prelude::*;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

use chrono::{DateTime, Utc};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

#[pyfunction]
fn main() {
    sub_module_1::what_a_function();

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("End of main: {}", Utc::now().format("%d-%m-%Y %H:%M:%S"));    
}
#[pymodule]
fn rust_app(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;

    Ok(())
}
