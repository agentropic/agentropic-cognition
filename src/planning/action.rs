use serde::{Deserialize, Serialize};

/// An action that can be performed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    name: String,
    parameters: Vec<String>,
}

impl Action {
    /// Create a new action
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: Vec::new(),
        }
    }

    /// Add a parameter
    pub fn with_parameter(mut self, param: impl Into<String>) -> Self {
        self.parameters.push(param.into());
        self
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get parameters
    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }
}
