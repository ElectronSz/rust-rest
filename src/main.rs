
use actix_web::middleware::Logger;
use actix_web::{App,web, HttpServer};

use rest_api::db::connect;
use rest_api::route;
use rest_api::state::AppState;

#[actix_web::main]

async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let pool = connect().await.unwrap();

    env_logger::init();

    
    let port: u16 = 3000;

    println!("🚀 Server running on port {} ",port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState{  db: pool.clone() }))
        .configure(route::config)
        .wrap(Logger::default())
        
    })
    
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

