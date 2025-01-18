use crate::scanner::ClasseInfo;

#[napi]
pub struct Parser {
  classes_info: Vec<ClasseInfo>,
}

#[napi]
impl Parser {
  #[napi(constructor)]
  pub fn new(classes_info: Vec<ClasseInfo>) -> Self {
    Self { classes_info }
  }

  #[napi]
  pub fn parse(&self) {}
}
