use actix_web::{App, HttpServer, middleware::Logger};
use dotenvy::dotenv;

mod api;
mod app;
mod ipp;

use app::{configure_app, get_app_data};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(
                "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .configure(configure_app)
            .data_factory(get_app_data)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
