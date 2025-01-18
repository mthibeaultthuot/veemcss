use crate::rules::get_rule;
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
  pub fn parse(&self) {
    for curr_classe in &self.classes_info {
      let token = &curr_classe.classe_name.clone().unwrap();
      let result = get_rule(token);
      println!("{:?}", result);
    }
  }
}
