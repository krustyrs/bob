extern crate bob;
use bob::Router;
use http_types::{Method, Url};
mod common;

#[test]
fn add_routes() {
  let mut router = Router::new();
  router.add(Method::Get, "/version");

  let url = Url::parse("https://example.net/version").unwrap();
  let body = router.route(Method::Get, &url);
  assert_eq!("/version", &body);
}
