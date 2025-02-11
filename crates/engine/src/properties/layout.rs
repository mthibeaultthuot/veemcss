use logos::Logos;

#[derive(Clone, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Layout<'a> {
  #[token("d", |_| "display")]
  Display(&'a str),
  #[token("p", |_| "padding")]
  Padding(&'a str),
  #[token("pos", |_| "position")]
  Position(&'a str),
  #[token("z", |_| "z-index")]
  ZIndex(&'a str),
}
