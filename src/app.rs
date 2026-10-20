use actix_web::web::{self, scope};

use crate::api::printer_status;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/api").service(printer_status));
}

pub struct AppState {
    pub printers: Vec<String>,
}

pub async fn get_app_data() -> anyhow::Result<AppState> {
    let printers = std::env::var("DUMONT_PRINTERS")
        .expect("No DUMONT_PRINTERS")
        .to_string();

    let printers = printers
        .split_whitespace()
        .map(|printer| printer.to_string())
        .collect();

    Ok(AppState { printers })
}
