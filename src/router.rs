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
  pub fn new() -> Router {
    Router {
      inner: Arc::new(RouterInner {
        table: HashMap::new(),
      }),
    }
  }

  pub fn add(&mut self, method: Method, path: impl AsRef<str>) -> &mut Router {
    self.add_with_config(method, path, None)
  }

  pub fn add_with_config(
    &mut self,
    method: Method,
    path: impl AsRef<str>,
    config: Option<RouteConfig>,
  ) -> &mut Router {
    let analysis = self.analyze(path);
    let mut entry = self.mut_inner().table.entry(method).or_insert(Entry {
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
      Some(records) => {
        for record in records {
          if record.segments.len() == segments.len() {
            return url.path().to_string();
          }
        }

        "".to_string()
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
    let mut segments = vec![];

    // TODO: skip first path_part
    for path_part in path_parts {
      let path_part = path_part.to_lowercase();

      // Literal
      if path_part.find("{") == None {
        let literal = path_part.clone();
        fingers.push(path_part);
        segments.push(SegmentRecord {
          empty: None,
          literal: Some(literal),
          wildcard: None,
        });
        continue;
      }
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
