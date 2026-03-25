//! Domain services - Domain logic
//!
//! Services that contain domain logic that doesn't belong to a single entity.

use crate::domain::{Command, Result, DomainError};

/// Command registry service
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn register(&mut self, command: Command) -> Result<()> {
        // Check for duplicate names
        if self.commands.iter().any(|c| c.name == command.name) {
            return Err(DomainError::InvalidCommand(format!(
                "Command '{}' already registered",
                command.name
            )));
        }

        self.commands.push(command);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn list(&self) -> Vec<&Command> {
        self.commands.iter().collect()
    }

    pub fn find_matching(&self, name: &str) -> Vec<&Command> {
        self.commands.iter()
            .filter(|c| c.name.contains(name) || c.description.contains(name))
            .collect()
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Input validation service
pub struct InputValidator {
    commands: Vec<Command>,
}

impl InputValidator {
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }

    pub fn validate(&self, input: &crate::domain::Input) -> Result<()> {
        // Find matching command
        let command = self.commands.iter()
            .find(|c| c.name == input.command)
            .ok_or_else(|| DomainError::CommandNotFound(input.command.clone()))?;

        // Check required arguments
        for arg in &command.arguments {
            if arg.required {
                if !input.args.contains_key(&arg.name) {
                    return Err(DomainError::MissingArgument(arg.name.clone()));
                }
            }
        }

        Ok(())
    }
}
