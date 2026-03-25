//! Value objects - Immutable domain values
//!
//! Value objects are immutable objects defined by their attributes.
//! They have no identity and are compared by their values.

use std::collections::HashMap;

/// Input value object for CLI execution
#[derive(Debug, Clone)]
pub struct Input {
    /// Command name
    pub command: String,
    /// Subcommand if any
    pub subcommand: Option<String>,
    /// Arguments as key-value pairs
    pub args: HashMap<String, ArgValue>,
    /// Options as key-value pairs
    pub opts: HashMap<String, Option<String>>,
    /// Flags as key-value pairs
    pub flags: HashMap<String, bool>,
}

impl Input {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            subcommand: None,
            args: HashMap::new(),
            opts: HashMap::new(),
            flags: HashMap::new(),
        }
    }

    pub fn arg(mut self, name: impl Into<String>, value: impl Into<ArgValue>) -> Self {
        self.args.insert(name.into(), value.into());
        self
    }

    pub fn opt(mut self, name: impl Into<String>, value: Option<String>) -> Self {
        self.opts.insert(name.into(), value);
        self
    }

    pub fn flag(mut self, name: impl Into<String>, value: bool) -> Self {
        self.flags.insert(name.into(), value);
        self
    }

    pub fn subcommand(mut self, sub: impl Into<String>) -> Self {
        self.subcommand = Some(sub.into());
        self
    }
}

/// Argument value type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgValue {
    /// Single string value
    String(String),
    /// Integer value
    Integer(i64),
    /// Boolean value
    Boolean(bool),
    /// Multiple values
    Multiple(Vec<String>),
}

impl<T: Into<String>> From<T> for ArgValue {
    fn from(s: T) -> Self {
        ArgValue::String(s.into())
    }
}

/// Output value object for CLI execution
#[derive(Debug, Clone)]
pub struct Output {
    /// Output content
    pub content: OutputContent,
    /// Exit code
    pub exit_code: u8,
    /// Whether to exit immediately
    pub should_exit: bool,
}

impl Output {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: OutputContent::Text(content.into()),
            exit_code: 0,
            should_exit: true,
        }
    }

    pub fn json<T: serde::Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        let json = serde_json::to_string_pretty(value)?;
        Ok(Self {
            content: OutputContent::Json(json),
            exit_code: 0,
            should_exit: true,
        })
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self {
            content: OutputContent::Error(content.into()),
            exit_code: 1,
            should_exit: true,
        }
    }

    pub fn exit_code(mut self, code: u8) -> Self {
        self.exit_code = code;
        self
    }

    pub fn no_exit(mut self) -> Self {
        self.should_exit = false;
        self
    }
}

/// Output content types
#[derive(Debug, Clone)]
pub enum OutputContent {
    Text(String),
    Json(String),
    Yaml(String),
    Error(String),
    None,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            content: OutputContent::None,
            exit_code: 0,
            should_exit: false,
        }
    }
}

impl Default for OutputContent {
    fn default() -> Self {
        OutputContent::None
    }
}

/// Execution context passed to handlers
#[derive(Debug, Clone)]
pub struct Context {
    /// Parsed input
    pub input: Input,
    /// Working directory
    pub cwd: std::path::PathBuf,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Config values
    pub config: HashMap<String, serde_json::Value>,
}

impl Context {
    pub fn new(input: Input) -> Self {
        Self {
            input,
            cwd: std::env::current_dir().unwrap_or_default(),
            env: std::env::vars().collect(),
            config: HashMap::new(),
        }
    }

    pub fn get_arg(&self, name: &str) -> Option<&ArgValue> {
        self.input.args.get(name)
    }

    pub fn get_str(&self, name: &str) -> Option<&str> {
        match self.input.args.get(name) {
            Some(ArgValue::String(s)) => Some(s),
            _ => None,
        }
    }

    pub fn get_opt(&self, name: &str) -> Option<&str> {
        self.input.opts.get(name).and_then(|o| o.as_deref())
    }

    pub fn get_flag(&self, name: &str) -> bool {
        self.input.flags.get(name).copied().unwrap_or(false)
    }
}

/// Result type for domain operations
pub type Result<T> = std::result::Result<T, super::errors::DomainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_builder() {
        let input = Input::new("greet")
            .arg("name", "Alice")
            .opt("verbose", Some("true".to_string()))
            .flag("debug", true);

        assert_eq!(input.command, "greet");
        assert_eq!(input.get_str("name"), Some("Alice"));
    }

    #[test]
    fn test_output_builder() {
        let output = Output::text("Hello, World!").exit_code(0);

        match output.content {
            OutputContent::Text(s) => assert_eq!(s, "Hello, World!"),
            _ => panic!("Expected Text content"),
        }
    }
}
