//! Command handlers - Write operations
//!
//! Commands represent write operations in the application.

use async_trait::async_trait;
use crate::domain::{Command, Input, Output, Context, Result, CommandHandler};

/// Default command handler implementation
pub struct DefaultHandler<F, Fut>
where
    F: Fn(&Context) -> Fut,
    Fut: std::future::Future<Output = Result<Output>>,
{
    handler: F,
}

impl<F, Fut> DefaultHandler<F, Fut>
where
    F: Fn(&Context) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<Output>> + Send,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait]
impl<F, Fut> CommandHandler for DefaultHandler<F, Fut>
where
    F: Fn(&Context) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<Output>> + Send,
{
    async fn handle(&self, ctx: &Context) -> Result<Output> {
        (self.handler)(ctx).await
    }
}

/// Command executor that coordinates command execution
pub struct CommandExecutor {
    commands: Vec<Command>,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn register(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn list_commands(&self) -> Vec<&Command> {
        self.commands.iter().collect()
    }
}

impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Input;

    #[tokio::test]
    async fn test_handler_execution() {
        let handler = DefaultHandler::new(|ctx: &Context| async move {
            let name = ctx.get_str("name").unwrap_or("World");
            Ok(Output::text(format!("Hello, {}!", name)))
        });

        let input = Input::new("greet").arg("name", "Alice");
        let ctx = Context::new(input);

        let output = handler.handle(&ctx).await.unwrap();

        match output.content {
            crate::domain::OutputContent::Text(s) => {
                assert_eq!(s, "Hello, Alice!");
            }
            _ => panic!("Expected text output"),
        }
    }
}
