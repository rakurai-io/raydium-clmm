//! Raydium CLMM client library.
//!
//! Example: `client::instructions::utils::compute_swap` for off-chain swap stepping.

pub mod config;
pub mod instructions;

pub use instructions::utils::compute_swap;
