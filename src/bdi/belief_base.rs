use super::Belief;
use std::collections::HashMap;

/// Collection of beliefs
#[derive(Debug, Clone)]
pub struct BeliefBase {
    beliefs: HashMap<String, Belief>,
}

impl BeliefBase {
    /// Create a new belief base
    pub fn new() -> Self {
        Self {
            beliefs: HashMap::new(),
        }
    }

    /// Add or update a belief
    pub fn add(&mut self, belief: Belief) {
        self.beliefs.insert(belief.key().to_string(), belief);
    }

    /// Get a belief by key
    pub fn get(&self, key: &str) -> Option<&Belief> {
        self.beliefs.get(key)
    }

    /// Remove a belief
    pub fn remove(&mut self, key: &str) -> Option<Belief> {
        self.beliefs.remove(key)
    }

    /// Check if belief exists
    pub fn contains(&self, key: &str) -> bool {
        self.beliefs.contains_key(key)
    }

    /// Get all beliefs
    pub fn all(&self) -> impl Iterator<Item = &Belief> {
        self.beliefs.values()
    }

    /// Query beliefs matching a condition
    pub fn query<F>(&self, predicate: F) -> Vec<&Belief>
    where
        F: Fn(&Belief) -> bool,
    {
        self.beliefs.values().filter(|b| predicate(b)).collect()
    }

    /// Clear all beliefs
    pub fn clear(&mut self) {
        self.beliefs.clear();
    }

    /// Number of beliefs
    pub fn len(&self) -> usize {
        self.beliefs.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.beliefs.is_empty()
    }
}

impl Default for BeliefBase {
    fn default() -> Self {
        Self::new()
    }
}
