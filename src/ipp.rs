use std::time::Duration;

use ipp::prelude::*;

pub struct PrinterInfo {
    pub reasons: Vec<String>,
    pub markers: Vec<(String, i32)>,
    pub ready: bool,
    pub queued_jobs: i32,
}

fn flatten(value: Option<&IppValue>) -> Vec<&IppValue> {
    match value {
        Some(IppValue::Array(items)) => items.iter().collect(),
        Some(v) => vec![v],
        None => vec![],
    }
}

pub fn printer_status(host: &str) -> Result<PrinterInfo, String> {
    let uri: Uri = format!("ipp://{host}/ipp/print")
        .parse()
        .map_err(|e| format!("{e:?}"))?;

    let client = IppClient::builder(uri.clone())
        .request_timeout(Duration::from_secs(2))
        .build();

    let operation = IppOperationBuilder::get_printer_attributes(uri)
        .attributes([
            "printer-state-reasons",
            "printer-state",
            "printer-is-accepting-jobs",
            "queued-job-count",
            "marker-names",
            "marker-levels",
        ])
        .build();

    let response = client.send(operation).map_err(|e| e.to_string())?;

    let group = response
        .attributes()
        .groups_of(DelimiterTag::PrinterAttributes)
        .next()
        .ok_or("No printer status returned")?;
    let attrs = group.attributes();

    let reasons: Vec<String> = flatten(attrs.get("printer-state-reasons").map(|a| a.value()))
        .iter()
        .map(|r| r.to_string())
        .collect();
    if reasons.is_empty() {
        return Err("No printer status returned".to_string());
    }

    let names = flatten(attrs.get("marker-names").map(|a| a.value()));
    let levels = flatten(attrs.get("marker-levels").map(|a| a.value()));
    let mut markers = Vec::new();
    for (name, level) in names.into_iter().zip(levels) {
        if let IppValue::Integer(l) = level
            && *l >= 0
        {
            markers.push((name.to_string(), *l));
        }
    }

    let stopped = matches!(
        attrs.get("printer-state").map(|a| a.value()),
        Some(IppValue::Enum(5))
    );
    let refusing = matches!(
        attrs.get("printer-is-accepting-jobs").map(|a| a.value()),
        Some(IppValue::Boolean(false))
    );
    let queued_jobs = match attrs.get("queued-job-count").map(|a| a.value()) {
        Some(IppValue::Integer(n)) => *n,
        _ => 0,
    };

    Ok(PrinterInfo {
        reasons,
        markers,
        ready: !stopped && !refusing,
        queued_jobs,
    })
}
