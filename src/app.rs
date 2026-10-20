use actix_web::web::{self, scope};

use crate::api::printer_status;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(printer_status),
    );
}
