use actix_files as fs;
use actix_web::{App, HttpServer};

mod ws;
use ws::on_ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Starting http server: 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(on_ws)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
