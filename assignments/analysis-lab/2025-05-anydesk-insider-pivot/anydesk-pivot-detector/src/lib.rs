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
    clippy::unused_async,
    clippy::too_many_lines,
    clippy::items_after_statements,
    clippy::cast_precision_loss,
    clippy::cloned_ref_to_slice_refs,
    clippy::unnecessary_debug_formatting,
    clippy::assertions_on_constants
)]
pub mod actions;
pub mod analyzers;
pub mod api;
pub mod config;
pub mod errors;
pub mod models;
pub mod monitors;
pub mod parsers;
pub mod reporters;
pub mod utils;
