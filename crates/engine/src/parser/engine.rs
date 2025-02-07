use crate::parser::parser::Parser;

#[napi]
pub struct Engine {}
#[napi]
impl Engine {
  pub fn from_string(code: &str) -> Result<(), std::fmt::Error> {
    let mut parser = Parser::new(code);

    // read only
    let _tree_result = parser.parse().unwrap();

    // call generation from tree read only
    Ok(())
  }

  pub fn filter_classes(classes: Vec<&str>) -> Result<Vec<&str>, std::fmt::Error> {
    let mut filtered_classes = Vec::new();
    let breakpoints = vec!["sm", "md", "lg", "xl", "2xl"];
    let mut i = 0;
    loop {
      if i > breakpoints.len() - 1 {
        break;
      }
      let curr_breakpoint = breakpoints[i];

      let mut find_classes = classes
        .iter()
        .filter(|classe| classe.contains(curr_breakpoint))
        .cloned()
        .collect::<Vec<_>>();
      filtered_classes.append(&mut find_classes);

      i += 1;
    }

    let remaining_classes: Vec<&str> = classes
      .iter()
      .filter(|classe| !breakpoints.iter().any(|bp| classe.contains(bp)))
      .cloned()
      .collect();

    filtered_classes.append(&mut remaining_classes.to_vec());

    Ok(filtered_classes)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_string() {
    let code = r#"<div class="md:w-[100px] bg-[#111]"></div>"#;
    Engine::from_string(code).unwrap();
  }

  #[test]
  fn filter_class() {
    let classes = vec!["bg-[#000]", "md:bg-[#333]", "md:w-[100px]", "sm:bg-[#222]"];
    let filtered_classes = Engine::filter_classes(classes).unwrap();
    assert_eq!("sm:bg-[#222]", filtered_classes[0]);
    assert_eq!("md:bg-[#333]", filtered_classes[1]);
    assert_eq!("md:w-[100px]", filtered_classes[2]);
    assert_eq!("bg-[#000]", filtered_classes[3]);
  }
}
