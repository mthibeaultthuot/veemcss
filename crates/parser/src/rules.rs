use logos::{Lexer, Logos, Source};
use regex::bytes::Regex;
use regex::Regex as OtherRegex;
use std::fmt;
use std::io::Read;

#[derive(Clone, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum CSSRules<'a> {
  /// background definition
  #[token("bg", |_| "background")]
  Background(&'a str),

  /// padding
  #[token("p", |_| "padding")]
  Padding(&'a str),
}

impl fmt::Display for CSSRules<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn get_rule(token: &str) -> Option<String> {
  let mut lex = CSSRules::lexer(token);
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
}
