use std::time::Duration;

use ipp::prelude::*;

const PRINTER_STATE_REASONS: &str = "printer-state-reasons";

pub fn printer_status(host: &str) -> Result<Vec<String>, String> {
    let uri: Uri = format!("ipp://{host}/ipp/print")
        .parse()
        .map_err(|e| format!("{e:?}"))?;

    let client = IppClient::builder(uri.clone())
        .request_timeout(Duration::from_secs(2))
        .build();

    let operation = IppOperationBuilder::get_printer_attributes(uri)
        .attributes(&[PRINTER_STATE_REASONS])
        .build();

    let response = client.send(operation).map_err(|e| e.to_string())?;

    let group = response
        .attributes()
        .groups_of(DelimiterTag::PrinterAttributes)
        .next()
        .ok_or("No printer status returned")?;

    match group
        .attributes()
        .get(PRINTER_STATE_REASONS)
        .map(|a| a.value())
    {
        Some(IppValue::Array(reasons)) => Ok(reasons.iter().map(|r| r.to_string()).collect()),
        Some(reason) => Ok(vec![reason.to_string()]),
        None => Err("No printer status returned".to_string()),
    }
}
