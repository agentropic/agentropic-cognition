use super::Intention;

/// Stack of intentions
#[derive(Debug, Clone)]
pub struct IntentionStack {
    intentions: Vec<Intention>,
}

impl IntentionStack {
    /// Create a new intention stack
    pub fn new() -> Self {
        Self {
            intentions: Vec::new(),
        }
    }

    /// Push an intention
    pub fn push(&mut self, intention: Intention) {
        self.intentions.push(intention);
    }

    /// Pop an intention
    pub fn pop(&mut self) -> Option<Intention> {
        self.intentions.pop()
    }

    /// Get current intention
    pub fn current(&self) -> Option<&Intention> {
        self.intentions.last()
    }

    /// Get mutable current intention
    pub fn current_mut(&mut self) -> Option<&mut Intention> {
        self.intentions.last_mut()
    }

    /// Remove completed intentions
    pub fn remove_completed(&mut self) {
        self.intentions.retain(|i| !i.is_completed());
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.intentions.is_empty()
    }

    /// Get number of intentions
    pub fn len(&self) -> usize {
        self.intentions.len()
    }

    /// Clear all intentions
    pub fn clear(&mut self) {
        self.intentions.clear();
    }
}

impl Default for IntentionStack {
    fn default() -> Self {
        Self::new()
    }
}
