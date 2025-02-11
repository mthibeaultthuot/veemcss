use crate::parser::parser::TokenKind;

pub struct Generator<'a> {
  pub ast: Vec<TokenKind<'a>>,
  pub curr: usize,
}

impl<'a> Generator<'a> {
  pub fn new(ast: Vec<TokenKind<'a>>) -> Self {
    Self { ast, curr: 0 }
  }

  pub fn generate_css(&mut self) -> String {
    let mut generated_code = String::from("");
    while self.is_not_finish() {
      generated_code.push_str(self.generate_next_iteration().as_str());
      self.curr += 1;
    }
    generated_code
  }

  pub fn is_not_finish(&self) -> bool {
    self.curr < self.ast.len()
  }

  pub fn generate_next_iteration(&mut self) -> String {
    let curr = self.ast[self.curr].clone();
    format!("{}", curr)
  }
}
