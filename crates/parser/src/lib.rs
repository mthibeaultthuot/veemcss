#![deny(clippy::all)]

mod parser;
mod scanner;
mod writter;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
