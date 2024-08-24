use warp::Filter;

#[tokio::main]
async fn main(){
    // Define a simple route that returns "Hello, World"
    let hello = warp::path::end().map(|| "Hello, World!");

    // Start the web server on port 3030
    warp::serve(hello)
        .run(([127,0,0,1], 3030))
        .await;
}