use std::net::TcpListener;

use listall_api::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server is listening on 127.0.0.1:{}", port);
    run(listener)?.await
}
