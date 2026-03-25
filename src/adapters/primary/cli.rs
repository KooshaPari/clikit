//! CLI adapter - Command line input parsing
//!
//! Primary adapter for parsing command line arguments.

use crate::domain::{Command, Input, CliParser, Result, DomainError};
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

pub struct ClapParser;

impl ClapParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ClapParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CliParser for ClapParser {
    fn parse(&self, args: Vec<String>) -> Result<Input> {
        let mut iter = args.into_iter().peekable();

        // Skip first element (program name)
        iter.next();

        // Get command name
        let command = iter.next()
            .ok_or_else(|| DomainError::InvalidCommand("Missing command".to_string()))?;

        let mut input = Input::new(command);
        let mut current_arg: Option<String> = None;

        while let Some(arg) = iter.next() {
            if arg.starts_with('-') {
                if arg.starts_with("--") {
                    // Long option
                    let opt_name = arg.trim_start_matches("--");
                    if let Some(next) = iter.peek() {
                        if !next.starts_with('-') {
                            input = input.opt(opt_name, Some(iter.next().unwrap()));
                        } else {
                            input = input.flag(opt_name.to_string(), true);
                        }
                    } else {
                        input = input.flag(opt_name.to_string(), true);
                    }
                } else if arg.len() > 1 {
                    // Short option(s)
                    for c in arg.chars().skip(1) {
                        if let Some(next) = iter.peek() {
                            if !next.starts_with('-') {
                                input = input.opt(&c.to_string(), Some(iter.next().unwrap()));
                            } else {
                                input = input.flag(c.to_string(), true);
                            }
                        } else {
                            input = input.flag(c.to_string(), true);
                        }
                    }
                }
            } else if let Some(name) = current_arg.take() {
                input = input.arg(name, arg.clone());
            } else {
                // Positional argument
                current_arg = Some(arg);
            }
        }

        Ok(input)
    }

    fn format_help(&self, command: &Command) -> String {
        let mut help = format!("{} {}\n\n{}\n\nUsage:\n  {}",
            command.name,
            command.version.as_deref().unwrap_or(""),
            command.description,
            command.name
        );

        if !command.arguments.is_empty() {
            help.push_str(" <arguments>");
        }

        if !command.options.is_empty() {
            help.push_str(" [options]");
        }

        if !command.flags.is_empty() {
            help.push_str(" [flags]");
        }

        if !command.arguments.is_empty() {
            help.push_str("\n\nArguments:");
            for arg in &command.arguments {
                help.push_str(&format!("\n  {}  {}", arg.name, arg.description));
            }
        }

        if !command.options.is_empty() {
            help.push_str("\n\nOptions:");
            for opt in &command.options {
                let opt_str = match opt.short {
                    Some(s) => format!("-{}, --{}", s, opt.long),
                    None => format!("--{}", opt.long),
                };
                help.push_str(&format!("\n  {} <{}>  {}", opt_str, opt.value_name, opt.description));
            }
        }

        if !command.flags.is_empty() {
            help.push_str("\n\nFlags:");
            for flag in &command.flags {
                let flag_str = match flag.short {
                    Some(s) => format!("-{}, --{}", s, flag.long),
                    None => format!("--{}", flag.long),
                };
                help.push_str(&format!("\n  {}  {}", flag_str, flag.description));
            }
        }

        help
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let parser = ClapParser::new();
        let args: Vec<String> = vec!["prog", "greet", "Alice"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let input = parser.parse(args).unwrap();

        assert_eq!(input.command, "greet");
    }

    #[test]
    fn test_parse_with_options() {
        let parser = ClapParser::new();
        let args: Vec<String> = vec!["prog", "greet", "--name", "Alice"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let input = parser.parse(args).unwrap();

        assert_eq!(input.command, "greet");
        assert_eq!(input.get_opt("name"), Some("Alice"));
    }

    #[test]
    fn test_parse_with_flags() {
        let parser = ClapParser::new();
        let args: Vec<String> = vec!["prog", "greet", "--verbose", "--debug"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let input = parser.parse(args).unwrap();

        assert!(input.get_flag("verbose"));
        assert!(input.get_flag("debug"));
    }
}
