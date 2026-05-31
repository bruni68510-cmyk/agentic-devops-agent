use crate::core::capability::PluginMetadata;

pub mod service;

pub trait Plugin {
    fn metadata(&self) -> PluginMetadata;
}
