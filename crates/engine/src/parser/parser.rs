use crate::parser::lexer::{Lexer, Token};
use crate::scanner::scanner::Scanner;

#[derive(Debug, Clone)]
pub struct Tree<'a> {
  nodes: Vec<Node<'a>>,
}

impl<'a> Tree<'a> {
  pub fn new() -> Self {
    Self { nodes: Vec::new() }
  }

  pub fn push(&mut self, node: Node<'a>) -> &Node<'a> {
    self.nodes.push(node);
    self.nodes.last().unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
  value: TokenKind<'a>,
  next: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
  pub fn new(value: TokenKind<'a>, next: Option<Box<Node<'a>>>) -> Self {
    Self { value, next }
  }

  pub fn push_next(&mut self, next: Node<'a>) {
    self.next = Some(Box::new(next));
  }
}

#[derive(Debug, Clone)]
pub enum TokenKind<'a> {
  BreakpointQuery {
    breakpoint: &'a str,
    children: Vec<TokenKind<'a>>,
  },
  ClassName {
    class_name: &'a str,
    child: Option<Box<TokenKind<'a>>>,
  },
  MetaData(&'a str),
}

pub struct Parser<'a> {
  pub code: &'a str,
}

impl<'a> Parser<'a> {
  pub fn new(code: &'a str) -> Self {
    Self { code }
  }

  pub fn parse(&self) -> Result<Vec<TokenKind>, std::fmt::Error> {
    let mut token_tree: Vec<TokenKind> = Vec::new();
    let classes = Scanner::scan(self.code).unwrap();
    let mut last_token_index: Option<usize> = None;
    for classe in &classes {
      let tokens = Lexer::lex(classe).unwrap();
      for token in tokens {
        match token {
          Token::Breakpoint(s) => {
            let new_token = TokenKind::BreakpointQuery {
              breakpoint: s,
              children: Vec::new(),
            };

            token_tree.push(new_token.clone());
            last_token_index = Some(token_tree.len() - 1);
          }
          Token::ClassName(s) => {
            let new_token = TokenKind::ClassName {
              class_name: s,
              child: None,
            };
            if let Some(index) = last_token_index {
              if let TokenKind::BreakpointQuery { children, .. } = &mut token_tree[index] {
                children.push(new_token.clone())
              } else {
                token_tree.push(new_token)
              }
            }
            last_token_index = Some(token_tree.len() - 1);
          }
          Token::Metadata(s) => {
            let new_token = TokenKind::MetaData(s);
            if let Some(index) = last_token_index {
              if let TokenKind::ClassName { ref mut child, .. } = token_tree[index] {
                *child = Some(Box::new(new_token.clone()));
              }
            }
          }
        };
      }
    }

    Ok(token_tree)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let code = r#"<div class="md:w-[100px] md:bg-[#333]"></div>"#;
    let parser = Parser::new(code);
    let parse_result = parser.parse().unwrap();
    println!("{:?}", parse_result);
  }
}
