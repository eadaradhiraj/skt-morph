use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStep {
    pub form: String,
    pub sutras: Vec<String>,
    pub kind: String,
    #[serde(flatten)]
    pub meta: HashMap<String, String>,
}

impl EngineStep {
    pub fn new(form: impl Into<String>, sutras: Vec<&str>, kind: impl Into<String>) -> Self {
        Self { form: form.into(), sutras: sutras.into_iter().map(|s| s.to_string()).collect(), kind: kind.into(), meta: HashMap::new() }
    }
}
