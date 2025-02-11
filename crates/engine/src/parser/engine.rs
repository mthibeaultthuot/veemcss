use napi::JsError;

use crate::parser::generator::Generator;
use crate::parser::lexer::Lexer;
use crate::parser::lexer::Token;
use crate::parser::parser::Parser;
use crate::properties::Properties;
use crate::scanner::scanner::Scanner;
use std::fmt;

#[derive(Debug)]
pub enum EngineError {
  LexerNotFound,
}

impl fmt::Display for EngineError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      EngineError::LexerNotFound => write!(f, "Lexer not found"),
    }
  }
}

impl From<EngineError> for napi::Error {
  fn from(err: EngineError) -> Self {
    napi::Error::new(napi::Status::GenericFailure, format!("{}", err))
  }
}

impl From<EngineError> for JsError {
  fn from(err: EngineError) -> Self {
    napi::Error::new(napi::Status::GenericFailure, format!("{}", err)).into()
  }
}

#[napi]
pub struct Engine {}
#[napi]
impl Engine {
  #[napi]
  pub fn from_string(code: String) -> Result<String, EngineError> {
    let classes_scanner = Scanner::scan(code.as_str()).unwrap();
    let filtered_classes = Engine::filter_classes(classes_scanner)?;

    let mut parser = Parser::new(filtered_classes);

    let tree_result = parser.parse()?;

    let mut generator = Generator::new(tree_result);
    let output_css = generator.generate_css();

    Ok(output_css)
  }

  pub fn filter_classes(classes: Vec<&str>) -> Result<Vec<&str>, EngineError> {
    let mut filtered_classes = Vec::new();
    let breakpoints = vec!["sm", "md", "lg", "xl", "2xl"];
    let mut i = 0;
    loop {
      if i > breakpoints.len() - 1 {
        break;
      }
      let curr_breakpoint = breakpoints[i];

      let mut find_classes = classes
        .iter()
        .filter(|classe| classe.contains(curr_breakpoint) && Engine::check_classe_exist(classe))
        .cloned()
        .collect::<Vec<_>>();
      filtered_classes.append(&mut find_classes);

      i += 1;
    }

    let remaining_classes: Vec<&str> = classes
      .iter()
      .filter(|classe| {
        !breakpoints.iter().any(|bp| classe.contains(bp)) && Engine::check_classe_exist(classe)
      })
      .cloned()
      .collect();

    filtered_classes.append(&mut remaining_classes.to_vec());
    Ok(filtered_classes)
  }

  pub fn check_classe_exist(classe: &str) -> bool {
    let lexer = Lexer::lex(classe);
    match lexer {
      Ok(tokens) => tokens.iter().any(|token| {
        if let Token::ClassName(classe_name) = token {
          let i = Properties::parse(classe_name).is_some();
          i
        } else {
          false
        }
      }),
      Err(_err) => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_string() {
    let code = r#"<div class="background-white md:bg-[#111]"></div>"#;
    Engine::from_string(code).unwrap();
  }

  #[test]
  fn filter_class() {
    //let classes = vec!["bg-[#000]", "md:bg-[#333]", "md:w-[100px]", "sm:bg-[#222]"];
    //let filtered_classes = Engine::filter_classes(classes).unwrap();
    //assert_eq!("sm:bg-[#222]", filtered_classes[0]);
    //assert_eq!("md:bg-[#333]", filtered_classes[1]);
    //assert_eq!("md:w-[100px]", filtered_classes[2]);
    //assert_eq!("bg-[#000]", filtered_classes[3]);
  }
}
