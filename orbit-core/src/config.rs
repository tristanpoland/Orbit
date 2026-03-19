use serde::{Deserialize, Serialize};

/// Configuration for Orbit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitConfig {
    /// Enable debug mode
    pub debug: bool,
    
    /// Enable safe mode (launch Explorer on errors)
    pub safe_mode: bool,
    
    /// Path to orbit-ui executable
    pub ui_path: String,
    
    /// Window decoration settings
    pub decorations: DecorationConfig,
    
    /// Compositor settings
    pub compositor: CompositorConfig,
}

impl Default for OrbitConfig {
    fn default() -> Self {
        Self {
            debug: false,
            safe_mode: true,
            ui_path: "orbit-ui.exe".to_string(),
            decorations: DecorationConfig::default(),
            compositor: CompositorConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationConfig {
    /// Title bar height in pixels
    pub title_bar_height: u32,
    
    /// Border width in pixels
    pub border_width: u32,
    
    /// Button size
    pub button_size: u32,
}

impl Default for DecorationConfig {
    fn default() -> Self {
        Self {
            title_bar_height: 32,
            border_width: 1,
            button_size: 32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositorConfig {
    /// Target frames per second
    pub fps: u32,
    
    /// Enable vsync
    pub vsync: bool,
    
    /// Maximum cached textures
    pub max_textures: usize,
}

impl Default for CompositorConfig {
    fn default() -> Self {
        Self {
            fps: 60,
            vsync: true,
            max_textures: 100,
        }
    }
}
