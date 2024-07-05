mod aggregate;
mod command;
mod database;
mod presentation;
mod query;

pub use aggregate::Aggregate;
pub use command::Command;
pub use database::journal::Entity;
pub use query::Query;

pub const TYPE: &str = "Journal";
