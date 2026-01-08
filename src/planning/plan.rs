use super::Action;

/// A plan is a sequence of actions
#[derive(Debug, Clone)]
pub struct Plan {
    name: String,
    actions: Vec<Action>,
}

impl Plan {
    /// Create a new plan
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            actions: Vec::new(),
        }
    }

    /// Add an action
    pub fn add_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get actions
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.actions.len()
    }
}
