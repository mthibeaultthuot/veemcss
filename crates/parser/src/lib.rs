#![deny(clippy::all)]

mod models;
mod parser;
mod rules;
mod scanner;
mod tokenizer;
mod writter;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
