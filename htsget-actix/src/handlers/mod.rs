use actix_web::{http::StatusCode, Either, HttpRequest, Responder};

use htsget_config::types::JsonResponse;
use htsget_http::Result;
use pretty_json::PrettyJson;

pub use crate::handlers::service_info::{
  get_service_info_json, reads_service_info, variants_service_info,
};
use http::HeaderMap as HttpHeaderMap;

pub mod get;
pub mod post;
pub mod service_info;

mod pretty_json;

struct HeaderMap(HttpHeaderMap);

impl HeaderMap {
  fn into_inner(self) -> HttpHeaderMap {
    self.0
  }
}

impl From<&HttpRequest> for HeaderMap {
  fn from(http_request: &HttpRequest) -> Self {
    HeaderMap(HttpHeaderMap::from_iter(
      http_request.headers().clone().into_iter(),
    ))
  }
}

/// Handles a response, converting errors to json and using the proper HTTP status code
fn handle_response(response: Result<JsonResponse>) -> Either<impl Responder, impl Responder> {
  match response {
    Err(error) => {
      let (json, status_code) = error.to_json_representation();
      Either::Left(PrettyJson(json).customize().with_status(status_code))
    }
    Ok(json) => Either::Right(PrettyJson(json).customize().with_status(StatusCode::OK)),
  }
}
