#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::similar_names)]

pub mod actions;
pub mod agent;
pub mod api;
pub mod cipherstring;
pub mod config;
pub mod dirs;
mod error;
pub mod identity;
pub mod locked;
pub mod pinentry;
mod prelude;
// pub mod secrets;
