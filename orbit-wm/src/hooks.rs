use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use orbit_core::Result;

static mut HOOK_HANDLE: Option<HHOOK> = None;

/// Install system-wide window hooks
pub fn install_hooks() -> Result<()> {
    unsafe {
        let hook = SetWindowsHookExW(
            WH_CBT,
            Some(cbt_hook_proc),
            None,
            0, // 0 = system-wide hook
        )?;
        
        HOOK_HANDLE = Some(hook);
        tracing::info!("Installed CBT hook: {:?}", hook);
    }
    
    Ok(())
}

/// Uninstall window hooks
pub fn uninstall_hooks() -> Result<()> {
    unsafe {
        if let Some(hook) = HOOK_HANDLE.take() {
            UnhookWindowsHookEx(hook)?;
            tracing::info!("Uninstalled CBT hook");
        }
    }
    
    Ok(())
}

/// CBT hook procedure - called for window events
/// 
/// SAFETY: This is called by Windows for every window operation system-wide.
/// Must be extremely fast and never panic.
unsafe extern "system" fn cbt_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // If code < 0, must call CallNextHookEx without processing
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    
    match code {
        HCBT_CREATEWND => {
            // Window is being created
            let hwnd = HWND(wparam.0 as isize);
            
            // Remove native decorations
            if let Err(e) = remove_decorations(hwnd) {
                tracing::error!("Failed to remove decorations for {:?}: {}", hwnd, e);
            }
            
            // Track window (TODO: send IPC message)
            tracing::debug!("Window created: {:?}", hwnd);
        }
        
        HCBT_DESTROYWND => {
            // Window is being destroyed
            let hwnd = HWND(wparam.0 as isize);
            tracing::debug!("Window destroyed: {:?}", hwnd);
            
            // Remove from registry (TODO: send IPC message)
        }
        
        _ => {}
    }
    
    // Always call next hook
    CallNextHookEx(None, code, wparam, lparam)
}

/// Remove native window decorations
/// 
/// SAFETY: Must be called during window creation (HCBT_CREATEWND)
unsafe fn remove_decorations(hwnd: HWND) -> Result<()> {
    // Get current style
    let style = GetWindowLongW(hwnd, GWL_STYLE);
    
    // Remove caption, frame, and system menu
    let new_style = style & !(WS_CAPTION.0 as i32 | WS_THICKFRAME.0 as i32 | WS_SYSMENU.0 as i32);
    
    // Set new style
    SetWindowLongW(hwnd, GWL_STYLE, new_style);
    
    // Force window to redraw with new style
    SetWindowPos(
        hwnd,
        None,
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
    )?;
    
    Ok(())
}
