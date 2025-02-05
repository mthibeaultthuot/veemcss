use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
  #[regex(r#"sm:|md:|lg:|xl:"#, |lex| &lex.slice()[0..lex.slice().len() - 1])]
  Breakpoint(&'a str),

  #[regex(r"[a-zA-Z]+", |lex| lex.slice())]
  ClassName(&'a str),

  #[regex(r"-\[([^\]]+)\]", |lex| &lex.slice()[2..lex.slice().len() - 1])]
  Metadata(&'a str),
}

pub struct Lexer<'a> {
  data: &'a str,
}

impl<'a> Lexer<'a> {
  pub fn lex(data: &'a str) -> Result<Vec<Token>, std::fmt::Error> {
    let mut output = Vec::new();
    let mut lex = Token::lexer(data);
    for token in lex.by_ref() {
      match token {
        Ok(Token::Breakpoint(ref s)) => output.push(Token::Breakpoint(s)),
        Ok(Token::ClassName(ref s)) => output.push(Token::ClassName(s)),
        Ok(Token::Metadata(ref s)) => output.push(Token::Metadata(s)),
        Err(_err) => panic!("Lexer failed : Invalid class name"),
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
    let output = Lexer::lex(input).unwrap();
    assert_eq!(Token::Breakpoint("md"), output[0]);
    assert_eq!(Token::ClassName("text"), output[1]);
    assert_eq!(Token::Metadata("100px"), output[2]);
  }

  #[test]
  fn lexer_class() {
    let input = "text-[100px]";
    let _output = Lexer::lex(input);
  }
}
