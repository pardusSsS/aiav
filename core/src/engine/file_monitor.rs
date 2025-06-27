use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::os::windows::ffi::{OsStrExt as _, OsStringExt}; // For converting wide chars to OsString
use log::{info, warn, error};
use crate::engine::scanner::Scanner;
use crate::error::AvError;

// FFI bindings for the C++ WindowsFileWatcher
#[link(name = "system_integration", kind = "static")] // Assuming static linking or adjust to "dylib" for dynamic
extern "C" {
    fn create_windows_file_watcher() -> *mut std::os::raw::c_void;
    fn destroy_windows_file_watcher(watcher_ptr: *mut std::os::raw::c_void);
    fn start_watching_path(watcher_ptr: *mut std::os::raw::c_void, path_wstr: *const u16, watch_subdirectories: bool) -> bool;
    fn stop_watching_path(watcher_ptr: *mut std::os::raw::c_void);
    fn set_rust_file_event_callback(watcher_ptr: *mut std::os::raw::c_void, callback: extern "C" fn(i32, *const u16, *const u16));
}

// Map C++ enum to Rust enum
#[repr(i32)]
#[derive(Debug)]
pub enum FileEventType {
    FileAdded = 0,
    FileRemoved = 1,
    FileModified = 2,
    FileRenamedOldName = 3,
    FileRenamedNewName = 4,
}

// Global static mutable variable to hold the Arc<Scanner>
// This is necessary because the C++ callback is a raw C function pointer and cannot capture context directly.
// We use a Mutex to ensure thread-safe access.
static mut GLOBAL_SCANNER: Option<Arc<Scanner>> = None;
static GLOBAL_CALLBACK_LOCK: Mutex<()> = Mutex::new(()); // To protect GLOBAL_SCANNER during mutation

// Callback function exposed to C++
extern "C" fn file_event_callback_c(event_type_raw: i32, path_wstr: *const u16, old_path_wstr: *const u16) {
    let _guard = GLOBAL_CALLBACK_LOCK.lock().unwrap(); // Acquire lock for thread safety

    if let Some(scanner) = unsafe { GLOBAL_SCANNER.as_ref() } {
        let event_type = match FileEventType::try_from(event_type_raw) {
            Ok(et) => et,
            Err(_) => {
                error!("Unknown file event type received from C++: {}", event_type_raw);
                return;
            }
        };

        let path = unsafe {
            let len = (0..).take_while(|&i| *path_wstr.offset(i) != 0).count();
            String::from_utf16_lossy(std::slice::from_raw_parts(path_wstr, len))
        };
        let path_buf = PathBuf::from(path);

        let old_path = if !old_path_wstr.is_null() {
            unsafe {
                let len = (0..).take_while(|&i| *old_path_wstr.offset(i) != 0).count();
                Some(String::from_utf16_lossy(std::slice::from_raw_parts(old_path_wstr, len)))
            }
        } else {
            None
        };

        info!("File event received: {:?} for path: {:?}", event_type, path_buf);

        // Perform actions based on event type
        match event_type {
            FileEventType::FileAdded | FileEventType::FileModified => {
                info!("Scanning file due to modification/addition: {:?}", path_buf);
                // Perform a scan in a non-blocking way
                let scanner_clone = scanner.clone();
                tokio::spawn(async move {
                    match scanner_clone.scan_file(&path_buf).await {
                        Ok(scan_result) => {
                            if scan_result.is_malicious {
                                warn!("Malicious file detected: {:?} - {:?}", path_buf, scan_result.threat_info.unwrap_or_default().description);
                                // Further actions like quarantine will be handled by the scanner
                            } else {
                                info!("File {:?} is clean.", path_buf);
                            }
                        }
                        Err(e) => {
                            error!("Error scanning file {:?}: {:?}", path_buf, e);
                        }
                    }
                });
            },
            FileEventType::FileRemoved => {
                info!("File removed: {:?}", path_buf);
                // Optional: Update internal state, remove from cache etc.
            },
            FileEventType::FileRenamedOldName => {
                info!("File renamed (old name): {:?}", path_buf);
            },
            FileEventType::FileRenamedNewName => {
                info!("File renamed (new name): {:?}, old name: {:?}", path_buf, old_path.unwrap_or_default());
                // After rename, new file might need to be scanned
                let scanner_clone = scanner.clone();
                tokio::spawn(async move {
                    match scanner_clone.scan_file(&path_buf).await {
                        Ok(scan_result) => {
                            if scan_result.is_malicious {
                                warn!("Malicious renamed file detected: {:?} - {:?}", path_buf, scan_result.threat_info.unwrap_or_default().description);
                            }
                        }
                        Err(e) => {
                            error!("Error scanning renamed file {:?}: {:?}", path_buf, e);
                        }
                    }
                });
            },
        }
    } else {
        error!("Scanner not initialized in GLOBAL_SCANNER for file event callback.");
    }
}

// Implement TryFrom for FileEventType

impl TryFrom<i32> for FileEventType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FileEventType::FileAdded),
            1 => Ok(FileEventType::FileRemoved),
            2 => Ok(FileEventType::FileModified),
            3 => Ok(FileEventType::FileRenamedOldName),
            4 => Ok(FileEventType::FileRenamedNewName),
            _ => Err(()),
        }
    }
}

pub struct FileMonitor {
    watcher_ptr: *mut std::os::raw::c_void,
}

impl FileMonitor {
    pub fn new(scanner: Arc<Scanner>) -> Result<Self, AvError> {
        // Initialize the global scanner. This is a critical section.
        let _guard = GLOBAL_CALLBACK_LOCK.lock().unwrap();
        unsafe {
            if GLOBAL_SCANNER.is_some() {
                // If already initialized, it means a FileMonitor was created before.
                // This might indicate a logic error or a need for a singleton pattern.
                warn!("GLOBAL_SCANNER is already initialized. This might be a re-initialization.");
            }
            GLOBAL_SCANNER = Some(scanner);
        }

        let watcher_ptr = unsafe { create_windows_file_watcher() };
        if watcher_ptr.is_null() {
            error!("Failed to create Windows file watcher.");
            return Err(AvError::Other("Failed to initialize file monitor".to_string()));
        }

        unsafe {
            set_rust_file_event_callback(watcher_ptr, file_event_callback_c);
        }

        Ok(FileMonitor { watcher_ptr })
    }

    pub fn start_watching(&self, path: &Path, watch_subdirectories: bool) -> Result<(), AvError> {
        let path_wstr = path.as_os_str()
                             .encode_wide()
                             .chain(std::iter::once(0)) // Null terminator
                             .collect::<Vec<u16>>();
        
        let success = unsafe {
            start_watching_path(self.watcher_ptr, path_wstr.as_ptr(), watch_subdirectories)
        };

        if success {
            info!("Successfully started watching path: {:?}", path);
            Ok(())
        } else {
            error!("Failed to start watching path: {:?}", path);
            Err(AvError::Other(format!("Failed to start watching path: {:?}", path)))
        }
    }

    pub fn stop_watching(&self) {
        unsafe {
            stop_watching_path(self.watcher_ptr);
        }
        info!("Stopped watching.");
    }
}

impl Drop for FileMonitor {
    fn drop(&mut self) {
        self.stop_watching();
        unsafe {
            destroy_windows_file_watcher(self.watcher_ptr);
            // Clear the global scanner reference on drop
            let _guard = GLOBAL_CALLBACK_LOCK.lock().unwrap();
            GLOBAL_SCANNER = None;
        }
    }
}