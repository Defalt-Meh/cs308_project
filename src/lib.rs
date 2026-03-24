//! # CS 308 Online Store — Library Root
//!
//! Re-exports all modules so that integration tests (in `tests/`)
//! can access internals without going through `main`. The binary
//! crate (`main.rs`) also uses these via `mod` declarations.

pub mod config;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;