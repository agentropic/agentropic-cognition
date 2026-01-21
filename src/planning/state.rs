use std::collections::HashMap;

/// World state
#[derive(Debug, Clone)]
pub struct State {
    variables: HashMap<String, String>,
}

impl State {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Set a variable
    pub fn set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Get a variable
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Check if variable exists with value
    pub fn matches(&self, key: &str, value: &str) -> bool {
        self.variables.get(key).is_some_and(|v| v == value)
    }

    /// Get all variables
    pub fn variables(&self) -> &HashMap<String, String> {
        &self.variables
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
