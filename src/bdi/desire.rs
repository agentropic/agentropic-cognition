use super::Goal;

/// A desire represents a goal with priority
#[derive(Debug, Clone)]
pub struct Desire {
    goal: Goal,
    priority: f64,
}

impl Desire {
    /// Create a new desire
    pub fn new(goal: Goal, priority: f64) -> Self {
        Self {
            goal,
            priority: priority.clamp(0.0, 1.0),
        }
    }

    /// Get the goal
    pub fn goal(&self) -> &Goal {
        &self.goal
    }

    /// Get the priority
    pub fn priority(&self) -> f64 {
        self.priority
    }

    /// Set the priority
    pub fn set_priority(&mut self, priority: f64) {
        self.priority = priority.clamp(0.0, 1.0);
    }
}
