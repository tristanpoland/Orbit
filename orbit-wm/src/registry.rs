use std::collections::HashMap;
use windows::Win32::Foundation::HWND;
use orbit_core::WindowInfo;

/// Thread-safe registry of all windows
pub struct WindowRegistry {
    windows: HashMap<isize, WindowInfo>,
}

impl WindowRegistry {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, info: WindowInfo) {
        self.windows.insert(info.hwnd as isize, info);
    }
    
    pub fn remove(&mut self, hwnd: HWND) -> Option<WindowInfo> {
        self.windows.remove(&hwnd.0)
    }
    
    pub fn get(&self, hwnd: HWND) -> Option<&WindowInfo> {
        self.windows.get(&hwnd.0)
    }
    
    pub fn get_mut(&mut self, hwnd: HWND) -> Option<&mut WindowInfo> {
        self.windows.get_mut(&hwnd.0)
    }
    
    pub fn all(&self) -> Vec<&WindowInfo> {
        self.windows.values().collect()
    }
}

impl Default for WindowRegistry {
    fn default() -> Self {
        Self::new()
    }
}
