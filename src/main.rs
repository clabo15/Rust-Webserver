use rust_web_server::get_routes;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get all routes
    let routes = get_routes();

    // Start the web server on port 3030
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
