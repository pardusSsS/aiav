//! core/src/api.rs
//! Axum tabanlı REST + WebSocket API

use crate::config::settings::Settings;
use crate::engine::{Engine, EngineStatus, ScanProgress};
use crate::engine::quarantine::QuarantineStats;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tracing::{info, warn, error};

// ---------------------------------------------------------------
// Uygulama geneli paylaşılan state tipleri
// ---------------------------------------------------------------

pub type AppState = Arc<Mutex<Engine>>;   // Engine + tüm alt modüller
pub type LogTx    = broadcast::Sender<String>;

// ---------------------------------------------------------------
// Veri yapı(lar)ı
// ---------------------------------------------------------------

#[derive(Serialize)]
struct WebProtectionStatus {
    is_active: bool,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    path: String,
}

// ---------------------------------------------------------------
// Router kurulum fonksiyonu
// ---------------------------------------------------------------

pub fn create_router(app_state: AppState, log_tx: LogTx) -> Router {
    // Web-koruma alt-router’ı
    let protection_router = Router::new()
        .route("/web",        get(get_web_protection_status))
        .route("/web/enable", post(enable_web_protection_handler))
        .route("/web/disable",post(disable_web_protection_handler));

    // Ana router
    Router::new()
        // Motor & tarama
        .route("/api/status",         get(get_engine_status_handler))
        .route("/api/engine-status",  get(get_engine_status_handler))
        .route("/api/scan",           post(scan_handler))
        .route("/api/scan/quick",     post(quick_scan_handler))
        .route("/api/scan/progress",  get(get_scan_progress_handler))
        // Karantina
        .route("/api/quarantine",                 get(get_quarantine_handler))
        .route("/api/quarantine/{filename}",      delete(delete_from_quarantine_handler))
        .route("/api/quarantine/{filename}/restore", post(restore_from_quarantine_handler))
        // Ayarlar
        .route("/api/settings", get(get_settings_handler).post(update_settings_handler))
        // Log WebSocket’i
        .route("/api/ws/logs", get(log_websocket_handler))
        // Web-koruma alt-router’ını ekle
        .nest("/api/protection", protection_router)
        // Ortak state
        .with_state((app_state, log_tx))
}

// ---------------------------------------------------------------
// WebSocket → log yayını
// ---------------------------------------------------------------

async fn log_websocket_handler(
    ws: WebSocketUpgrade,
    State((_engine, log_tx)): State<(AppState, LogTx)>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_log_socket(socket, log_tx))
}

async fn handle_log_socket(mut socket: WebSocket, log_tx: LogTx) {
    let mut log_rx = log_tx.subscribe();
    loop {
        tokio::select! {
            Ok(msg) = log_rx.recv() => {
                if socket.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            Some(Ok(_)) = socket.next() => {},          // istemci mesajlarını yoksay
            else => break,
        }
    }
}

// ---------------------------------------------------------------
// Motor durum & tarama handler’ları
// ---------------------------------------------------------------

async fn get_engine_status_handler(State((engine, _)): State<(AppState, LogTx)>) -> Result<Json<EngineStatus>, StatusCode> {
    let eng = engine.lock().await;
    eng.get_current_status().await.map(Json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_scan_progress_handler(
    State((engine, _)): State<(AppState, LogTx)>,
) -> Json<ScanProgress> {
    let eng   = engine.lock().await;
    let guard = eng.scan_progress.lock().await;
    Json(guard.clone())
}

async fn scan_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Json(payload): Json<ScanRequest>,
) -> (StatusCode, String) {
    let path_buf = std::path::PathBuf::from(&payload.path);
    if !path_buf.exists() {
        return (StatusCode::BAD_REQUEST, "Yol bulunamadı.".into());
    }
    info!("Manual scan requested: {}", path_buf.display());

    let engine_clone = Arc::clone(&engine);
    tokio::spawn(async move {
        let eng = engine_clone.lock().await;
        if path_buf.is_dir() {
            if let Err(e) = eng.run_path_scan(&payload.path).await {
                warn!("Manual dir scan failed: {e}");
            }
        } else if path_buf.is_file() {
            eng.scan_and_handle(&path_buf).await;
        }
    });

    (StatusCode::ACCEPTED, "Tarama isteği alındı.".into())
}

async fn quick_scan_handler(
    State((engine, _)): State<(AppState, LogTx)>
) -> StatusCode {
    info!("Quick scan requested.");
    let eng = Arc::clone(&engine);
    tokio::spawn(async move {
        if let Err(e) = eng.lock().await.run_quick_scan().await {
            warn!("Quick scan failed: {e}");
        }
    });
    StatusCode::ACCEPTED
}

// ---------------------------------------------------------------
// Karantina handler’ları
// ---------------------------------------------------------------

async fn get_quarantine_handler(
    State((engine, _)): State<(AppState, LogTx)>,
) -> Result<Json<QuarantineStats>, StatusCode> {
    engine.lock().await
        .get_quarantine_stats().await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn delete_from_quarantine_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Path(filename): Path<String>,
) -> StatusCode {
    match engine.lock().await.delete_quarantined_file(&filename).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

async fn restore_from_quarantine_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Path(filename): Path<String>,
) -> StatusCode {
    match engine.lock().await.restore_quarantined_file(&filename).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

// ---------------------------------------------------------------
// Ayarlar handler’ları
// ---------------------------------------------------------------

async fn get_settings_handler(
    State((engine, _)): State<(AppState, LogTx)>
) -> Json<Settings> {
    Json(engine.lock().await.settings.clone())
}

async fn update_settings_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Json(new_settings): Json<Settings>,
) -> StatusCode {
    match engine.lock().await.update_and_save_settings(new_settings).await {
        Ok(_)  => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// ---------------------------------------------------------------
// Web-protection handler’ları
// ---------------------------------------------------------------

#[axum::debug_handler]
async fn get_web_protection_status(State((engine, _)): State<(AppState, LogTx)>) -> Json<WebProtectionStatus> {
    // DÜZELTME: Artık async metodu çağırıyoruz.
    let active = engine.lock().await.is_web_protection_active().await;
    Json(WebProtectionStatus { is_active: active })
}

#[axum::debug_handler]
async fn enable_web_protection_handler(State((engine, _)): State<(AppState, LogTx)>) -> StatusCode {
    match engine.lock().await.enable_web_protection().await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("API: Failed to enable web protection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[axum::debug_handler]
async fn disable_web_protection_handler(State((engine, _)): State<(AppState, LogTx)>) -> StatusCode {
    // DÜZELTME: Artık async metodu çağırıyoruz.
    match engine.lock().await.disable_web_protection().await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!("API: Failed to disable web protection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}