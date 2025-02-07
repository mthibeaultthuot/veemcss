#![deny(clippy::all)]

mod parser;
mod properties;
mod rules;
mod scanner;
mod utils;

#[macro_use]
extern crate napi_derive;
