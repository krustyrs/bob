use fancy_regex::Regex;
use http_types::{Method, Url};
use std::collections::HashMap;
use std::sync::Arc;

mod segment;
use segment::{Analysis, Segment, SegmentRecord};

lazy_static! {
  static ref RE_PARSE_PARAM: Regex = Regex::new(r"((?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+)|(?:\{(\w+)(?:(\*)(\d+)?)?(\?)?\})").unwrap();
  static ref RE_VALIDATE_PATH: Regex = Regex::new(r"(?:^\/$)|(?:^(?:\/(?:(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+|(?:\{\w+(?:\*[1-9]\d*)?\})|(?:(?:(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+(?:\{\w+\??\}))+(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})*)|(?:(?:\{\w+\??\})(?:(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+(?:\{\w+\??\}))+(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})*)|(?:(?:\{\w+\??\})(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+)))*(?:\/(?:\{\w+(?:(?:\*(?:[1-9]\d*)?)|(?:\?))?\})?)?$)").unwrap();
  static ref RE_VALIDATE_PATH_ENCODED: Regex = Regex::new(r"%(?:2[146-9A-E]|3[\dABD]|4[\dA-F]|5[\dAF]|6[1-9A-F]|7[\dAE])").unwrap();
}

#[derive(Debug, Clone)]
pub struct RouteConfig {
  pub vhost: String,
}

#[derive(Debug, Clone)]
struct Entry {
  routes: Vec<String>, // TODO: Change to handlers
  segment: Segment,
}

struct RouterInner {
  table: HashMap<Method, Entry>,
}

#[derive(Clone)]
pub struct Router {
  inner: Arc<RouterInner>,
}

impl Router {
  pub fn new() -> Self {
    Router {
      inner: Arc::new(RouterInner {
        table: HashMap::new(),
      }),
    }
  }

  pub fn add(&mut self, method: Method, path: impl AsRef<str>) -> &mut Self {
    self.add_with_config(method, path, None)
  }

  pub fn add_with_config(
    &mut self,
    method: Method,
    path: impl AsRef<str>,
    config: Option<RouteConfig>,
  ) -> &mut Router {
    let analysis = self.analyze(path);
    let entry = self.mut_inner().table.entry(method).or_insert(Entry {
      routes: vec![],
      segment: Segment::new(),
    });

    entry.segment.add(analysis);
    self
  }

  pub fn route(self, method: Method, url: &Url) -> String {
    let segments = if url.path().len() == 1 {
      vec![""]
    } else {
      let mut parts: Vec<&str> = url.path().split("/").collect();
      parts.split_off(1)
    };

    match self.inner.table.get(&method) {
      None => "".to_string(),
      Some(_entry) => {
        return url.path().to_string();
      }
    }
  }

  //fn lookup<'a>(self, path: &'a str, segments: Vec<&'a str>, records: Vec<Record>)

  fn mut_inner(&mut self) -> &mut RouterInner {
    Arc::get_mut(&mut self.inner).expect("error obtaining mutable router")
  }

  fn analyze<S: AsRef<str>>(&mut self, path: S) -> Analysis {
    RE_VALIDATE_PATH.is_match(path.as_ref()).unwrap();

    let path_parts = path.as_ref().split("/");
    let mut fingers = vec![];
    let mut params: Vec<SegmentRecord> = vec![];
    let mut segments: Vec<SegmentRecord> = vec![];

    // TODO: skip first path_part
    for path_part in path_parts {
      let path_part = path_part.to_lowercase();

      // Literal
      if path_part.find("{") == None {
        let literal = path_part.clone();
        fingers.push(path_part);
        segments.push(SegmentRecord::new().with_literal(literal));
        continue;
      }

      // Parameter
      let parts = parse_params(path_part);
    }

    let analysis = Analysis {
      path: path.as_ref().to_string(),
      segments,
      params: vec![],
      fingerprint: format!("/{}", fingers.join("/")),
    };
    analysis
  }
}

fn parse_params(part: String) -> Vec<SegmentRecord> {
  let mut parts: Vec<SegmentRecord> = vec![];

  // groups are: 0: literal, 1: name, 2: wildcard, 3: count, 4: empty
  match RE_PARSE_PARAM.captures(part.as_str()) {
    Ok(captures) => match captures {
      Some(captures) => match captures.get(0) {
        Some(literal) => {
          parts.push(SegmentRecord::new().with_literal(literal.as_str().to_string()));
        }
        None => {
          let mut record = SegmentRecord::new();
          record = match captures.get(1) {
            Some(name) => record.with_name(name.as_str().to_string()),
            None => record,
          };

          record = match captures.get(2) {
            Some(_wildcard) => record.with_wildcard(true),
            None => record.with_wildcard(false),
          };

          record = match captures.get(3) {
            Some(count) => record.with_count(count.as_str().to_string().parse::<i32>().unwrap()),
            None => record,
          };

          record = match captures.get(2) {
            Some(_empty) => record.with_empty(true),
            None => record.with_empty(false),
          };

          parts.push(record);
        }
      },
    },
  }

  parts
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_add_route() {
    let mut router = Router::new();
    router.add(Method::Get, "/test");
  }

  #[test]
  fn fails_invalid_path() {
    let mut router = Router::new();
    router.add(Method::Get, "/{p}{x}b");
  }

  #[test]
  fn can_route() {
    let mut router = Router::new();
    router.add(Method::Get, "/test");
    let url = Url::parse("http://test.com/test");
    assert!(!url.is_err());
    let found = router.route(Method::Get, &url.unwrap());
    assert_eq!(found, "/test");
  }

  #[test]
  fn check_path_regex() {
    let invalids = vec!["path", "/%path/", "/path/{param*}/to"];

    for invalid in invalids {
      let result = RE_VALIDATE_PATH.is_match(invalid);
      assert!(!result.unwrap());
    }

    let valids = vec!["/", "/path", "/path/"];
    for valid in valids {
      let result = RE_VALIDATE_PATH.is_match(valid);
      assert!(result.unwrap());
    }
  }
}
