use actix_web::{HttpResponse, get, web};
use serde_json::json;

use crate::app::AppState;

#[get("/printer_status")]
pub async fn printer_status(app_data: web::Data<AppState>) -> HttpResponse {
    let mut statuses = Vec::new();

    for printer in &app_data.printers {
        let host = printer.clone();

        let result = web::block(move || crate::ipp::printer_status(&host)).await;

        match result {
            Ok(Ok(info)) => {
                let markers: Vec<_> = info
                    .markers
                    .iter()
                    .map(|(name, level)| json!({ "name": name, "level": level }))
                    .collect();

                statuses.push(json!({
                    "printer": printer,
                    "reasons": info.reasons,
                    "markers": markers,
                    "queued_jobs": info.queued_jobs,
                    "has_error": !info.ready
                        || info.reasons.iter().any(|reason| reason.ends_with("-error")),
                }));
            }
            Ok(Err(error)) => {
                statuses.push(json!({
                    "printer": printer,
                    "error": error,
                }));
            }
            Err(error) => {
                statuses.push(json!({
                    "printer": printer,
                    "error": error.to_string(),
                }));
            }
        }
    }

    HttpResponse::Ok().json(statuses)
}
