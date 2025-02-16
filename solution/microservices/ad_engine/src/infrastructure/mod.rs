//! # Infrastructure
//! Contains components for interaction with external resources.
//! In the `<name_infra>` module, provide the traits that the
//! `<lib_or_framefork>` module implements. This is the only reason why the
//! library is connected to the `lib.rs`.

pub mod configurate;
pub mod database_connection;

pub mod logger;
pub mod repository;

pub mod cash;
pub mod metrics;
