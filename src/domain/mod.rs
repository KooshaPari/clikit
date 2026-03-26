//! Domain layer - Pure business logic
//!
//! This layer contains the core domain model with zero external dependencies.
//! All business rules, entities, and value objects live here.

pub mod entities;
pub mod value_objects;
pub mod services;
pub mod ports;
pub mod events;
pub mod errors;

pub use entities::{Argument, CliOption, Command, Flag};
pub use value_objects::{ArgValue, Context, Input, Output, OutputContent, ParsedInput, Result};
pub use services::{CommandRegistry, InputValidator};
pub use ports::*;
pub use events::*;
pub use errors::*;

pub use ports::CommandExecutor as CommandExecutorPort;
pub use ports::CommandHandler as CommandHandlerPort;
