use crate::models::alert::{Alert, AlertSeverity};
use std::collections::HashMap;

pub struct AnomalyScorer {
    weights: HashMap<AlertSeverity, f32>,
    threshold: f32,
    session_scores: HashMap<String, f32>, // AnyDesk ID -> Score
}

impl AnomalyScorer {
    #[must_use] 
    pub fn new(threshold: f32) -> Self {
        let mut weights = HashMap::new();
        weights.insert(AlertSeverity::Low, 10.0);
        weights.insert(AlertSeverity::Medium, 30.0);
        weights.insert(AlertSeverity::High, 60.0);
        weights.insert(AlertSeverity::Critical, 100.0);

        Self {
            weights,
            threshold,
            session_scores: HashMap::new(),
        }
    }

    /// Weighted scoring system
    pub fn score_alert(&mut self, alert: &Alert, session_id: &str) -> f32 {
        let weight = self.weights.get(&alert.severity).copied().unwrap_or(0.0);
        let current_score = self
            .session_scores
            .entry(session_id.to_string())
            .or_insert(0.0);
        *current_score += weight;
        *current_score
    }

    /// Baseline creation (simple implementation)
    #[must_use] 
    pub fn get_baseline_score(&self, _session_id: &str) -> f32 {
        // In a real app, this would query a database for historical averages
        20.0 // Default baseline
    }

    /// Alert for high-risk pivot scores
    #[must_use] 
    pub fn check_risk_level(&self, score: f32) -> String {
        if score >= self.threshold * 2.0 {
            "CRITICAL PIVOT RISK".to_string()
        } else if score >= self.threshold {
            "HIGH ANOMALY".to_string()
        } else {
            "NORMAL".to_string()
        }
    }
}
