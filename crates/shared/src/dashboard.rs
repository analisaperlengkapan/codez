//! Dashboard service for real-time visualization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Dashboard definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub layout: DashboardLayout,
    pub widgets: Vec<WidgetConfig>,
    pub permissions: Vec<DashboardPermission>,
    pub is_public: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Dashboard layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub grid_width: u32,
    pub grid_height: u32,
    pub refresh_interval: u32,
    pub theme: String,
}

/// Widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub id: Uuid,
    pub widget_type: String,
    pub title: String,
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub config: HashMap<String, serde_json::Value>,
    pub data_source: Option<String>,
}

/// Dashboard permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardPermission {
    pub user_id: Uuid,
    pub permission_type: PermissionType,
}

/// Permission type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionType {
    View,
    Edit,
    Admin,
}

/// Dashboard version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardVersion {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub version: u32,
    pub layout: DashboardLayout,
    pub widgets: Vec<WidgetConfig>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Dashboard service
pub struct DashboardService {
    dashboards: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, Dashboard>>>,
    versions: std::sync::Arc<tokio::sync::RwLock<Vec<DashboardVersion>>>,
}

impl DashboardService {
    /// Create new dashboard service
    pub fn new() -> Self {
        Self {
            dashboards: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            versions: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    /// Create dashboard
    pub async fn create_dashboard(
        &self,
        name: String,
        owner_id: Uuid,
    ) -> Result<Dashboard, String> {
        let dashboard = Dashboard {
            id: Uuid::new_v4(),
            name,
            description: None,
            owner_id,
            layout: DashboardLayout {
                grid_width: 12,
                grid_height: 8,
                refresh_interval: 30,
                theme: "light".to_string(),
            },
            widgets: Vec::new(),
            permissions: vec![DashboardPermission {
                user_id: owner_id,
                permission_type: PermissionType::Admin,
            }],
            is_public: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let mut dashboards = self.dashboards.write().await;
        dashboards.insert(dashboard.id, dashboard.clone());

        Ok(dashboard)
    }

    /// Get dashboard
    pub async fn get_dashboard(&self, dashboard_id: Uuid) -> Result<Dashboard, String> {
        let dashboards = self.dashboards.read().await;
        dashboards
            .get(&dashboard_id)
            .cloned()
            .ok_or_else(|| format!("Dashboard not found: {}", dashboard_id))
    }

    /// Update dashboard
    pub async fn update_dashboard(&self, dashboard: Dashboard) -> Result<(), String> {
        let mut dashboards = self.dashboards.write().await;
        let mut updated = dashboard;
        updated.updated_at = chrono::Utc::now();
        dashboards.insert(updated.id, updated);
        Ok(())
    }

    /// Delete dashboard
    pub async fn delete_dashboard(&self, dashboard_id: Uuid) -> Result<(), String> {
        let mut dashboards = self.dashboards.write().await;
        dashboards.remove(&dashboard_id);
        Ok(())
    }

    /// Add widget
    pub async fn add_widget(&self, dashboard_id: Uuid, widget: WidgetConfig) -> Result<(), String> {
        let mut dashboards = self.dashboards.write().await;
        if let Some(dashboard) = dashboards.get_mut(&dashboard_id) {
            dashboard.widgets.push(widget);
            dashboard.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(format!("Dashboard not found: {}", dashboard_id))
        }
    }

    /// Remove widget
    pub async fn remove_widget(&self, dashboard_id: Uuid, widget_id: Uuid) -> Result<(), String> {
        let mut dashboards = self.dashboards.write().await;
        if let Some(dashboard) = dashboards.get_mut(&dashboard_id) {
            dashboard.widgets.retain(|w| w.id != widget_id);
            dashboard.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(format!("Dashboard not found: {}", dashboard_id))
        }
    }

    /// List dashboards
    pub async fn list_dashboards(&self) -> Vec<Dashboard> {
        let dashboards = self.dashboards.read().await;
        dashboards.values().cloned().collect()
    }

    /// Save version
    pub async fn save_version(
        &self,
        dashboard_id: Uuid,
        created_by: Uuid,
    ) -> Result<DashboardVersion, String> {
        let dashboards = self.dashboards.read().await;
        let dashboard = dashboards
            .get(&dashboard_id)
            .ok_or_else(|| format!("Dashboard not found: {}", dashboard_id))?;

        let versions = self.versions.read().await;
        let version_num = versions
            .iter()
            .filter(|v| v.dashboard_id == dashboard_id)
            .count() as u32
            + 1;

        drop(versions);

        let version = DashboardVersion {
            id: Uuid::new_v4(),
            dashboard_id,
            version: version_num,
            layout: dashboard.layout.clone(),
            widgets: dashboard.widgets.clone(),
            created_by,
            created_at: chrono::Utc::now(),
        };

        let mut versions = self.versions.write().await;
        versions.push(version.clone());

        Ok(version)
    }

    /// Get versions
    pub async fn get_versions(&self, dashboard_id: Uuid) -> Vec<DashboardVersion> {
        let versions = self.versions.read().await;
        versions
            .iter()
            .filter(|v| v.dashboard_id == dashboard_id)
            .cloned()
            .collect()
    }
}

impl Default for DashboardService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_dashboard() {
        let service = DashboardService::new();
        let owner_id = Uuid::new_v4();

        let result = service
            .create_dashboard("Test Dashboard".to_string(), owner_id)
            .await;

        assert!(result.is_ok());
        let dashboard = result.unwrap();
        assert_eq!(dashboard.name, "Test Dashboard");
        assert_eq!(dashboard.owner_id, owner_id);
    }

    #[tokio::test]
    async fn test_add_widget() {
        let service = DashboardService::new();
        let owner_id = Uuid::new_v4();

        let dashboard = service
            .create_dashboard("Test Dashboard".to_string(), owner_id)
            .await
            .unwrap();

        let widget = WidgetConfig {
            id: Uuid::new_v4(),
            widget_type: "LineChart".to_string(),
            title: "Test Widget".to_string(),
            position: (0, 0),
            size: (4, 4),
            config: HashMap::new(),
            data_source: None,
        };

        let result = service.add_widget(dashboard.id, widget).await;
        assert!(result.is_ok());

        let updated = service.get_dashboard(dashboard.id).await.unwrap();
        assert_eq!(updated.widgets.len(), 1);
    }
}
