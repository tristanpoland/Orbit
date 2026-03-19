//! Orbit Core Library
//! 
//! Common types, IPC protocol, and utilities shared across all Orbit components.

pub mod ipc;
pub mod window;
pub mod config;
pub mod error;

pub use error::{OrbitError, Result};
pub use window::{WindowInfo, WindowState, WindowAction};
pub use ipc::{IpcMessage, IpcClient, IpcServer};
pub use config::OrbitConfig;

/// Initialize logging for the application
pub fn init_logging(component: &str) -> Result<()> {
    use tracing_subscriber::{fmt, EnvFilter};
    
    fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(false)
        .init();
    
    tracing::info!("Initialized {} logging", component);
    Ok(())
}
