//! Service layer for Orchestrator logic
use std::collections::HashMap;
use crate::superapp::SuperApp;
use codeza_mfe_manager::mfe::{MFEManifest, SharedConfig, MicroFrontend};

/// Generate MFE Manifest for a SuperApp based on active MFEs
pub fn generate_manifest(
    app: &SuperApp,
    active_mfes: &HashMap<String, MicroFrontend>,
) -> MFEManifest {
    let mut remotes = HashMap::new();

    for module in &app.modules {
        // Try to fetch latest details from registry
        let remote_entry = if let Some(mfe) = active_mfes.get(&module.name) {
            // Use registered URL
            mfe.remote_entry.clone()
        } else {
            // Fallback to configured URL
            module.remote_entry.clone()
        };

        // Format: "scope": "url"
        remotes.insert(module.scope.clone(), remote_entry);
    }

    // Use shared dependencies from SuperApp config if available, otherwise use defaults
    let mut shared = app.config.shared_dependencies.clone();

    if shared.is_empty() {
        shared.insert("react".to_string(), SharedConfig {
            singleton: true,
            strict_version: true,
            eager: true,
            required_version: Some("^18.0.0".to_string()),
        });
        shared.insert("react-dom".to_string(), SharedConfig {
            singleton: true,
            strict_version: true,
            eager: true,
            required_version: Some("^18.0.0".to_string()),
        });
    }

    MFEManifest {
        name: app.name.clone(),
        version: app.version.clone(),
        remotes,
        exposes: HashMap::new(), // SuperApp usually doesn't expose, it consumes
        shared,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::superapp::AppModule;

    use codeza_mfe_manager::mfe::MFEStatus;

    #[test]
    fn test_generate_manifest_resolves_remotes() {
        let mut app = SuperApp::new("my-superapp".to_string(), "1.0.0".to_string());

        // Add a module with a fallback URL
        app.add_module(AppModule::new(
            "dashboard".to_string(),
            "1.0.0".to_string(),
            "http://fallback.com/remoteEntry.js".to_string(),
            "@dashboard".to_string()
        ));

        // Case 1: MFE is not active/registered -> should use fallback
        let active_mfes = HashMap::new();
        let manifest = generate_manifest(&app, &active_mfes);

        assert_eq!(manifest.remotes.get("@dashboard").unwrap(), "http://fallback.com/remoteEntry.js");

        // Case 2: MFE is active -> should use registry URL
        let mut active_mfes = HashMap::new();
        let mut mfe = MicroFrontend::new(
            "dashboard".to_string(),
            "1.2.0".to_string(),
            "http://registry.com/remoteEntry.js".to_string(),
            "@dashboard".to_string()
        );
        mfe.status = MFEStatus::Active;
        active_mfes.insert("dashboard".to_string(), mfe);

        let manifest = generate_manifest(&app, &active_mfes);
        assert_eq!(manifest.remotes.get("@dashboard").unwrap(), "http://registry.com/remoteEntry.js");
    }

    #[test]
    fn test_generate_manifest_default_shared() {
        let app = SuperApp::new("my-superapp".to_string(), "1.0.0".to_string());
        let active_mfes = HashMap::new();
        let manifest = generate_manifest(&app, &active_mfes);

        assert!(manifest.shared.contains_key("react"));
        assert!(manifest.shared.contains_key("react-dom"));
    }
}
