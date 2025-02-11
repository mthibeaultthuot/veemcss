//use crate::utils::regex_util::enum_to_string;
use logos::Logos;
//use std::fmt;

#[derive(Clone, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum BreakpointRule<'a> {
  #[token("sm", |_| "640px")]
  Small(&'a str),
  #[token("md", |_| "768px")]
  Medium(&'a str),
  #[token("lg", |_| "1024px")]
  Large(&'a str),
  #[token("xl", |_| "1280px")]
  XLarge(&'a str),
  #[token("xxl", |_| "1536px")]
  XXLarge(&'a str),
}
/*
impl fmt::Display for BreakpointRule<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl<'a> BreakpointRule<'a> {
  pub fn parse(token: &'a str) -> Option<BreakpointRule<'a>> {
    if let Some(Ok(rule)) = BreakpointRule::lexer(token).next() {
      return Some(rule);
    } else {
      return None;
    }
  }
  pub fn to_css(&self) -> Option<String> {
    enum_to_string(self.to_string())
  }
}*/
