//! Orbit Window Manager
//! 
//! System-wide window hooks for decoration removal and window tracking.

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use orbit_core::{WindowInfo, Result};

mod registry;
mod hooks;

pub use registry::WindowRegistry;

/// Global window registry (thread-safe)
static WINDOW_REGISTRY: once_cell::sync::Lazy<Arc<RwLock<HashMap<isize, WindowInfo>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

/// Initialize window manager and install hooks
pub fn initialize() -> Result<()> {
    tracing::info!("Initializing window manager");
    hooks::install_hooks()?;
    Ok(())
}

/// Shutdown window manager and remove hooks
pub fn shutdown() -> Result<()> {
    tracing::info!("Shutting down window manager");
    hooks::uninstall_hooks()?;
    Ok(())
}

/// Get information about a window
pub fn get_window_info(hwnd: HWND) -> Option<WindowInfo> {
    let registry = WINDOW_REGISTRY.read().ok()?;
    registry.get(&hwnd.0).cloned()
}

/// Get all tracked windows
pub fn get_all_windows() -> Vec<WindowInfo> {
    WINDOW_REGISTRY
        .read()
        .map(|registry| registry.values().cloned().collect())
        .unwrap_or_default()
}
