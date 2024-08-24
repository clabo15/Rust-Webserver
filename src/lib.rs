use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

/// Creates and returns a combined set of Warp filters (routes) for the web server.
///
/// This function defines and combines various routes:
/// - Serves static files from the `/static` directory.
/// - Serves the `index.html` file at the root path (`/`).
/// - Provides a simple "About" page at the `/about` route.
/// - Attaches logging to all routes.
///
/// The routes are combined and handled by Warp, with consistent error handling applied.
///
/// # Returns
/// An implementation of `Filter` that extracts a `Reply` and does not produce any errors (i.e., `Infallible`).
pub fn get_routes() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    // Serve files from the /static directory
    let static_files = warp::path("static")
        .and(warp::fs::dir("./static"));

    // Serve index.html on the root path "/"
    let index = warp::path::end()
        .and(warp::fs::file("./static/index.html"));

    // Serve a dynamic "about" route
    let about = warp::path("about")
        .map(|| "This is the about page.");

    // Combine all routes
    let routes = static_files
        .or(index)
        .or(about)
        .with(warp::log("rust_web_server"));

    // Apply recover at the end to ensure all errors are handled consistently
    routes.recover(handle_rejection)
}

/// Handles rejections and converts them into appropriate HTTP responses.
///
/// This function is used as an error handler in the Warp web framework.
/// It catches `Rejection` errors, determines the type of rejection, and
/// returns a corresponding HTTP response:
/// - `404 Not Found` for missing resources.
/// - `500 Internal Server Error` for other types of errors.
///
/// # Parameters
/// - `err`: The `Rejection` error that needs to be handled.
///
/// # Returns
/// A `Result` containing an HTTP response that is either a `404 Not Found` or
/// `500 Internal Server Error` response. The result is infallible, meaning
/// that it cannot produce an error.
///
/// # Examples
/// ```rust,ignore
/// let rejection = warp::reject::not_found();
/// let response = handle_rejection(rejection).await.unwrap();
/// assert_eq!(response.status(), warp::http::StatusCode::NOT_FOUND);
/// ```
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("404 - Not Found", warp::http::StatusCode::NOT_FOUND))
    } else {
        Ok(warp::reply::with_status("500 - Internal Server Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    }
}
