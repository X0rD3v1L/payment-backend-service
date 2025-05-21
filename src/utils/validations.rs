use garde::Report;
use serde_json::{Value, json};

pub fn format_validation_errors_json(report: Report) -> Value {
    let formatted = format!("{}", report);
    let errors = formatted
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ": ");
            let field = parts.next()?.trim();
            let message = parts.next()?.trim();
            Some(json!({
                "field": field,
                "message": message
            }))
        })
        .collect::<Vec<Value>>();

    json!({ "errors": errors })
}

#[derive(Default)]
pub struct ProfileUpdateContext;

pub fn validate_optional_name(
    value: &Option<String>,
    _context: &ProfileUpdateContext,
) -> garde::Result {
    if let Some(name) = value {
        let len = name.len();
        let is_valid_chars = name
            .chars()
            .all(|c| c.is_alphabetic() || c == '-' || c == '\'' || c == ' ');

        if len < 2 || len > 50 {
            return Err(garde::Error::new("name must be 2–50 characters long"));
        }

        if !is_valid_chars {
            return Err(garde::Error::new("name contains invalid characters"));
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct TxnTypeContext;

pub fn is_valid_txn_type(value: &str, _context: &TxnTypeContext) -> garde::Result {
    if value != "purchase" && value != "credit" {
        return Err(garde::Error::new(
            "txn_type must be either 'purchase' or 'credit'",
        ));
    }
    Ok(())
}

#[derive(Default)]
pub struct TxnViewContext;

pub fn is_valid_tx_id(value: &str, _context: &TxnViewContext) -> garde::Result {
    let pattern =
        regex::Regex::new(r"^tx-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
            .unwrap();

    if pattern.is_match(value) {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid transaction ID format"))
    }
}
