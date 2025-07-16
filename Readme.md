

## Dosya Açıklamaları

### Core Engine (Rust)
- **scanner.rs**: Ana tarama motoru, dosya ve bellek taraması
- **quarantine.rs**: Şüpheli dosyaları izole etme sistemi
- **file_monitor.rs**: Gerçek zamanlı dosya sistemi izleme
- **signature_detector.rs**: İmza tabanlı tehdit tespiti
- **ai_interface.rs**: AI motoruyla iletişim arayüzü

### System Integration (C++)
- **file_watcher.cpp**: Platform-specific dosya izleme
- **process_watcher.cpp**: Süreç aktivitesi izleme
- **driver_interface.cpp**: Kernel seviyesi sürücü iletişimi
- Kernel sürücüleri: Düşük seviye sistem hooks

### AI Engine (Python)
- **malware_classifier.py**: Makine öğrenmesi tabanlı malware sınıflandırıcı
- **behavior_analyzer.py**: Davranış analizi modeli
- **anomaly_detector.py**: Anomali tespit algoritmaları
- **onnx_converter.py**: Modelleri ONNX formatına dönüştürme

### Frontend (Qt C++)
- **mainwindow.cpp**: Ana uygulama penceresi
- **dashboard_widget.cpp**: Sistem durumu dashboard'u
- **scan_widget.cpp**: Tarama arayüzü
- **system_tray.cpp**: Sistem tepsisi entegrasyonu

### Cloud Services (Rust)
- **threat_intelligence.rs**: Bulut tabanlı tehdit bilgileri
- **signature_updates.rs**: İmza güncellemeleri
- **reputation_service.rs**: Dosya/URL itibar servisi

Bu mimari, modüler, ölçeklenebilir ve maintainable bir yapı sunuyor. Her bileşen bağımsız olarak geliştirilebilir ve test edilebilir.


# AI-Entegreli Antivirüs Koruma Programı - Kapsamlı Geliştirme Dökümanı

## 📋 Proje Özeti

**Proje Adı**: AI-Powered Antivirus Protection System  
**Sürüm**: v1.0.0  
**Geliştirme Başlangıç**: 2025  
**Tahmini Süre**: 6 ay  
**Hedef Platformlar**: Windows, Linux  

### Proje Hedefleri
- Geleneksel imza tabanlı korumanın ötesinde AI destekli tehdit tespiti
- Gerçek zamanlı koruma ve davranış analizi
- Düşük sistem kaynak kullanımı ile yüksek performans
- Modern, kullanıcı dostu arayüz
- Cloud tabanlı tehdit bilgileri entegrasyonu

## 🛠 Teknoloji Stack'i

### Core Technologies
| Bileşen | Teknoloji | Sürüm | Kullanım Amacı |
|---------|-----------|-------|----------------|
| **Core Engine** | Rust | 1.75+ | Ana tarama motoru, güvenlik kritik operasyonlar |
| **System Integration** | C++ | C++20 | Sistem API'leri, kernel seviye operasyonlar |
| **AI/ML Engine** | Python | 3.10+ | Makine öğrenmesi modelleri, data science |
| **Frontend** | Qt 6 C++ | 6.5+ | Native desktop GUI, cross-platform UI |
| **Cloud Services** | Rust + Actix | - | RESTful API, tehdit bilgileri servisi |

### Supporting Technologies
- **Database**: PostgreSQL (cloud), SQLite (local)
- **Caching**: Redis
- **ML Frameworks**: PyTorch, TensorFlow
- **Model Deployment**: ONNX Runtime
- **Build System**: CMake, Cargo
- **Testing**: pytest, Google Test, Qt Test
- **CI/CD**: GitHub Actions
- **Containerization**: Docker, Kubernetes

## 🏗 Sistem Mimarisi

### High-Level Architecture
┌─────────────────────────────────────────────────────────┐
│                    Frontend Layer                        │
│                    (Qt C++ GUI)                         │
├─────────────────────────────────────────────────────────┤
│                   API Gateway                           │
│                   (Rust Actix)                          │
├─────────────────┬─────────────────┬─────────────────────┤
│   Core Engine   │   AI Engine     │   Cloud Services    │
│   (Rust)        │   (Python)      │   (Rust)           │
├─────────────────┴─────────────────┴─────────────────────┤
│              System Integration Layer                    │
│              (C++ Platform APIs)                        │
├─────────────────────────────────────────────────────────┤
│                Kernel Drivers                           │
│            (Windows/Linux Drivers)                      │
├─────────────────────────────────────────────────────────┤
│                Operating System                         │
│              (Windows/Linux APIs)                       │
└─────────────────────────────────────────────────────────┘


### Component Interaction Flow
1. **File System Events** → System Integration → Core Engine
2. **Core Engine** → AI Engine (malware analysis)
3. **AI Engine** → Core Engine (threat classification)
4. **Core Engine** → Frontend (user notifications)
5. **Cloud Services** → Core Engine (signature updates)

## 📁 Detaylı Proje Yapısı

### Root Level Structure
ai-antivirus/
├── 📄 README.md                    # Proje açıklaması ve kurulum
├── 📄 LICENSE                      # MIT/Apache lisans
├── 📄 CMakeLists.txt               # C++ build configuration
├── 📄 Cargo.toml                   # Rust workspace configuration
├── 📄 .gitignore                   # Git ignore rules
├── 📄 docker-compose.yml           # Development environment
└── 📁 [core directories...]


### Core Engine (Rust) - /core/
**Amaç**: Ana antivirüs motoru, güvenlik kritik operasyonlar

rust
// Temel yapı örneği
pub struct ScanEngine {
    signature_db: SignatureDatabase,
    ai_client: AIClient,
    quarantine: QuarantineManager,
}

impl ScanEngine {
    pub async fn scan_file(&self, path: &Path) -> ScanResult {
        // 1. Signature kontrolü
        // 2. AI analizi
        // 3. Heuristik analiz
        // 4. Sonuç döndürme
    }
}


**Key Files**:
- src/main.rs - Entry point ve service management
- src/engine/scanner.rs - Ana tarama motoru
- src/detection/signature_detector.rs - İmza tabanlı tespit
- src/detection/ai_interface.rs - AI servisi entegrasyonu
- src/api/rest_server.rs - HTTP API sunucusu

### System Integration (C++) - /system-integration/
**Amaç**: Platform-specific sistem operasyonları

cpp
// Örnek file watcher interface
class FileWatcher {
public:
    virtual void StartWatching(const std::string& path) = 0;
    virtual void SetCallback(FileEventCallback callback) = 0;
    
protected:
    FileEventCallback m_callback;
};

// Platform implementations
class WindowsFileWatcher : public FileWatcher { /* ... */ };
class LinuxFileWatcher : public FileWatcher { /* ... */ };


**Key Components**:
- File system monitoring (real-time)
- Process activity tracking
- Network traffic interception
- Registry/system configuration monitoring
- Kernel driver communication

### AI Engine (Python) - /ai-engine/
**Amaç**: Machine learning tabanlı tehdit analizi

python
# Örnek model interface
class MalwareClassifier:
    def __init__(self, model_path: str):
        self.model = self.load_model(model_path)
        
    def predict(self, file_features: Dict) -> ThreatPrediction:
        # Feature extraction
        # Model inference
        # Result formatting
        pass
        
    def extract_features(self, file_path: str) -> Dict:
        # PE header analysis
        # Entropy calculation
        # API call patterns
        # String analysis
        pass


**ML Models**:
1. **Malware Classifier**: Binary classification (malware/benign)
2. **Family Classifier**: Malware family identification
3. **Behavior Analyzer**: Runtime behavior analysis
4. **Anomaly Detector**: Zero-day threat detection

### Frontend (Qt C++) - /frontend/
**Amaç**: User interface ve system tray integration

**Main Windows**:
- **Dashboard**: System status, real-time protection status
- **Scanner**: Manual scan interface, scan history
- **Quarantine**: Isolated files management
- **Settings**: Configuration, preferences
- **Logs**: Activity logs, threat history

cpp
// Main window örneği
class MainWindow : public QMainWindow {
    Q_OBJECT
    
public:
    explicit MainWindow(QWidget *parent = nullptr);
    
private slots:
    void onScanRequested();
    void onThreatDetected(const ThreatInfo& threat);
    
private:
    DashboardWidget* m_dashboard;
    ScanWidget* m_scanner;
    SystemTray* m_systemTray;
    APIClient* m_apiClient;
};


## 🔧 Development Workflow

### Phase 1: Foundation (2 weeks)
**Objectives**: 
- Project setup and basic infrastructure
- Core Rust engine skeleton
- Basic file monitoring

**Deliverables**:
- [x] Project structure creation
- [ ] Rust core engine basic setup
- [ ] C++ system integration skeleton
- [ ] Basic CMake/Cargo build system
- [ ] CI/CD pipeline setup

**Tasks**:
bash
# 1. Repository setup
git init ai-antivirus
cd ai-antivirus

# 2. Rust workspace
cargo init core --lib
cargo init cloud-services --lib

# 3. C++ setup
mkdir system-integration
cd system-integration && cmake init .

# 4. Python environment
python -m venv ai-engine/venv
pip install torch tensorflow onnx


### Phase 2: Core Detection (4 weeks)
**Objectives**:
- Signature-based detection
- File hash verification
- Basic quarantine system

**Deliverables**:
- [ ] SHA256/MD5 hash database
- [ ] YARA rule integration
- [ ] File quarantine mechanism
- [ ] Basic REST API

### Phase 3: AI Integration (4 weeks) 
**Objectives**:
- ML model development
- Feature extraction
- ONNX model deployment

**Deliverables**:
- [ ] Malware classification model
- [ ] Feature extraction pipeline
- [ ] ONNX runtime integration
- [ ] Model performance benchmarks

### Phase 4: System Integration (4 weeks)
**Objectives**:
- Real-time file monitoring
- Process behavior analysis
- Network activity monitoring

**Deliverables**:
- [ ] File system watcher (Windows/Linux)
- [ ] Process monitoring hooks
- [ ] Network packet inspection
- [ ] Kernel driver development

### Phase 5: Frontend Development (3 weeks)
**Objectives**:
- Qt GUI implementation
- System tray integration
- User experience optimization

**Deliverables**:
- [ ] Main application window
- [ ] Real-time notifications
- [ ] Scan progress indicators
- [ ] Settings configuration

### Phase 6: Cloud Services (2 weeks)
**Objectives**:
- Threat intelligence API
- Signature updates
- Telemetry collection

**Deliverables**:
- [ ] RESTful API server
- [ ] Database schema
- [ ] Automated signature updates
- [ ] Anonymous telemetry

### Phase 7: Testing & Optimization (2 weeks)
**Objectives**:
- Performance optimization
- Security testing
- User acceptance testing

**Deliverables**:
- [ ] Unit test coverage >80%
- [ ] Integration test suite
- [ ] Performance benchmarks
- [ ] Security audit report

## 🧪 Testing Strategy

### Unit Testing
- **Rust**: cargo test with coverage reports
- **C++**: Google Test framework
- **Python**: pytest with fixtures
- **Qt**: Qt Test framework

### Integration Testing
- Cross-component communication tests
- API endpoint testing
- Database integration tests
- File system operation tests

### Performance Testing
- Scan speed benchmarks
- Memory usage profiling
- CPU utilization monitoring
- Startup time measurements

### Security Testing
- Static code analysis (clippy, cppcheck)
- Dependency vulnerability scanning
- Penetration testing
- Buffer overflow testing

## 📈 Performance Requirements

### System Requirements
- **RAM**: Minimum 2GB, Recommended 4GB
- **CPU**: Dual-core 2.0GHz minimum
- **Storage**: 500MB installation space
- **Network**: Internet connection for updates

### Performance Benchmarks
- **File Scan Speed**: >10MB/s per core
- **Memory Usage**: <200MB idle, <500MB scanning
- **CPU Usage**: <5% idle, <50% scanning
- **Startup Time**: <3 seconds
- **Update Frequency**: Every 4 hours

## 🔐 Security Considerations

### Code Security
- Memory-safe Rust for core components
- RAII patterns in C++ code
- Input validation at all boundaries
- Secure coding practices

### Runtime Security
- Process privilege separation
- Encrypted communication channels
- Secure key management
- Anti-tampering mechanisms

### Data Protection
- Encrypted configuration files
- Secure log file handling
- User privacy protection
- GDPR compliance considerations

## 🚀 Deployment Strategy

### Installation Packages
- **Windows**: NSIS installer (.exe)
- **Linux**: DEB/RPM packages
- **Development**: Docker containers

### Update Mechanism
- Automatic signature updates
- Incremental software updates
- Rollback capability
- Update verification

### Distribution Channels
- Official website download
- Package repositories (apt, yum)
- Microsoft Store (future)
- GitHub releases

## 📊 Monitoring & Analytics

### Telemetry Data
- Threat detection statistics
- Performance metrics
- Error reporting
- Usage patterns (anonymous)

### Logging System
- Structured logging (JSON format)
- Log rotation and archival
- Centralized log collection
- Real-time log analysis

## 🤝 Contributing Guidelines

### Code Standards
- **Rust**: rustfmt + clippy compliance
- **C++**: Google C++ Style Guide
- **Python**: PEP 8 + Black formatter
- **Documentation**: Inline comments + external docs

### Git Workflow
- Feature branch development
- Pull request reviews
- Automated testing on PR
- Semantic versioning

### Issue Tracking
- Bug reports with reproduction steps
- Feature requests with use cases
- Security issues (private reporting)
- Performance improvement suggestions

## 📝 Documentation Plan

### Developer Documentation
- API reference documentation
- Architecture decision records
- Setup and build instructions
- Troubleshooting guides

### User Documentation
- Installation guide
- User manual
- FAQ section
- Video tutorials

### Technical Documentation
- Protocol specifications
- Database schemas
- Configuration file formats
- Plugin development guide

## 🎯 Success Metrics

### Technical Metrics
- Detection accuracy >95%
- False positive rate <1%
- System performance impact <10%
- Stability (MTBF >30 days)

### User Metrics
- User satisfaction score >4.0/5.0
- Support ticket volume <5% of users
- Feature adoption rate >60%
- User retention rate >80%

## 🔄 Maintenance Plan

### Regular Maintenance
- Weekly signature updates
- Monthly security patches
- Quarterly feature releases
- Annual major version updates

### Long-term Support
- Bug fix support: 2 years
- Security updates: 3 years
- Migration assistance
- Legacy system compatibility

---

## 🏁 Getting Started

### Prerequisites
bash
# Rust installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# C++ development tools
# Windows: Visual Studio 2022
# Linux: build-essential, cmake

# Python environment
python3 -m pip install --user pipenv


### Quick Start
bash
# Clone repository
git clone https://github.com/your-org/ai-antivirus.git
cd ai-antivirus

# Build core engine
cd core && cargo build --release

# Build system integration
cd ../system-integration && cmake . && make

# Setup AI environment
cd ../ai-engine && pip install -r requirements.txt

# Run tests
cargo test && make test && pytest


### Development Environment
bash
# Start development services
docker-compose up -d

# Run in development mode
cargo run --bin core-engine


---

**Document Version**: 1.0  
**Last Updated**: 2025-06-17  
**Next Review**: 2025-07-17


✅ AI-Antivirus Projesi Durum Tablosu
| Aşama                                                           | Durum   | Açıklama                                                                                             |
| --------------------------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------- |
| 📁 Proje klasör yapısının oluşturulması                         | ✅ Tamam | `ai-engine`, `core`, `system-integration`, `config` gibi klasörler doğru yapılandırıldı.             |
| 🧠 AI modeli üretimi (`malware_classifier.onnx`)                | ✅ Tamam | `dummy.py` ile ONNX formatında basit bir model oluşturuldu (IR=10, opset 22).                        |
| 🚀 AI sunucusu (FastAPI) servisi                                | ✅ Tamam | `python -m ai_engine.main` komutu ile başarıyla çalışıyor ve `/v1/predict` endpoint'i yanıt veriyor. |
| 🔄 Rust tarafında AI ile bağlantı kurulması (`AiClient`)        | ✅ Tamam | `reqwest` üzerinden `POST /v1/predict` başarılı, response alınıyor.                                  |
| 🧾 SHA256 signature veritabanı kontrolü (`virus-signatures.db`) | ✅ Tamam | Dosya oluşturuldu, hash içeriği eklendi ve Rust tarafı başarıyla okuyor.                             |
| 🧪 Dosya analizi (boyut + hash + AI)                            | ✅ Tamam | Taranan dosyanın hash’i kontrol ediliyor ve AI ile analiz ediliyor.                                  |
| 🦠 Tehdit tespiti ve karantina                                  | ✅ Tamam | Tehlikeli dosya tespit edildiğinde `C:/AIAV/quarantine/` içine taşınıyor ve loglanıyor.              |
| 🔍 Loglama ve hata ayıklama çıktıları                           | ✅ Tamam | CLI üzerinden detaylı log çıktıları alınabiliyor (AI response, karantina path vs).                   |


🧩 Eksik / Geliştirme Adayı Aşamalar
| Aşama                                                 | Durum    | Önerilen Eylem                                                                                             |
| ----------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------- |
| 📊 Web dashboard veya GUI                             | ⏳ Eksik  | Web arayüz veya Tauri/GTK ile masaüstü uygulaması geliştirilebilir.                                        |
| 🔧 `config/default.toml`'dan tüm ayarların yüklenmesi | ⏳ Kısmen | Şu anda `Settings` struct'ı kullanılıyor ama TOML üzerinden her parametre dinamik mi kontrol edilmeli?     |
| 📜 Log dosyasına yazma                                | ⏳ Eksik  | Şu anda terminalde gösteriliyor. `log.txt` gibi bir dosyaya yazılabilir.                                   |
| 📡 Sistem entegrasyonu (watcher + tray icon)          | ⏳ Eksik  | Dosya sistemi dinleyicisi (`notify`) aktif değil. Tray ile sistem entegrasyonu yapılabilir.                |
| 🧪 Gerçek AI modeli ve özellik çıkarımı               | ⏳ Dummy  | Şu an `predictor.py` içinde `np.zeros((1,256))` kullanılıyor. Gerçek feature extraction ile desteklenmeli. |
| 🔐 Karantina dosyalarını şifreleme veya imzalama      | ❌ Eksik  | Gelişmiş güvenlik için zip+AES veya PGP desteği eklenebilir.                                               |



|       |       file_watcher.cpp
|       |       
|       \---windows
|               file_watcher.rs
|               mod.rs
