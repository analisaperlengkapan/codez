//! Routing and navigation for SuperApp

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Route definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: Uuid,
    pub path: String,
    pub name: String,
    pub component: String,
    pub module: String,
    pub children: Vec<Route>,
    pub guards: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Navigation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationEvent {
    pub from: String,
    pub to: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Router
pub struct Router {
    routes: Vec<Route>,
    current_path: String,
    history: Vec<NavigationEvent>,
}

impl Router {
    /// Create new router
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            current_path: "/".to_string(),
            history: Vec::new(),
        }
    }

    /// Add route
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Navigate to path
    pub fn navigate(&mut self, path: String) -> Result<(), String> {
        if self.find_route(&path).is_some() {
            let event = NavigationEvent {
                from: self.current_path.clone(),
                to: path.clone(),
                timestamp: chrono::Utc::now(),
            };
            self.history.push(event);
            self.current_path = path;
            Ok(())
        } else {
            Err(format!("Route not found: {}", path))
        }
    }

    /// Find route by path
    pub fn find_route(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|r| r.path == path)
    }

    /// Get current path
    pub fn current_path(&self) -> &str {
        &self.current_path
    }

    /// Get navigation history
    pub fn history(&self) -> &[NavigationEvent] {
        &self.history
    }

    /// Go back
    pub fn go_back(&mut self) -> Result<(), String> {
        if self.history.len() > 1 {
            let last_event = self.history.pop().unwrap();
            self.current_path = last_event.from;
            Ok(())
        } else {
            Err("No history to go back".to_string())
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Route {
    /// Create new route
    pub fn new(path: String, name: String, component: String, module: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            path,
            name,
            component,
            module,
            children: Vec::new(),
            guards: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add child route
    pub fn add_child(&mut self, route: Route) {
        self.children.push(route);
    }

    /// Add guard
    pub fn add_guard(&mut self, guard: String) {
        self.guards.push(guard);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_navigation() {
        let mut router = Router::new();
        let route = Route::new(
            "/dashboard".to_string(),
            "Dashboard".to_string(),
            "Dashboard".to_string(),
            "dashboard".to_string(),
        );

        router.add_route(route);
        let result = router.navigate("/dashboard".to_string());

        assert!(result.is_ok());
        assert_eq!(router.current_path(), "/dashboard");
    }

    #[test]
    fn test_router_history() {
        let mut router = Router::new();
        let route1 = Route::new(
            "/home".to_string(),
            "Home".to_string(),
            "Home".to_string(),
            "home".to_string(),
        );
        let route2 = Route::new(
            "/about".to_string(),
            "About".to_string(),
            "About".to_string(),
            "about".to_string(),
        );

        router.add_route(route1);
        router.add_route(route2);

        let _ = router.navigate("/home".to_string());
        let _ = router.navigate("/about".to_string());

        assert_eq!(router.history().len(), 2);
    }
}
