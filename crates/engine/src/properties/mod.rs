use crate::properties::background::Background;
use crate::utils::regex_util::enum_to_string;
pub mod background;
pub mod layout;
use logos::Logos;
use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Properties<'a> {
  Background(Background<'a>),
}

impl fmt::Display for Properties<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl<'a> Properties<'a> {
  pub fn parse(token: &'a str) -> Option<Properties<'a>> {
    if let Some(Ok(rule)) = Background::lexer(token).next() {
      return Some(Properties::Background(rule));
    } else {
      return None;
    }
  }
  pub fn to_css(&self) -> Option<String> {
    enum_to_string(self.to_string())
  }
}
