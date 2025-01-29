use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
  #[regex(r#"sm:|md:|lg:|xl:"#, |lex| lex.slice()[0..lex.slice().len() - 1].to_owned())]
  Breakpoint(String),

  #[regex(r"[a-zA-Z]+", |lex| lex.slice().to_owned())]
  ClassName(String),

  #[regex(r"-\[([^\]]+)\]", |lex| lex.slice()[2..lex.slice().len() - 1].to_owned())]
  Metadata(String),
}

pub struct TokenizeOutput {
  token: Box<Token>,
}

pub struct Lexer<'a> {
  data: &'a str,
}

impl<'a> Lexer<'a> {
  pub fn lex(data: &'a str) -> Result<Vec<Token>, std::fmt::Error> {
    let mut output = Vec::new();
    let mut lex = Token::lexer(data);
    while let Some(token) = lex.next() {
      match token {
        Ok(Token::Breakpoint(ref s)) => {
          output.push((crate::parser::lexer::Token::Breakpoint.clone())(s.clone()))
        }
        Ok(Token::ClassName(ref s)) => {
          output.push((crate::parser::lexer::Token::ClassName.clone())(s.clone()))
        }
        Ok(Token::Metadata(ref s)) => {
          output.push((crate::parser::lexer::Token::Metadata.clone())(s.clone()))
        }
        Err(er) => panic!("Lexer failed : Invalid class name"),
      }
    }
    Ok(output)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lexer_full_class() {
    let input = "md:text-[100px]";
    let output = Lexer::lex(input);
  }

  #[test]
  fn lexer_class() {
    let input = "text-[100px]";
    let output = Lexer::lex(input);
    println!("{:?}", output);
  } 
}
