use logos::Lexer;
use logos::Logos;
use logos::Source;

pub fn get_rules(token: &str) {
  let mut lex = Token::lexer(token);
  println!("{:?}", lex.next());
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token<'a> {
  /// background definition
  #[token("bg", |_| "background")]
  Background(&'a str),

  /// padding
  #[token("p", |_| "padding")]
  Padding(&'a str),
}
