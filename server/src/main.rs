mod server;

#[tokio::main]
async fn main() {
    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, server::app().unwrap()).await.unwrap();
}
