use std::{borrow::Cow, cell::LazyCell, collections::HashMap};

use validator::ValidationError;

const MOBILE_PHONE_REGEX: LazyCell<regex::Regex> = LazyCell::new(|| {
    regex::Regex::new(r"^1[3456789]\d{9}$")
        .unwrap_or_else(|e| panic!("Failed to compile mobile phone regex: {}", e))
});

pub fn is_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if MOBILE_PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(build_validation_error("ivalid mobile phone"))
    }
}
fn build_validation_error(message: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(message)),
        params: HashMap::new(),
    }
}
