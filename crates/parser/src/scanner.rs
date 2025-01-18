use crate::rules::get_rules;
use regex::Regex;

#[napi]
struct Scanner {
  code: String,
}

#[napi]
struct ScannerOutput {
  css_class: Vec<String>,
}

#[derive(Debug)]
#[napi(object)]
pub struct ClasseInfo {
  pub breakpoint: Option<String>,
  pub classe_name: Option<String>,
  pub size: Option<String>,
}

#[napi]
impl Scanner {
  #[napi(constructor)]
  pub fn new(code: String) -> Self {
    Self { code }
  }

  #[napi]
  pub fn scan(&self) -> napi::Result<Vec<ClasseInfo>> {
    let re = Regex::new(r#"(?:class:\s*"|class=")([^"]+)""#).unwrap();
    let mut results = Vec::new();
    for (_, [classes]) in re.captures_iter(&self.code).map(|c| c.extract()) {
      let mut vec_classes: Vec<_> = classes.split_whitespace().map(str::to_string).collect();
      results.append(&mut vec_classes);
    }

    Ok(self.find_class(results.clone()).unwrap())
  }

  #[napi]
  pub fn find_class(&self, classes: Vec<String>) -> napi::Result<Vec<ClasseInfo>> {
    let re = Regex::new(r#"(?:(sm|md|lg|xl):)?([a-z-]+)?-(?:\[(.*?)\])?"#).unwrap();
    let mut results = Vec::new();

    for curr_classe in &classes {
      let mut matched = false;
      for cap in re.captures_iter(curr_classe) {
        matched = true;
        let breakpoint = cap.get(1).map_or(None, |m| Some(m.as_str().to_string()));
        let classe_name = cap.get(2).map_or(None, |m| Some(m.as_str().to_string()));
        let size = cap.get(3).map_or(None, |m| Some(m.as_str().to_string()));
        let new_info = ClasseInfo {
          breakpoint,
          classe_name,
          size,
        };
        results.push(new_info);
      }
    }
    Ok(results)
  }
}
