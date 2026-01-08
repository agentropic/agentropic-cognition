use super::{Action, Plan, State};
use crate::CognitionError;

/// Simple planner
#[derive(Debug)]
pub struct Planner {
    actions: Vec<Action>,
}

impl Planner {
    /// Create a new planner
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Create with available actions
    pub fn with_actions(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    /// Plan from initial state to goal state
    pub fn plan(&self, _initial: &State, _goal: &State) -> Result<Plan, CognitionError> {
        // Simple placeholder implementation
        // Real implementation would use STRIPS, HTN, or other planning algorithm
        Ok(Plan::new("simple_plan"))
    }

    /// Add an available action
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Get available actions
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}
