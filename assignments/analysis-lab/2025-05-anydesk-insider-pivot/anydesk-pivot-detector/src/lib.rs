#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::struct_field_names,
    clippy::unused_async
)]
pub mod analyzers;
pub mod api;
pub mod config;
pub mod errors;
pub mod models;
pub mod monitors;
pub mod parsers;
pub mod reporters;
pub mod utils;
