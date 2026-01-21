/// Utility function for decision-making
#[derive(Debug)]
pub struct UtilityFunction {
    name: String,
}

impl UtilityFunction {
    /// Create a new utility function
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Evaluate utility (placeholder)
    pub fn evaluate(&self, _state: &[String]) -> f64 {
        // Placeholder - real implementation would calculate utility
        0.5
    }
}
