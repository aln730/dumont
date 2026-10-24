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

                let (up, has_error) = match result {
                    Ok(Ok(info)) => (1, !info.ready || info.reasons.iter().any(|r| r.ends_with("-error"))),
                    _ => (0, true),
                };

                let tags = [format!("printer:{}", printer.split('.').next().unwrap())];
                client.gauge("printer.up", up.to_string(), &tags).ok();
                client
                    .gauge("printer.has_error", (has_error as i32).to_string(), &tags)
                    .ok();
            }
            actix_web::rt::time::sleep(Duration::from_secs(30)).await;
        }
    });
}
