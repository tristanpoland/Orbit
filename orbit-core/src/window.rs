use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::HWND;

/// Information about a window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    /// Native window handle (as usize for serialization)
    pub hwnd: usize,
    /// Process ID that owns this window
    pub process_id: u32,
    /// Window title
    pub title: String,
    /// Window bounds (screen coordinates)
    pub bounds: Rectangle,
    /// Current window state
    pub state: WindowState,
    /// Whether this window has custom decorations
    pub has_custom_decorations: bool,
}

impl WindowInfo {
    pub fn hwnd(&self) -> HWND {
        HWND(self.hwnd as isize)
    }
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Actions that can be performed on a window
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowAction {
    Minimize,
    Maximize,
    Restore,
    Close,
    StartDrag,
    StartResize(ResizeEdge),
}

/// Edge being resized
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResizeEdge {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Rectangle in screen coordinates
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }
}

/// Point in screen coordinates
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Size dimensions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
