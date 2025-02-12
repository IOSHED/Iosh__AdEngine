//! # Interface
//! Manages communication between user interactions and backend.
//! In the <lib_or_framework> module, provide the traits that the
//! <name_interface> module implements. This is the only reason why the library
//! is connected to the `lib.rs`.

use async_trait::async_trait;

pub mod actix;
pub mod lapin;

#[async_trait]
pub trait IServer {
    type ErrorLaunch;

    async fn launch(self) -> Result<(), Self::ErrorLaunch>;
}
