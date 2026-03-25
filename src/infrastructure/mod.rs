//! Infrastructure layer - Cross-cutting concerns
//!
//! This layer contains infrastructure implementations like logging,
//! tracing, and error handling.

pub mod logging;
pub mod error;
pub mod tracing;

pub use logging::*;
pub use error::*;
