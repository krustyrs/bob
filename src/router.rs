use fancy_regex::Regex;
use http_types::{Method, Url};
use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
  static ref RE_PARSE_PARAM: Regex = Regex::new(r"((?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+)|(?:\{(\w+)(?:(\*)(\d+)?)?(\?)?\})").unwrap();
  static ref RE_VALIDATE_PATH: Regex = Regex::new(r"(?:^\/$)|(?:^(?:\/(?:(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+|(?:\{\w+(?:\*[1-9]\d*)?\})|(?:(?:(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+(?:\{\w+\??\}))+(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})*)|(?:(?:\{\w+\??\})(?:(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+(?:\{\w+\??\}))+(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})*)|(?:(?:\{\w+\??\})(?:[\w\!\$&'\(\)\*\+\,;\=\:@\-\.~]|%[A-F0-9]{2})+)))*(?:\/(?:\{\w+(?:(?:\*(?:[1-9]\d*)?)|(?:\?))?\})?)?$)").unwrap();
  static ref RE_VALIDATE_PATH_ENCODED: Regex = Regex::new(r"%(?:2[146-9A-E]|3[\dABD]|4[\dA-F]|5[\dAF]|6[1-9A-F]|7[\dAE])").unwrap();
}

#[derive(Debug, Clone)]
pub struct RouteConfig {
  pub method: Method,
  pub path: &'static str,
}

struct Record {
  path: &'static str,
  segments: Vec<Segment>,
  params: Vec<&'static str>,
  fingerprint: Vec<&'static str>,
}

struct Segment {
  literal: &'static str,
}

struct RouterInner {
  routes: HashMap<Method, Vec<Record>>,
}

#[derive(Clone)]
pub struct Router {
  inner: Arc<RouterInner>,
}

impl Router {
  pub fn new() -> Router {
    Router {
      inner: Arc::new(RouterInner {
        routes: HashMap::new(),
      }),
    }
  }

  pub fn add(&mut self, config: RouteConfig) -> &mut Router {
    let analysis = self.analyze(config.path);
    self
      .mut_inner()
      .routes
      .entry(config.method)
      .or_insert(vec![])
      .push(analysis);

    self
  }

  pub fn route(self, method: Method, url: &Url) -> String {
    let body = match (method, url.path()) {
      (Method::Get, "/") => "Hello",
      (Method::Get, "/version") => "Version",
      (Method::Post, "/version") => "Posted version",
      _ => "",
    };

    body.to_string()
  }

  fn mut_inner(&mut self) -> &mut RouterInner {
    Arc::get_mut(&mut self.inner).expect("error obtaining mutable router")
  }

  fn analyze(&mut self, path: &'static str) -> Record {
    RE_VALIDATE_PATH.is_match(path).unwrap();

    let path_parts = path.split("/");
    let mut fingers = vec![];
    let mut segments = vec![];

    for mut path_part in path_parts {
      let path_part = path_part.to_lowercase();
      if path_part.find("{") == None {
        fingers.push(path_part.clone());
        segments.push(Segment {
          literal: &path_part.clone(),
        });
      }
    }

    let record = Record {
      path,
      segments,
      params: vec![],
      fingerprint: vec![],
    };
    record
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_add_route() {
    let mut router = Router::new();
    router.add(RouteConfig {
      method: Method::Get,
      path: "/test",
    });
  }

  #[test]
  fn fails_invalid_path() {
    let mut router = Router::new();
    router.add(RouteConfig {
      method: Method::Get,
      path: "/{p}{x}b",
    });
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
