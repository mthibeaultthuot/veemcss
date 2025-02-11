use crate::properties::background::Background;
use crate::properties::layout::Layout;
use crate::utils::regex_util::enum_to_string;
pub mod background;
pub mod layout;
use logos::Logos;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Properties<'a> {
  Background(Background<'a>),
  Layout(Layout<'a>),
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
    }

    if let Some(Ok(rule)) = Layout::lexer(token).next() {
      return Some(Properties::Layout(rule));
    }
    None
  }
  pub fn to_css(&self) -> Option<String> {
    enum_to_string(self.to_string())
  }
}
