use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:8888";
    println!(" _|￣|○ -----🎉🎉🎉👍💁👌 Listing to {}  ⚽🎍😍🎉🎉🎉------○|￣|_ ",addr);
    let srv= HttpServer::new(move  ||  {
            App::new()
            // .service(fs::Files::new("/", "client/public").index_file("index.html"))
            .service(Files::new("/","./").index_file("index.html"))
    })
    .bind(&addr)?;
    srv.run().await
    // println!("Hello, world!");
}
