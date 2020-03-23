use super::segment_record::SegmentRecord;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Analysis {
  pub path: String,
  pub segments: Vec<SegmentRecord>,
  pub params: Vec<String>,
  pub fingerprint: String,
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

  pub fn add(&mut self, _analysis: Analysis) {
    /*
        { literal: 'x' }        -> x
        { empty: false }        -> {p}
        { wildcard: true }      -> {p*}
        { mixed: /regex/ }      -> a{p}b
    */
    /*
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
    */
  }
}
