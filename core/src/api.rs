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
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tracing::{info, warn};

// Hem ana motoru hem de log yayıncısını paylaşmak için birleşik bir state tipi.
// Ancak Axum'un son versiyonları, state'in tuple olarak verilmesini daha kolay yönetir.
pub type AppState = Arc<Mutex<Engine>>;
pub type LogTx = broadcast::Sender<String>;

// Web koruma durumu için API yanıt yapısı
#[derive(Serialize)]
struct WebProtectionStatus {
    is_active: bool,
}

// Manuel tarama için API istek yapısı
#[derive(Deserialize)]
pub struct ScanRequest {
    path: String,
}

/// Tüm API endpoint'lerini tanımlayan ve ana router'ı oluşturan fonksiyon.
pub fn create_router(app_state: AppState, log_tx: LogTx) -> Router {
    // Web Güvenliği ile ilgili rotaları ayrı bir router'da gruplayalım
    let protection_router = Router::new()
        .route("/web", get(get_web_protection_status))
        .route("/web/enable", post(enable_web_protection_handler))
        .route("/web/disable", post(disable_web_protection_handler));

    // Ana router
    Router::new()
        .route("/api/status", get(get_engine_status_handler)) // Genel durum için bunu koruyalım
        .route("/api/engine-status", get(get_engine_status_handler))
        .route("/api/scan", post(scan_handler))
        .route("/api/scan/quick", post(quick_scan_handler))
        .route("/api/quarantine", get(get_quarantine_handler))
        .route(
            "/api/quarantine/{filename}",
            delete(delete_from_quarantine_handler),
        )
        .route(
            "/api/quarantine/{filename}/restore",
            post(restore_from_quarantine_handler),
        )
        .route(
            "/api/settings",
            get(get_settings_handler).post(update_settings_handler),
        )
        .route("/api/scan/progress", get(get_scan_progress_handler))
        .route("/api/ws/logs", get(log_websocket_handler))
        // Gruplanmış 'protection' rotalarını ana API'ye ekleyelim
        .nest("/api/protection", protection_router)
        // State'i tüm rotalara dağıt
        .with_state((app_state, log_tx))
}

// --- WebSocket Handler'ları ---
async fn log_websocket_handler(
    ws: WebSocketUpgrade,
    State((_engine_state, log_tx)): State<(AppState, LogTx)>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_log_socket(socket, log_tx))
}

async fn handle_log_socket(mut socket: WebSocket, log_tx: LogTx) {
    let mut log_rx = log_tx.subscribe();
    loop {
        tokio::select! {
            Ok(msg) = log_rx.recv() => {
                if socket.send(Message::Text(msg.into())).await.is_err() { break; }
            }
            Some(Ok(_)) = socket.next() => {},
            else => { break; }
        }
    }
}


// --- Diğer API Handler'ları ---

async fn get_engine_status_handler(
    State((engine, _)): State<(AppState, LogTx)>,
) -> Result<Json<EngineStatus>, StatusCode> {
    let engine_guard = engine.lock().await;
    match engine_guard.get_current_status().await {
        Ok(status) => Ok(Json(status)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_scan_progress_handler(
    State((engine, _)): State<(AppState, LogTx)>,
) -> Json<ScanProgress> {
    let engine_guard = engine.lock().await;
    let progress_guard = engine_guard.scan_progress.lock().await;
    Json(progress_guard.clone())
}

async fn scan_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Json(payload): Json<ScanRequest>,
) -> (StatusCode, String) {
    let path_buf = std::path::PathBuf::from(&payload.path);
    if !path_buf.exists() {
        return (StatusCode::BAD_REQUEST, "Yol bulunamadı.".to_string());
    }
    info!("Manual scan request received for: {}", path_buf.display());
    let engine_clone = Arc::clone(&engine);
    tokio::spawn(async move {
        let engine_guard = engine_clone.lock().await;
        if path_buf.is_dir() {
            if let Err(e) = engine_guard.run_path_scan(&payload.path).await {
                warn!("Manual directory scan failed via API: {}", e);
            }
        } else if path_buf.is_file() {
            engine_guard.scan_and_handle(&path_buf).await;
        }
    });
    (StatusCode::ACCEPTED, "Tarama isteği alındı.".to_string())
}

async fn quick_scan_handler(State((engine, _)): State<(AppState, LogTx)>) -> StatusCode {
    info!("Quick scan request received via API.");
    let engine_clone = Arc::clone(&engine);
    tokio::spawn(async move {
        let engine_guard = engine_clone.lock().await;
        if let Err(e) = engine_guard.run_quick_scan().await {
            warn!("Quick scan failed: {}", e);
        }
    });
    StatusCode::ACCEPTED
}

async fn get_quarantine_handler(
    State((engine, _)): State<(AppState, LogTx)>,
) -> Result<Json<QuarantineStats>, StatusCode> {
    let engine_guard = engine.lock().await;
    match engine_guard.get_quarantine_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_from_quarantine_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Path(filename): Path<String>,
) -> StatusCode {
    let engine_guard = engine.lock().await;
    match engine_guard.delete_quarantined_file(&filename).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

async fn restore_from_quarantine_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Path(filename): Path<String>,
) -> StatusCode {
    let engine_guard = engine.lock().await;
    match engine_guard.restore_quarantined_file(&filename).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

async fn get_settings_handler(State((engine, _)): State<(AppState, LogTx)>) -> Json<Settings> {
    let engine_guard = engine.lock().await;
    Json(engine_guard.settings.clone())
}

async fn update_settings_handler(
    State((engine, _)): State<(AppState, LogTx)>,
    Json(new_settings): Json<Settings>,
) -> StatusCode {
    let mut engine_guard = engine.lock().await;
    if engine_guard.update_and_save_settings(new_settings).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn get_web_protection_status(State((engine, _)): State<(AppState, LogTx)>) -> Json<WebProtectionStatus> {
    let engine_guard = engine.lock().await;
    let is_active = engine_guard.web_security.is_protection_active();
    Json(WebProtectionStatus { is_active })
}

async fn enable_web_protection_handler(State((engine, _)): State<(AppState, LogTx)>) -> StatusCode {
    let engine_guard = engine.lock().await;
    // Artık bu çağrı asenkron
    match engine_guard.enable_web_protection().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn disable_web_protection_handler(State((engine, _)): State<(AppState, LogTx)>) -> StatusCode {
    let engine_guard = engine.lock().await;
    match engine_guard.web_security.remove_blocks() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}