use std::fmt;

#[derive(Debug, Clone)]
pub struct SegmentRecord {
  pub name: Option<String>,
  pub count: Option<i32>,
  pub empty: Option<bool>,
  pub literal: Option<String>,
  pub wildcard: Option<bool>,
}

impl SegmentRecord {
  pub fn new() -> Self {
    SegmentRecord {
      name: None,
      count: None,
      empty: None,
      literal: None,
      wildcard: None,
    }
  }

  pub fn with_name(mut self, name: String) -> Self {
    self.name = Some(name);
    self
  }

  pub fn with_count(mut self, count: i32) -> Self {
    self.count = Some(count);
    self
  }

  pub fn with_empty(mut self, empty: bool) -> Self {
    self.empty = Some(empty);
    self
  }

  pub fn with_literal(mut self, literal: String) -> Self {
    self.literal = Some(literal);
    self
  }

  pub fn with_wildcard(mut self, wildcard: bool) -> Self {
    self.wildcard = Some(wildcard);
    self
  }
}

impl fmt::Display for SegmentRecord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let name = match &self.name {
      Some(name) => name,
      None => "None",
    };

    let count = match &self.count {
      Some(count) => count.to_string(),
      None => "None".to_string(),
    };

    let wildcard = match &self.wildcard {
      Some(_wildcard) => "True",
      None => "None",
    };

    let literal = match &self.literal {
      Some(literal) => literal,
      None => "None",
    };

    write!(
      f,
      "name: {} \nwildcard: {}\nliteral: {} \ncount: {}",
      name, wildcard, literal, count
    )
  }
}
