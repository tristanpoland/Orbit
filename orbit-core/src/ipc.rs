use serde::{Deserialize, Serialize};
use crate::window::{WindowInfo, Point};

/// Messages sent over IPC between service and UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcMessage {
    /// A new window was created
    WindowCreated(WindowInfo),
    
    /// A window was destroyed
    WindowDestroyed { hwnd: usize },
    
    /// A window was moved
    WindowMoved { hwnd: usize, position: Point },
    
    /// A window was resized
    WindowResized { hwnd: usize, width: u32, height: u32 },
    
    /// Window z-order changed
    WindowZOrderChanged { hwnd: usize, z_index: i32 },
    
    /// Service status update
    ServiceStatus(ServiceStatus),
    
    /// Ping/keepalive
    Ping,
    
    /// Pong response
    Pong,
}

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub dwm_disabled: bool,
    pub explorer_disabled: bool,
    pub hooks_installed: bool,
}

/// IPC client (placeholder - will be implemented later)
pub struct IpcClient;

impl IpcClient {
    pub fn connect() -> crate::Result<Self> {
        todo!("IPC client implementation")
    }
    
    pub fn send(&self, _message: IpcMessage) -> crate::Result<()> {
        todo!("IPC send implementation")
    }
    
    pub fn recv(&self) -> crate::Result<IpcMessage> {
        todo!("IPC recv implementation")
    }
}

/// IPC server (placeholder - will be implemented later)
pub struct IpcServer;

impl IpcServer {
    pub fn start() -> crate::Result<Self> {
        todo!("IPC server implementation")
    }
    
    pub fn accept(&self) -> crate::Result<IpcClient> {
        todo!("IPC accept implementation")
    }
}
