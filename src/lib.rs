//! clikit - Universal CLI Framework
//!
//! A hexagonal architecture CLI framework for building robust, extensible
//! command-line applications.
//!
//! # Architecture
//!
//! clikit follows hexagonal (ports & adapters) architecture:
//!
//! - **Domain**: Pure business logic with no external dependencies
//! - **Application**: Use cases that orchestrate domain logic
//! - **Adapters**: Infrastructure implementations (CLI, config, logging)
//!
//! # Example
//!
//! ```
//! use clikit::prelude::*;
//!
//! let app = cli!("myapp", "1.0.0")
//!     .command("greet", |c| c
//!         .arg("name", |a| a.default_value("World"))
//!         .handler(|ctx| {
//!             Ok(format!("Hello, {}!", ctx.get::<String>("name")))
//!         })
//!     );
//!
//! app.run();
//! ```

pub mod domain;
pub mod application;
pub mod adapters;
pub mod infrastructure;
pub mod plugins;

pub mod prelude {
    //! Re-exports commonly used types
    pub use crate::domain::entities::*;
    pub use crate::domain::value_objects::*;
    pub use crate::domain::ports::*;
    pub use crate::domain::services::*;
    pub use crate::application::commands::*;
    pub use crate::application::queries::*;
    pub use crate::adapters::primary::cli::*;
    pub use crate::adapters::secondary::config::*;
}

pub use domain::entities::{Command, Argument, CliOption, Flag};
pub use domain::value_objects::{Input, Output, Context, Result};
pub use application::commands::DefaultHandler;
pub use application::queries::QueryHandler;
