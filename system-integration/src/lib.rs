#[cfg(target_os = "windows")]
pub mod windows;           // <‐ artık bulacak

#[cfg(target_os = "linux")]
pub mod linux;             // şimdilik boş, derlenmez

#[cfg(target_os = "windows")]
pub use windows::*;        // FileWatcher, FsEvent

#[cfg(target_os = "linux")]
pub use linux::*;
