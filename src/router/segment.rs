use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SegmentRecord {
  empty: Option<String>,
  literal: Option<String>,
  wildcard: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Analysis {
  path: String,
  segments: Vec<SegmentRecord>,
  params: Vec<String>,
  fingerprint: String,
}

#[derive(Clone)]
pub struct Segment {
  fulls: HashMap<String, Analysis>,
}

impl Segment {
  pub fn new() -> Segment {
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
