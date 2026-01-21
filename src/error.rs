use thiserror::Error;

/// Cognition errors
#[derive(Error, Debug)]
pub enum CognitionError {
    #[error("Planning failed: {0}")]
    PlanningFailed(String),

    #[error("Goal not achievable: {0}")]
    GoalNotAchievable(String),

    #[error("Belief revision failed: {0}")]
    BeliefRevisionFailed(String),

    #[error("Reasoning error: {0}")]
    ReasoningError(String),

    #[error("Cognition error: {0}")]
    Other(String),
}
