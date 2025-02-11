use crate::parser::engine::EngineError;
use crate::parser::lexer::{Lexer, Token};
use crate::properties::Properties;
use crate::rules::breakpoint;
use core::fmt;
use std::fmt::format;

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

impl<'a> fmt::Display for TokenKind<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TokenKind::BreakpointQuery {
        breakpoint,
        children,
      } => {
        if let TokenKind::ClassName {
          class_name,
          metadata,
        } = *children.clone()
        {
          let properties = Properties::parse(class_name)
            .ok_or(fmt::Error)?
            .to_css()
            .ok_or(fmt::Error)?;
          let metadata = metadata.ok_or(fmt::Error)?;
          let base_class_name = format!(".{}\\:{}-\\[{}\\]", breakpoint, class_name, metadata);
          let new_breakpoint = Lexer::lex_breakpoint(breakpoint).unwrap();
          return write!(
            f,
            "@media only screen and (max-width: {}) {{ \n {}  {{ \n \t{} : {}; \n }} \n}}\n",
            new_breakpoint, base_class_name, properties, metadata
          );
        }
        write!(f, "@media ({}) {{ {} }}", breakpoint, children)
      }
      TokenKind::ClassName {
        class_name,
        metadata,
      } => {
        if let Some(metadata) = metadata {
          write!(f, ".{} /* {} */", class_name, metadata)
        } else {
          write!(f, ".{}", class_name)
        }
      }
    }
  }
}

pub struct Parser<'a> {
  classes: Vec<&'a str>,
  curr: usize,
}

impl<'a> Parser<'a> {
  pub fn new(classes: Vec<&'a str>) -> Self {
    Self { classes, curr: 0 }
  }

  pub fn parse(&mut self) -> Result<Vec<TokenKind<'a>>, EngineError> {
    let mut token_tree = Vec::new();
    while self.is_not_finish() {
      if let Some(result) = self.parse_next_iteration() {
        token_tree.push(result);
      };
      self.curr += 1;
    }
    Ok(token_tree)
  }

  pub fn is_not_finish(&self) -> bool {
    self.curr < self.classes.len()
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
      Some(Token::ClassName(class_name)) => {
        let mut metadata = None;
        if let Some(Token::Metadata(m)) = eval_iter.next() {
          metadata = Some(m);
        }
        let class = TokenKind::ClassName {
          class_name,
          metadata,
        };
        Some(class)
      }
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
