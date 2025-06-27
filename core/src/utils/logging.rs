use tokio::sync::broadcast;
use tracing::Subscriber;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

// WebSocket'e log göndermek için kullanılacak LogTx tipi
pub type LogTx = broadcast::Sender<String>;

/// Uygulama için loglama altyapısını başlatır ve WebSocket için bir gönderici döndürür.
pub fn init() -> LogTx {
    // 1. Dosyaya loglama için hedef (appender) yapılandırması
    let file_appender = tracing_appender::rolling::daily("logs", "aiav_core.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    // Dosya loglarını JSON formatında yazacak olan katman.
    let file_layer = fmt::layer()
        .json() // Dosya loglarını JSON formatında yazacak olan katman.
        .with_ansi(false) // Dosyaya ANSI renk kodları yazılmaz.
        .with_writer(non_blocking_appender);

    // 2. Konsola loglama için katman yapılandırması
    let console_layer = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_level(true);

    // 3. WebSocket'e loglama için katman yapılandırması
    // 32 mesajlık bir kapasite ile bir broadcast kanalı oluşturuyoruz.
    let (log_tx, _) = broadcast::channel(32);
    let websocket_layer = WebsocketLayer::new(log_tx.clone());
    
    // Log seviyelerini RUST_LOG çevre değişkeninden veya varsayılan olarak "info" seviyesinden alır.
    // Örnek: `RUST_LOG=debug cargo run` komutu debug loglarını da gösterir.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,hyper=warn"));

    // Yukarıda tanımlanan katmanları birleştirerek son subscriber'ı oluştur.
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .with(websocket_layer) // WebSocket katmanını ekliyoruz
        .init();
    
    // non-blocking guard'ın fonksiyon sonlandığında drop olmasını engeller.
    std::mem::forget(guard);

    // Oluşturulan göndericiyi main fonksiyonunun kullanabilmesi için döndür.
    log_tx
}

// --- WebSocket için Özel Tracing Katmanı ---

struct WebsocketLayer {
    tx: LogTx,
}

impl WebsocketLayer {
    fn new(tx: LogTx) -> Self {
        Self { tx }
    }
}

impl<S> Layer<S> for WebsocketLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut buffer = String::new();
        let mut writer = fmt::format::Writer::new(&mut buffer);

        // Logu basit bir metin formatında oluşturuyoruz.
        // Zaman damgası + seviye + hedef + mesaj
        let _ = write!(writer, "{} ", chrono::Local::now().format("%H:%M:%S%.3f"));
        let _ = write!(writer, "[{}] ", event.metadata().level());
        
        // Log mesajını almak için özel bir "visitor" kullanıyoruz.
        let mut visitor = LogMessageVisitor::new();
        event.record(&mut visitor);

        let _ = writeln!(writer, "{}", visitor.message);
        
        // Mesajı kanala gönder (eğer kanal dolu değilse).
        // Gönderim başarısız olursa (hiç dinleyen yoksa) hata basma.
        let _ = self.tx.send(buffer);
    }
}

// Event'in 'message' alanını yakalamak için bir visitor.
struct LogMessageVisitor {
    message: String,
}

impl LogMessageVisitor {
    fn new() -> Self {
        Self { message: String::new() }
    }
}

impl tracing::field::Visit for LogMessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}