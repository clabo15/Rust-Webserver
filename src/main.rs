use warp::Filter;

#[tokio::main]
async fn main(){
    // Serve index.html
    let static_route = warp::path::end().and(warp::fs::file("./static/index.html"));
    
    // Start the web server on port 3030
    warp::serve(static_route)
        .run(([127,0,0,1], 3030))
        .await;
}