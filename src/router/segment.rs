use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SegmentRecord {
  name: Option<String>,
  count: Option<i32>,
  empty: Option<bool>,
  literal: Option<String>,
  wildcard: Option<bool>,
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

#[derive(Debug, Clone)]
pub struct Analysis {
  path: String,
  segments: Vec<SegmentRecord>,
  params: Vec<String>,
  fingerprint: String,
}

#[derive(Debug, Clone)]
pub struct Segment {
  fulls: HashMap<String, Analysis>,
}

impl Segment {
  pub fn new() -> Self {
    Segment {
      fulls: HashMap::new(),
    }
  }

  pub fn add(&mut self, analysis: Analysis) {
    /*
        { literal: 'x' }        -> x
        { empty: false }        -> {p}
        { wildcard: true }      -> {p*}
        { mixed: /regex/ }      -> a{p}b
    */

    let current = analysis.segments[0];
    let remaining = analysis.segments.split_off(1);
    let is_edge = remaining.len() == 0;

    let mut literals: Vec<String> = vec![];
    let mut is_literal = true;
    for segment in analysis.segments {
      match segment.literal {
        Some(literal) => {
          literals.push(literal);
        }
        None => {
          is_literal = false;
          break;
        }
      }
    }

    if is_literal {}
  }
}
