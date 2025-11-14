//! Alerting and notifications

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Alert status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertStatus {
    Active,
    Resolved,
    Acknowledged,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub condition: String,
    pub severity: AlertSeverity,
    pub enabled: bool,
    pub notifications: Vec<String>,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: AlertStatus,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: chrono::DateTime<chrono::Utc>,
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: HashMap<String, String>,
}

/// Alert manager
pub struct AlertManager {
    rules: std::sync::Arc<tokio::sync::RwLock<Vec<AlertRule>>>,
    alerts: std::sync::Arc<tokio::sync::RwLock<Vec<Alert>>>,
}

impl AlertManager {
    /// Create new alert manager
    pub fn new() -> Self {
        Self {
            rules: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            alerts: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Add alert rule
    pub async fn add_rule(&self, rule: AlertRule) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
    }

    /// Trigger alert
    pub async fn trigger(&self, rule_id: Uuid, message: String, severity: AlertSeverity) {
        let alert = Alert {
            id: Uuid::new_v4(),
            rule_id,
            status: AlertStatus::Active,
            severity,
            message,
            triggered_at: chrono::Utc::now(),
            resolved_at: None,
            metadata: HashMap::new(),
        };

        let mut alerts = self.alerts.write().await;
        alerts.push(alert);
    }

    /// Resolve alert
    pub async fn resolve(&self, alert_id: Uuid) {
        let mut alerts = self.alerts.write().await;
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(chrono::Utc::now());
        }
    }

    /// Get active alerts
    pub async fn active_alerts(&self) -> Vec<Alert> {
        let alerts = self.alerts.read().await;
        alerts
            .iter()
            .filter(|a| a.status == AlertStatus::Active)
            .cloned()
            .collect()
    }

    /// Get all alerts
    pub async fn all_alerts(&self) -> Vec<Alert> {
        self.alerts.read().await.clone()
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertRule {
    /// Create new alert rule
    pub fn new(name: String, condition: String, severity: AlertSeverity) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            condition,
            severity,
            enabled: true,
            notifications: Vec::new(),
        }
    }

    /// Add notification channel
    pub fn add_notification(&mut self, channel: String) {
        self.notifications.push(channel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alert_rule() {
        let rule = AlertRule::new(
            "High CPU".to_string(),
            "cpu > 80".to_string(),
            AlertSeverity::High,
        );

        assert_eq!(rule.name, "High CPU");
        assert!(rule.enabled);
    }

    #[tokio::test]
    async fn test_alert_manager() {
        let manager = AlertManager::new();
        let rule = AlertRule::new(
            "Test Alert".to_string(),
            "test".to_string(),
            AlertSeverity::Medium,
        );

        manager.add_rule(rule.clone()).await;
        manager
            .trigger(rule.id, "Test triggered".to_string(), AlertSeverity::Medium)
            .await;

        let active = manager.active_alerts().await;
        assert_eq!(active.len(), 1);
    }
}
