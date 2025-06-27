use once_cell::sync::Lazy;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use tracing::info;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum FileEventType {
    Created,
    Modified,
    Deleted,
}

#[link(name = "system_integration")]
extern "C" {
    fn start_file_watcher(path: *const c_char, callback: extern "C" fn(*const c_char, FileEventType));
    fn stop_file_watcher();
}

// `static mut` yerine, thread-safe olan `Lazy` ve `Mutex` kullanıyoruz.
static SENDER: Lazy<Mutex<Option<Sender<(PathBuf, FileEventType)>>>> = Lazy::new(|| Mutex::new(None));

// Bu C-stili callback, C++ tarafından çağrıldığında SENDER'ı güvenle kilitler.
#[no_mangle]
extern "C" fn rust_callback(file_path_c: *const c_char, event_type: FileEventType) {
    if let Ok(guard) = SENDER.lock() {
        if let Some(sender) = &*guard {
            let file_path = unsafe { CStr::from_ptr(file_path_c).to_string_lossy().into_owned() };
            let event = (PathBuf::from(file_path), event_type);
            if sender.send(event).is_err() {
                // Loglama yerine eprintln! kullanmak daha basit, çünkü bu alçak seviye bir callback.
                eprintln!("[watcher_callback] Failed to send file event through channel.");
            }
        }
    }
}

pub struct FileWatcher;

impl FileWatcher {
    pub fn start(path_to_watch: &str) -> Receiver<(PathBuf, FileEventType)> {
        let (tx, rx) = channel();
        
        if let Ok(mut guard) = SENDER.lock() {
            *guard = Some(tx);
        }

        let path_c = CString::new(path_to_watch).expect("CString::new failed");
        
        unsafe {
            start_file_watcher(path_c.as_ptr(), rust_callback);
        }

        info!("File watcher started for path: {}", path_to_watch);
        rx
    }

    pub fn stop() {
        unsafe {
            stop_file_watcher();
        }
        if let Ok(mut guard) = SENDER.lock() {
            *guard = None;
        }
        info!("File watcher stopped.");
    }
}