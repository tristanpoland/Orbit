use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrbitError {
    #[error("Windows API error: {0}")]
    WindowsApi(String),
    
    #[error("IPC error: {0}")]
    Ipc(String),
    
    #[error("Window error: {0}")]
    Window(String),
    
    #[error("Service error: {0}")]
    Service(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<windows::core::Error> for OrbitError {
    fn from(err: windows::core::Error) -> Self {
        OrbitError::WindowsApi(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, OrbitError>;
