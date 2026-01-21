use serde::{Deserialize, Serialize};

/// A belief represents knowledge about the world
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Belief {
    key: String,
    value: String,
    certainty: f64,
}

impl Belief {
    /// Create a new belief
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            certainty: 1.0,
        }
    }

    /// Create a belief with certainty
    pub fn with_certainty(
        key: impl Into<String>,
        value: impl Into<String>,
        certainty: f64,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            certainty: certainty.clamp(0.0, 1.0),
        }
    }

    /// Create a fact (certainty = 1.0)
    pub fn fact(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(key, value)
    }

    /// Get the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the certainty
    pub fn certainty(&self) -> f64 {
        self.certainty
    }

    /// Set the certainty
    pub fn set_certainty(&mut self, certainty: f64) {
        self.certainty = certainty.clamp(0.0, 1.0);
    }
}
