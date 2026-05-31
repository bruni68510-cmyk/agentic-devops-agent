use crate::core::capability::Capability;

pub fn service_capabilities() -> Vec<Capability> {
    vec![
        Capability::read_only("service", "list", "Lister les services visibles"),
        Capability::read_only("service", "status", "Afficher l'état d'un service"),
        Capability::read_only("service", "failed", "Lister les services en échec"),
        Capability::read_only(
            "service",
            "logs_recent",
            "Lire un extrait borné des logs récents d'un service",
        ),
    ]
}
