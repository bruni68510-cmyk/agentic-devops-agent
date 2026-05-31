use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum AgentError {
    UnknownCapability(String),
    PolicyDenied(String),
    InvalidArguments(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCapability(capability) => {
                write!(formatter, "capacité inconnue: {capability}")
            }
            Self::PolicyDenied(reason) => {
                write!(formatter, "action refusée par la politique: {reason}")
            }
            Self::InvalidArguments(reason) => write!(formatter, "arguments invalides: {reason}"),
        }
    }
}

impl Error for AgentError {}
