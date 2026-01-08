/// A reasoning rule
#[derive(Debug, Clone)]
pub struct Rule {
    name: String,
    conditions: Vec<String>,
    conclusions: Vec<String>,
}

impl Rule {
    /// Create a new rule
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            conditions: Vec::new(),
            conclusions: Vec::new(),
        }
    }

    /// Add a condition
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Add a conclusion
    pub fn with_conclusion(mut self, conclusion: impl Into<String>) -> Self {
        self.conclusions.push(conclusion.into());
        self
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get conditions
    pub fn conditions(&self) -> &[String] {
        &self.conditions
    }

    /// Get conclusions
    pub fn conclusions(&self) -> &[String] {
        &self.conclusions
    }
}
