use warp::http::StatusCode;
use warp::test::request;
use rust_web_server::get_routes;
use warp::hyper::body::Bytes;

#[tokio::test]
async fn test_index_route() {
    let routes = get_routes();

    let response = request()
        .method("GET")
        .path("/")
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    // The body is already of type Bytes
    let body_bytes: Bytes = response.into_body();
    assert!(body_bytes.starts_with(b"<!DOCTYPE html>")); // assuming index.html starts like this
}

#[tokio::test]
async fn test_about_route() {
    let routes = get_routes();

    let response = request()
        .method("GET")
        .path("/about")
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    // Convert Bytes to a slice for comparison
    let body_bytes: Bytes = response.into_body();
    assert_eq!(body_bytes.as_ref(), b"This is the about page.");
}

#[tokio::test]
async fn test_404() {
    let routes = get_routes();

    let response = request()
        .method("GET")
        .path("/does_not_exist")
        .reply(&routes)
        .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Convert Bytes to a slice for comparison
    let body_bytes: Bytes = response.into_body();
    assert_eq!(body_bytes.as_ref(), b"404 - Not Found");
}