use crate::{
    AgentError,
    plugins::{Plugin, service::ServicePlugin},
};

pub fn run() -> Result<(), AgentError> {
    run_with_args(std::env::args())
}

pub fn run_with_args<I, T>(args: I) -> Result<(), AgentError>
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

    match args.as_slice() {
        [_binary] => {
            print_help();
            Ok(())
        }
        [_binary, flag] if flag == "--help" || flag == "-h" => {
            print_help();
            Ok(())
        }
        [_binary, flag] if flag == "--version" || flag == "-V" => {
            println!("agentic-devops {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        [_binary, plugin, command] if plugin == "service" && command == "capabilities" => {
            print_service_capabilities();
            Ok(())
        }
        _ => Err(AgentError::InvalidArguments(
            "commande attendue: agentic-devops service capabilities".to_string(),
        )),
    }
}

fn print_help() {
    println!("agentic-devops {}", env!("CARGO_PKG_VERSION"));
    println!("Agent DevOps local avec plugins contrôlés");
    println!();
    println!("USAGE:");
    println!("    agentic-devops service capabilities");
    println!();
    println!("COMMANDS:");
    println!("    service capabilities    Liste les capacités lecture seule du plugin service");
}

fn print_service_capabilities() {
    let metadata = ServicePlugin.metadata();

    for capability in metadata.capabilities {
        println!("{}\t{}", capability.full_name(), capability.description);
    }
}

#[cfg(test)]
mod tests {
    use super::run_with_args;

    #[test]
    fn accepts_service_capabilities_command() {
        let result = run_with_args(["agentic-devops", "service", "capabilities"]);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_unknown_command() {
        let result = run_with_args(["agentic-devops", "service", "restart", "ssh.service"]);

        assert!(result.is_err());
    }
}
