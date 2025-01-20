use logos::Logos;

#[derive(Clone, Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum LayoutRules<'a> {
  #[token("d", |_| "display")]
  Display(&'a str),
  #[token("pos", |_| "position")]
  Position(&'a str),
  #[token("z", |_| "z-index")]
  ZIndex(&'a str),
}
