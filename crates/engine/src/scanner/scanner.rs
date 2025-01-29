use regex::Regex;


pub struct Scanner();


impl Scanner {
  pub fn scan(code : &str) -> Result<Vec<&str>, std::fmt::Error> {
    let re = Regex::new(r#"(?:class:\s*"|class=")([^"]+)""#).unwrap();
    let mut results = Vec::new();
    for (_, [classes]) in re.captures_iter(code).map(|c| c.extract()) {
      let mut vec_classes: Vec<_> = classes.split_whitespace().collect();
      results.append(&mut vec_classes);
    }
    Ok(results)
  }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn scan() {
        let code = r#"<div class="md:w-[100px] bg-[#111]"></div><h1 class="txt-[1vw]"></h1>"#;
        let classes = Scanner::scan(code).unwrap();
        assert_eq!("md:w-[100px]", classes[0]);
        assert_eq!("bg-[#111]", classes[1]);
        assert_eq!("txt-[1vw]", classes[2]);
    }
}

