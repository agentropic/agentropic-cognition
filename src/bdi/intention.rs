use super::Goal;
use crate::planning::Plan;

/// An intention is a commitment to execute a plan to achieve a goal
#[derive(Debug, Clone)]
pub struct Intention {
    goal: Goal,
    plan: Plan,
    current_step: usize,
}

impl Intention {
    /// Create a new intention
    pub fn new(goal: Goal, plan: Plan) -> Self {
        Self {
            goal,
            plan,
            current_step: 0,
        }
    }

    /// Get the goal
    pub fn goal(&self) -> &Goal {
        &self.goal
    }

    /// Get the plan
    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    /// Get current step
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    /// Advance to next step
    pub fn next_step(&mut self) {
        if self.current_step < self.plan.actions().len() {
            self.current_step += 1;
        }
    }

    /// Check if completed
    pub fn is_completed(&self) -> bool {
        self.current_step >= self.plan.actions().len()
    }

    /// Reset to beginning
    pub fn reset(&mut self) {
        self.current_step = 0;
    }
}
