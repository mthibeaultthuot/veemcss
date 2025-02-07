use std::fmt::Error;

pub struct Generator<'a> {
  pub breakpoint_query: Option<&'a str>,
  pub classe_breakpoint_name: Option<&'a str>,
  pub classe_name: &'a str,
  pub classe_data: Option<&'a str>,
  pub css_code: &'a str,
}

impl<'a> Generator<'a> {
  pub fn new(
    breakpoint_query: Option<&'a str>,
    classe_breakpoint_name: Option<&'a str>,
    classe_name: &'a str,
    classe_data: Option<&'a str>,
    css_code: &'a str,
  ) -> Self {
    Self {
      breakpoint_query,
      classe_breakpoint_name,
      classe_name,
      classe_data,
      css_code,
    }
  }

  pub fn generate_css(&self, css_code: &str) -> Result<String, Error> {
    let mut code = String::new();
    let new_breakpoint_query = self.breakpoint_query.unwrap_or("");
    let new_breakpoint_query = self.classe_breakpoint_name.unwrap_or("");
    let new_classe_data = self.classe_data.unwrap_or("");
    let css = format!(
      "{}
          .{}{}-\\[{}\\] {{
              {}
          }}
      ",
      new_breakpoint_query, new_breakpoint_query, self.classe_name, new_classe_data, css_code,
    );
    code.push_str(css.as_str());
    //if breakpoint_query != "" {
    //  code.push_str("}");
    //}
    Ok(String::from("a"))
  }
}
