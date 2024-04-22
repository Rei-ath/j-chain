use crate::{test, utility};
use http::header::{
    ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONNECTION, CONTENT_TYPE, SEC_WEBSOCKET_KEY, SEC_WEBSOCKET_VERSION, UPGRADE
};
use http::{HeaderValue, Method, Version};
use http_body::Body;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::{upgrade, Request, Response};
use tokio_tungstenite::tungstenite::http::header::ACCESS_CONTROL_ALLOW_HEADERS;
use std::convert::Infallible;

/// Handles incoming HTTP requests.
///
/// Processes incoming HTTP requests and generates appropriate responses.
///
/// # Arguments
///
/// * `req` - The HTTP request received from the client.
///
/// # Returns
///
/// A `Result` containing either a successful `Response` or an error if processing fails.
pub async fn handle_request(
    req: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, Infallible> {
    let mut response = Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Full::new(Bytes::from("mewo, mewo nigga!\n")).boxed())
        .expect("values provided to the builder should be valid");

    if req.uri() == "/h" {
        println!("{:?}", req.body());
        response = Response::builder()
            .header(CONTENT_TYPE, "text/plain")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN,"*")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, POST")
            .body(Full::new(Bytes::from("m?c testing nigga.......")).boxed())
            .expect("values provided to the builder should be valid");
    }
    Ok(response)
}
