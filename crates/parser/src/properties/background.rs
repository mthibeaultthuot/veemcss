use logos::Logos;

#[derive(Clone, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum BackgroundPro<'a> {
  #[token("bg", |_| "background")]
  Background(&'a str),
  #[token("bgc", |_| "background-color")]
  BackgroundColor(&'a str),
  #[token("bgi", |_| "background-image")]
  BackgroundImage(&'a str),
}
