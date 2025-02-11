use logos::Logos;

#[derive(Clone, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Background<'a> {
  #[token("bg", |_| "background")]
  Background(&'a str),
}
