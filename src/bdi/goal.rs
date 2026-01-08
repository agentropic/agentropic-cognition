use serde::{Deserialize, Serialize};

/// Type of goal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalType {
    /// Achievement goal (reach a state)
    Achievement,
    /// Maintenance goal (maintain a state)
    Maintenance,
    /// Test goal (check if condition holds)
    Test,
}

/// A goal represents a desired state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Goal {
    name: String,
    goal_type: GoalType,
    conditions: Vec<String>,
    priority: f64,
}

impl Goal {
    /// Create a new goal
    pub fn new(name: impl Into<String>, goal_type: GoalType) -> Self {
        Self {
            name: name.into(),
            goal_type,
            conditions: Vec::new(),
            priority: 0.5,
        }
    }

    /// Create an achievement goal
    pub fn achievement(name: impl Into<String>) -> Self {
        Self::new(name, GoalType::Achievement)
    }

    /// Create a maintenance goal
    pub fn maintenance(name: impl Into<String>) -> Self {
        Self::new(name, GoalType::Maintenance)
    }

    /// Add a condition
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: f64) -> Self {
        self.priority = priority.clamp(0.0, 1.0);
        self
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the goal type
    pub fn goal_type(&self) -> GoalType {
        self.goal_type
    }

    /// Get conditions
    pub fn conditions(&self) -> &[String] {
        &self.conditions
    }

    /// Get priority
    pub fn priority(&self) -> f64 {
        self.priority
    }
}
