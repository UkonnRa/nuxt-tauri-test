mod aggregate;
mod command;
pub mod database;
mod presentation;
mod query;

pub use aggregate::Aggregate;
pub use command::*;
pub use database::journal::Entity;
pub use query::Query;

pub const TYPE: &str = "Journal";
