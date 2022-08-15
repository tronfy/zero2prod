use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "localhost:8080";
    let listener = TcpListener::bind(addr).expect("failed to bind port");

    println!("listening on: http://{}", addr);
    run(listener)?.await
}
