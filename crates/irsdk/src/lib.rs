#[cfg(target_family = "windows")]
mod client;
#[cfg(target_family = "windows")]
mod win;

pub use ibt;

#[cfg(target_family = "windows")]
pub use client::{IRacingClient, IRacingClientError};
