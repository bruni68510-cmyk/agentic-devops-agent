pub mod adapters;
pub mod cli;
pub mod core;
pub mod plugins;

pub use crate::core::error::AgentError;

pub fn run() -> Result<(), AgentError> {
    crate::core::audit::init_audit();
    crate::cli::run()
}
