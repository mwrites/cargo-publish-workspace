/// Common result type
pub type Result<T> = anyhow::Result<T>;

/// Common error type
pub type Error = anyhow::Error;

pub use anyhow::{
    bail,
    Context
};
