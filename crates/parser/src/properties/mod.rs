use crate::properties::background::BackgroundPro;
pub mod background;
pub mod layout;
use logos::Logos;
use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Properties<'a> {
  Background(BackgroundPro<'a>),
}

impl fmt::Display for Properties<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl<'a> Properties<'a> {
  pub fn parse(token: &'a str) -> Option<Properties<'a>> {
    if let Some(Ok(rule)) = BackgroundPro::lexer(token).next() {
      return Some(Properties::Background(rule));
    } else {
      return None;
    }
  }
  pub fn to_css(&self) -> Option<String> {
    let re = Regex::new(r#""(.*?)""#).unwrap();
    let string_enum = self.to_string().to_owned();
    let str_enum = &string_enum[..];
    if let Some(caps) = re.captures(str_enum) {
      match std::str::from_utf8(&caps[1].as_bytes()) {
        Ok(valid_str) => return Some(valid_str.to_string()),
        Err(_) => {}
      };
    }
    None
  }
}

/*
pub fn get_rule(token: &str) -> Option<String> {
  let mut lex = BackgroundPro::lexer(token);
  let n = lex.next().unwrap();
  match n {
    Ok(css) => {
      let re = Regex::new(r#""(.*?)""#).unwrap();
      if let Some(caps) = re.captures(&*css.to_string().as_bytes()) {
        let matched = &caps[1];
        match std::str::from_utf8(matched) {
          Ok(valid_str) => Some(valid_str.to_string()),
          Err(_) => None,
        }
      } else {
        None
      }
    }
    Err(_) => None,
  }
}*/
