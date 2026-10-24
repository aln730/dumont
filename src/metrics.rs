use dogstatsd::{Client, Options};
use std::time::Duration;

pub fn start(printers: Vec<String>) {
    let host = match std::env::var("DD_AGENT_HOST") {
        Ok(host) => host,
        Err(_) => return,
    };
    let options = Options::new(
        "0.0.0.0:0",
        &format!("{host}:8125"),
        "dumont",
        vec![],
        None,
        None,
    );
    let client = Client::new(options).expect("couldn't set up dogstatsd");

    actix_web::rt::spawn(async move {
        loop {
            for printer in &printers {
                let name = printer.clone();
                let result = actix_web::web::block(move || crate::ipp::printer_status(&name)).await;

                let tags = [format!("printer:{}", printer.split('.').next().unwrap())];

                let Ok(Ok(info)) = result else {
                    client.gauge("printer.up", "0", &tags).ok();
                    client.gauge("printer.has_error", "1", &tags).ok();
                    continue;
                };

                let has_error = !info.ready || info.reasons.iter().any(|r| r.ends_with("-error"));

                client.gauge("printer.up", "1", &tags).ok();
                client
                    .gauge("printer.has_error", (has_error as i32).to_string(), &tags)
                    .ok();

                for reason in info.reasons.iter().filter(|r| r.ends_with("-error")) {
                    let tags = [tags[0].clone(), format!("reason:{reason}")];
                    client.gauge("printer.error_reason", "1", &tags).ok();
                }
                if !info.ready {
                    let tags = [tags[0].clone(), "reason:printer-stopped".to_string()];
                    client.gauge("printer.error_reason", "1", &tags).ok();
                }
                client
                    .gauge("printer.queued_jobs", info.queued_jobs.to_string(), &tags)
                    .ok();

                for (marker, level) in &info.markers {
                    let tags = [tags[0].clone(), format!("marker:{marker}")];
                    client
                        .gauge("printer.marker_level", level.to_string(), &tags)
                        .ok();
                }
            }
            actix_web::rt::time::sleep(Duration::from_secs(30)).await;
        }
    });
}
