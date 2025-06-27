use anyhow::Result;
use aiav_core::{
    api,
    config::settings::Settings,
    engine::Engine,
    detection::watcher::{FileWatcher, FileEventType},
    utils::logging, // logging modülünü use ile ekliyoruz
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{info, warn, debug};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // 1. Loglama sistemini başlat ve WebSocket için göndericiyi al
    let log_tx = logging::init();

    // 2. Ayarları yükle
    let settings = Settings::load_default()?;
    info!(?settings, "AI-AV Core starting");

    // 3. Ana motoru oluştur ve güvenli paylaşım için sarmala
    let engine = Arc::new(Mutex::new(Engine::new(settings.clone()).await?));

    // --- Paralel Görevleri Başlatma ---

    // Görev 1: Gerçek Zamanlı Koruma (ayrı bir sistem thread'inde çalışır)
    let rt_engine_clone = Arc::clone(&engine);
    let watch_path = settings.watch_dir.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let watcher_receiver = FileWatcher::start(&watch_path);
            info!(path = %watch_path, "Real-time file watcher started in a background thread.");
            for (path, event_type) in watcher_receiver {
                if matches!(event_type, FileEventType::Created | FileEventType::Modified) {
                    debug!(?path, "File event received, queuing for scan.");
                    let engine_task_clone = Arc::clone(&rt_engine_clone);
                    tokio::spawn(async move {
                        let engine_guard = engine_task_clone.lock().await;
                        engine_guard.scan_and_handle(&path).await;
                    });
                }
            }
        });
    });

    // Görev 2: Periyodik Derin Tarama (arka plan Tokio task'i)
    let periodic_scan_engine_clone = Arc::clone(&engine);
    tokio::spawn(async move {
        // İlk taramadan önce bir süre bekle
        tokio::time::sleep(Duration::from_secs(10)).await;
        loop {
            {
                let engine_guard = periodic_scan_engine_clone.lock().await;
                let scan_path = engine_guard.settings.watch_dir.clone();
                if let Err(e) = engine_guard.run_path_scan(&scan_path).await {
                    warn!("Periodic scan task encountered an error: {}", e);
                }
            }
            tokio::time::sleep(Duration::from_secs(3600)).await;
        }
    });

    // Görev 3: Başlangıç Taraması (arka plan Tokio task'i)
    let initial_scan_engine_clone = Arc::clone(&engine);
    tokio::spawn(async move {
        let engine_guard = initial_scan_engine_clone.lock().await;
        let scan_path = engine_guard.settings.watch_dir.clone();
        if let Err(e) = engine_guard.run_path_scan(&scan_path).await {
            warn!(error = %e, "Initial startup scan failed to complete.");
        }
    });

    // --- Ana Görev: API Sunucusunu Çalıştırma ---
    info!("Starting API server on http://127.0.0.1:3000");

    let api_engine_clone = Arc::clone(&engine);
    // Router'a artık log göndericisini de veriyoruz.
    let app = api::create_router(api_engine_clone, log_tx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    
    warn!("Engine shut down gracefully");
    Ok(())
}