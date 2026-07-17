use accessibility_sys::{
    AXUIElementCopyAttributeValue, AXUIElementCreateApplication, AXUIElementRef,
    AXUIElementSetAttributeValue, AXValueCreate, AXValueGetValue, AXValueRef,
};
use core_foundation::base::{CFRelease, CFTypeRef, TCFType};
use core_foundation::string::CFString;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use std::ffi::c_void;
use std::mem::MaybeUninit;
use thiserror::Error;

// AXValue type constants
const K_AX_VALUE_TYPE_CGPOINT: u32 = 1;
const K_AX_VALUE_TYPE_CGSIZE: u32 = 2;

#[derive(Error, Debug)]
pub enum WindowError {
    #[error("Failed to get window attribute: {0}")]
    AttributeError(String),
    #[error("Failed to set window attribute: {0}")]
    SetAttributeError(String),
    #[error("No focused window found")]
    NoFocusedWindow,
    #[error("Window is not manipulable (may be fullscreen or a system window)")]
    NotManipulable,
}

/// Represents a macOS window that can be manipulated via the Accessibility API
pub struct Window {
    element: AXUIElementRef,
}

impl Window {
    /// Get the currently focused window
    pub fn focused() -> Result<Self, WindowError> {
        unsafe {
            // Get the frontmost application
            let frontmost_app = get_frontmost_application()?;

            // Get the focused window from that application
            let focused_window_attr = CFString::new("AXFocusedWindow");
            let mut window_ref: CFTypeRef = std::ptr::null();

            let result = AXUIElementCopyAttributeValue(
                frontmost_app,
                focused_window_attr.as_concrete_TypeRef(),
                &mut window_ref,
            );

            CFRelease(frontmost_app as CFTypeRef);

            if result != 0 || window_ref.is_null() {
                return Err(WindowError::NoFocusedWindow);
            }

            Ok(Window {
                element: window_ref as AXUIElementRef,
            })
        }
    }

    /// Get the current frame (position and size) of the window
    pub fn frame(&self) -> Result<CGRect, WindowError> {
        let position = self.position()?;
        let size = self.size()?;
        Ok(CGRect::new(&position, &size))
    }

    /// Get the current position of the window
    pub fn position(&self) -> Result<CGPoint, WindowError> {
        unsafe {
            let attr = CFString::new("AXPosition");
            let mut value_ref: CFTypeRef = std::ptr::null();

            let result = AXUIElementCopyAttributeValue(
                self.element,
                attr.as_concrete_TypeRef(),
                &mut value_ref,
            );

            if result != 0 || value_ref.is_null() {
                return Err(WindowError::AttributeError("position".to_string()));
            }

            let ax_value = value_ref as AXValueRef;
            let mut point = MaybeUninit::<CGPoint>::uninit();

            if AXValueGetValue(
                ax_value,
                K_AX_VALUE_TYPE_CGPOINT,
                point.as_mut_ptr() as *mut c_void,
            ) {
                CFRelease(value_ref);
                Ok(point.assume_init())
            } else {
                CFRelease(value_ref);
                Err(WindowError::AttributeError(
                    "failed to extract position value".to_string(),
                ))
            }
        }
    }

    /// Get the current size of the window
    pub fn size(&self) -> Result<CGSize, WindowError> {
        unsafe {
            let attr = CFString::new("AXSize");
            let mut value_ref: CFTypeRef = std::ptr::null();

            let result = AXUIElementCopyAttributeValue(
                self.element,
                attr.as_concrete_TypeRef(),
                &mut value_ref,
            );

            if result != 0 || value_ref.is_null() {
                return Err(WindowError::AttributeError("size".to_string()));
            }

            let ax_value = value_ref as AXValueRef;
            let mut size = MaybeUninit::<CGSize>::uninit();

            if AXValueGetValue(
                ax_value,
                K_AX_VALUE_TYPE_CGSIZE,
                size.as_mut_ptr() as *mut c_void,
            ) {
                CFRelease(value_ref);
                Ok(size.assume_init())
            } else {
                CFRelease(value_ref);
                Err(WindowError::AttributeError(
                    "failed to extract size value".to_string(),
                ))
            }
        }
    }

    /// Set the window frame (position and size)
    /// Note: Position must be set before size on macOS
    pub fn set_frame(&self, frame: CGRect) -> Result<(), WindowError> {
        self.set_position(frame.origin)?;
        self.set_size(frame.size)?;
        Ok(())
    }

    /// Set the window position
    pub fn set_position(&self, position: CGPoint) -> Result<(), WindowError> {
        unsafe {
            let attr = CFString::new("AXPosition");
            let value = AXValueCreate(
                K_AX_VALUE_TYPE_CGPOINT,
                &position as *const CGPoint as *const c_void,
            );

            if value.is_null() {
                return Err(WindowError::SetAttributeError(
                    "failed to create position value".to_string(),
                ));
            }

            let result =
                AXUIElementSetAttributeValue(self.element, attr.as_concrete_TypeRef(), value as _);

            CFRelease(value as CFTypeRef);

            if result != 0 {
                Err(WindowError::SetAttributeError(format!(
                    "position (error {})",
                    result
                )))
            } else {
                Ok(())
            }
        }
    }

    /// Set the window size
    pub fn set_size(&self, size: CGSize) -> Result<(), WindowError> {
        unsafe {
            let attr = CFString::new("AXSize");
            let value = AXValueCreate(
                K_AX_VALUE_TYPE_CGSIZE,
                &size as *const CGSize as *const c_void,
            );

            if value.is_null() {
                return Err(WindowError::SetAttributeError(
                    "failed to create size value".to_string(),
                ));
            }

            let result =
                AXUIElementSetAttributeValue(self.element, attr.as_concrete_TypeRef(), value as _);

            CFRelease(value as CFTypeRef);

            if result != 0 {
                Err(WindowError::SetAttributeError(format!(
                    "size (error {})",
                    result
                )))
            } else {
                Ok(())
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if !self.element.is_null() {
                CFRelease(self.element as CFTypeRef);
            }
        }
    }
}

/// Get the frontmost (focused) application's AXUIElement
unsafe fn get_frontmost_application() -> Result<AXUIElementRef, WindowError> {
    use objc2_app_kit::NSWorkspace;

    let workspace = NSWorkspace::sharedWorkspace();
    let frontmost = workspace.frontmostApplication().ok_or_else(|| {
        WindowError::AttributeError("No frontmost application".to_string())
    })?;

    // Get the process identifier using the selector
    let pid: i32 = objc2::msg_send![&*frontmost, processIdentifier];
    let app_element = AXUIElementCreateApplication(pid);

    if app_element.is_null() {
        return Err(WindowError::AttributeError(
            "Failed to create application element".to_string(),
        ));
    }

    Ok(app_element)
}
