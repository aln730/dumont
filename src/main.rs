use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;

mod api;
mod app;
mod snmp;

use app::configure_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_app)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
