#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolicyRequest {
    pub plugin: String,
    pub capability: String,
    pub action: String,
    pub resource: Option<String>,
    pub mode: PolicyMode,
    pub local_identity: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PolicyMode {
    Diagnostic,
    ControlledAction,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PolicyDecision {
    Allow,
    Deny { reason: String },
}

impl PolicyDecision {
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }
}

pub trait PolicyEngine {
    fn evaluate(&self, request: &PolicyRequest) -> PolicyDecision;
}

#[derive(Debug, Default)]
pub struct DenyByDefaultPolicy;

impl PolicyEngine for DenyByDefaultPolicy {
    fn evaluate(&self, request: &PolicyRequest) -> PolicyDecision {
        PolicyDecision::Deny {
            reason: format!(
                "aucune règle explicite n'autorise {}.{} sur {:?}",
                request.plugin, request.capability, request.resource
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DenyByDefaultPolicy, PolicyDecision, PolicyEngine, PolicyMode, PolicyRequest};

    #[test]
    fn deny_by_default_refuses_unknown_actions() {
        let request = PolicyRequest {
            plugin: "service".to_string(),
            capability: "restart".to_string(),
            action: "restart_service".to_string(),
            resource: Some("ssh.service".to_string()),
            mode: PolicyMode::ControlledAction,
            local_identity: Some("agentic-devops".to_string()),
        };

        let decision = DenyByDefaultPolicy.evaluate(&request);

        assert!(matches!(decision, PolicyDecision::Deny { .. }));
        assert!(!decision.is_allowed());
    }
}
