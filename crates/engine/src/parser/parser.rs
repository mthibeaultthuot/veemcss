use crate::parser::lexer::{Lexer, Token};
use crate::scanner::scanner::Scanner;

#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
  BreakpointQuery {
    breakpoint: &'a str,
    children: Box<TokenKind<'a>>,
  },
  ClassName {
    class_name: &'a str,
    metadata: Option<&'a str>,
  },
}

pub struct Parser<'a> {
  classes: Vec<&'a str>,
  curr: usize,
}

impl<'a> Parser<'a> {
  pub fn new(code: &'a str) -> Self {
    let classes = Scanner::scan(code).unwrap();
    Self { classes, curr: 0 }
  }

  pub fn parse(&mut self) -> Result<Vec<TokenKind<'a>>, std::fmt::Error> {
    let mut token_tree = Vec::new();
    while !self.is_finish() {
      if let Some(result) = self.parse_next_iteration() {
        token_tree.push(result);
        self.curr += 1;
      };
    }
    Ok(token_tree)
  }

  pub fn is_finish(&self) -> bool {
    self.curr >= self.classes.len() - 1
  }

  pub fn parse_next_iteration(&self) -> Option<TokenKind<'a>> {
    let curr_evaluation = Lexer::lex(self.classes[self.curr]).unwrap();
    let mut eval_iter = curr_evaluation.into_iter();

    let result = match eval_iter.next() {
      Some(Token::Breakpoint(breakpoint)) => {
        let classe = self.parse_classe(&mut eval_iter);
        match classe {
          Some(classe) => {
            let tok = TokenKind::BreakpointQuery {
              breakpoint,
              children: Box::new(classe),
            };
            Some(tok)
          }
          None => None,
        }
      }
      Some(Token::ClassName(_s)) => None,
      Some(Token::Metadata(_metdata)) => None,
      None => None,
    };

    result
  }

  pub fn parse_classe(
    &self,
    mut eval_iter: impl Iterator<Item = Token<'a>>,
  ) -> Option<TokenKind<'a>> {
    let item = eval_iter.next();

    if let Some(Token::ClassName(i)) = item {
      let mut metadata = None;
      if let Some(Token::Metadata(m)) = eval_iter.next() {
        metadata = Some(m);
      }

      return Some(TokenKind::ClassName {
        class_name: i,
        metadata,
      });
    };
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let code = r#"<div class="md:w-[100px] md:bg-[#333]"></div>"#;
    let mut parser = Parser::new(code);
    let parse_result = parser.parse().unwrap();
    println!("{:?}", parse_result);
  }
}
