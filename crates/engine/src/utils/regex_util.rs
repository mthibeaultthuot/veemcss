use regex::Regex;

pub fn enum_to_string(string_enum: String) -> Option<String> {
  let re = Regex::new(r#""(.*?)""#).unwrap();
  let str_enum = &string_enum[..];
  if let Some(caps) = re.captures(str_enum) {
    match std::str::from_utf8(&caps[1].as_bytes()) {
      Ok(valid_str) => return Some(valid_str.to_string()),
      Err(_) => {}
    };
  }
  None
}
