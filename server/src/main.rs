use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Starting http server: 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new().service(
            fs::Files::new("/", "./static")
                .show_files_listing()
                .use_last_modified(true)
                .index_file("index.html"),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
