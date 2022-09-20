// lints that are ok during dev but not on rls
#![warn(
    // clippy::multiple_crate_versions,
    clippy::pedantic,
    clippy::nursery,
    clippy::else_if_without_else,
    clippy::empty_structs_with_brackets,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::multiple_inherent_impl,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::todo,
    clippy::dbg_macro,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::unnecessary_self_imports
)]
// lints that are never allowed
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::empty_drop,
    clippy::integer_division,
    clippy::same_name_method,
    clippy::string_to_string,
    clippy::try_err,
    clippy::wildcard_enum_match_arm
)]
// kinda useless lint because qualified usages are ugly.
#![allow(clippy::module_name_repetitions)]

mod api;
mod backend;
mod ext;
mod metadata;

// reexports
pub use ext::ServerExt;
