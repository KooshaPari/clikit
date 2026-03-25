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

pub use entities::*;
pub use value_objects::*;
pub use services::*;
pub use ports::*;
pub use events::*;
pub use errors::*;
