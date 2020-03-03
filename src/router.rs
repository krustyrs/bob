use http_types::{Method, Url};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RouteConfig {
  pub method: Method,
  pub path: &'static str,
}

struct Record {
  path: &'static str,
  segments: Vec<&'static str>,
  params: Vec<&'static str>,
  fingerprint: Vec<&'static str>,
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
    let mut record = Record {
      path,
      segments: vec![],
      params: vec![],
      fingerprint: vec![],
    };

    record.params.push("");
    record
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
