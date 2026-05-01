#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
pub mod analyzers;
pub mod api;
pub mod config;
pub mod errors;
pub mod models;
pub mod monitors;
pub mod parsers;
pub mod reporters;
pub mod utils;
