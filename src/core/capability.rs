#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AccessMode {
    ReadOnly,
    Mutating,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Capability {
    pub plugin: String,
    pub name: String,
    pub description: String,
    pub access_mode: AccessMode,
}

impl Capability {
    pub fn read_only(
        plugin: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            plugin: plugin.into(),
            name: name.into(),
            description: description.into(),
            access_mode: AccessMode::ReadOnly,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{}.{}", self.plugin, self.name)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub capabilities: Vec<Capability>,
}

#[cfg(test)]
mod tests {
    use super::{AccessMode, Capability};

    #[test]
    fn builds_full_capability_name() {
        let capability = Capability::read_only("service", "list", "Lister les services");

        assert_eq!(capability.full_name(), "service.list");
        assert_eq!(capability.access_mode, AccessMode::ReadOnly);
    }
}
