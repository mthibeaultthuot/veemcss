use crate::properties::Properties;
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
  pub fn parse(&self) -> napi::Result<String> {
    let mut code = String::new();
    for curr_classe in &self.classes_info {
      let token = &curr_classe.classe_name.clone().unwrap();
      let propertie = Properties::parse(token.as_str());
      match propertie {
        Some(value) => {
          let css_string = match value.to_css() {
            Some(css_value) => css_value,
            None => break,
          };
          let css = format!(
            ".{}-\\[{}\\] {{
                    {} : {}
                }}
                ",
            token,
            curr_classe.size.clone().unwrap(),
            css_string,
            curr_classe.size.clone().unwrap()
          );
          code.push_str(css.as_str());
        }
        None => {}
      }
    }
    return Ok(code);
  }
}
