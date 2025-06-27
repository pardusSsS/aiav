//! system-integration/src/windows/file_watcher.rs
//! Windows‐özel gerçek-zamanlı dosya izleyici (ReadDirectoryChangesW temelli)

use anyhow::Result;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc; // Updated to use std::sync::mpsc
use tracing::{error, info};

/// Yüksek seviyeli olay kümesi
#[derive(Debug, Clone)]
pub enum FsEvent {
    Create(PathBuf),
    Modify(PathBuf),
    Remove(PathBuf),
    Rename(PathBuf, PathBuf),
}

#[derive(Debug)]
pub struct FileWatcher {
    _inner: RecommendedWatcher, // yaşam süresini tutalım
}

impl FileWatcher {
    /// Verilen dizini (recursive) izler ve (watcher, receiver) döner.
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<(Self, mpsc::Receiver<FsEvent>)> {
        let (tx, rx) = mpsc::channel(); // Removed buffer size for std::sync::mpsc

        // notify 6.x: önerilen watcher, kendi thread’inde çalışır
        let mut inner = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(ev) => {
                    let kind = ev.kind.clone();
                    match kind {
                        EventKind::Create(_) => {
                            for p in ev.paths {
                                let _ = tx.send(FsEvent::Create(p)); // Updated to std::sync::mpsc send
                            }
                        }
                        EventKind::Modify(_) => {
                            for p in ev.paths {
                                let _ = tx.send(FsEvent::Modify(p)); // Updated to std::sync::mpsc send
                            }
                        }
                        EventKind::Remove(_) => {
                            for p in ev.paths {
                                let _ = tx.send(FsEvent::Remove(p)); // Updated to std::sync::mpsc send
                            }
                        }
                        _ => {} // Removed Rename handling as it doesn't exist
                    }
                }
                Err(e) => error!(?e, "file-watcher error"),
            }
        })?;

        // İsteğe bağlı: düşük gürültü için Config
        inner.configure(Config::default())?; // Updated to use Config::default()
        inner.watch(&path.into(), RecursiveMode::Recursive)?;
        info!("FileWatcher (Windows) started");

        Ok((Self { _inner: inner }, rx))
    }
}