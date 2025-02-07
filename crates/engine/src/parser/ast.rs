/*use crate::parser::lexer::Token;
use std::sync::Arc;

#[derive(Debug)]
pub struct ASTTree {
  tree: Vec<ASTNode>,
}

impl ASTTree {
  pub fn new() -> Self {
    Self { tree: Vec::new() }
  }

  pub fn push(&mut self, node: ASTNode) {
    let _ = &self.tree.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct ASTNode {
  token: Token,
  next: Option<Vec<Box<Arc<ASTNode>>>>,
}

impl ASTNode {
  pub fn new(token: Token) -> Self {
    Self { token, next: None }
  }

  pub fn push(&mut self, node: ASTNode) -> Result<(), std::fmt::Error> {
    let mut next_vec: Vec<Box<Arc<ASTNode>>> = match &self.next {
      Some(data) => (&data).to_vec(),
      None => Vec::new(),
    };
    next_vec.push(Box::new(Arc::new(node)));
    self.next = Some(next_vec);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn ast_push() {
    let mut main_ast = ASTNode::new(Token::Breakpoint(String::from("md")));
    main_ast
      .push(ASTNode::new(Token::ClassName(String::from("w"))))
      .unwrap();
    assert_eq!(
      Token::ClassName(String::from("w")),
      main_ast.next.unwrap()[0].token
    );
  }
}

*/
