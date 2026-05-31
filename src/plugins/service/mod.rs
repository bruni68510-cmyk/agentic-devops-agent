use crate::{
    core::capability::PluginMetadata,
    plugins::{Plugin, service::capability::service_capabilities},
};

pub mod capability;

#[derive(Debug, Default)]
pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "service".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: service_capabilities(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plugins::{Plugin, service::ServicePlugin};

    #[test]
    fn service_plugin_exposes_read_only_capabilities() {
        let metadata = ServicePlugin.metadata();
        let names: Vec<_> = metadata
            .capabilities
            .iter()
            .map(|capability| capability.full_name())
            .collect();

        assert_eq!(metadata.name, "service");
        assert_eq!(
            names,
            vec![
                "service.list",
                "service.status",
                "service.failed",
                "service.logs_recent",
            ]
        );
    }
}
