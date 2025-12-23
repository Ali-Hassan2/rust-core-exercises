use axum:: Router;
use dotenvy:: dotenv;
use std::net::SocketAddr;


#[tokio::main]
fn main() {
    dotenv().ok();
    let app = Router::new()
    let addr  = SocketAddr::from(([127,0,0,1],3000));
    println!("Server is running at: {}",addr)

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap()
}
