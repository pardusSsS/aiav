┌─────────────────────────────────────────┐
│              Frontend (Qt/C++)           │
├─────────────────────────────────────────┤
│         API Gateway (Rust)              │
├─────────────────────────────────────────┤
│  Core Engine     │   AI Engine          │
│  (Rust/C++)      │   (Python/ONNX)     │
├─────────────────────────────────────────┤
│         System Layer (C++/Rust)         │
│  • File Monitor  • Network  • Process   │
├─────────────────────────────────────────┤
│            OS APIs (Windows/Linux)       │
└─────────────────────────────────────────┘


# AI-Entegreli Antivirüs Koruma Programı - Detaylı Mimari Şeması

## Proje Klasör Yapısı

ai-antivirus/
├── README.md
├── LICENSE
├── CMakeLists.txt
├── Cargo.toml
├── .gitignore
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── security-scan.yml
│
├── docs/
│   ├── architecture.md
│   ├── api-reference.md
│   ├── installation.md
│   └── user-guide.md
│
├── scripts/
│   ├── build.sh
│   ├── install.sh
│   ├── uninstall.sh
│   └── setup-dev.sh
│
├── config/
│   ├── default.toml
│   ├── development.toml
│   ├── production.toml
│   └── signatures/
│       ├── virus-signatures.db
│       ├── malware-patterns.json
│       └── whitelist.json
│
├── core/                           # Rust Core Engine
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   ├── settings.rs
│   │   │   └── database.rs
│   │   ├── engine/
│   │   │   ├── mod.rs
│   │   │   ├── scanner.rs
│   │   │   ├── quarantine.rs
│   │   │   ├── file_monitor.rs
│   │   │   ├── process_monitor.rs
│   │   │   └── network_monitor.rs
│   │   ├── detection/
│   │   │   ├── mod.rs
│   │   │   ├── signature_detector.rs
│   │   │   ├── heuristic_analyzer.rs
│   │   │   └── ai_interface.rs
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── rest_server.rs
│   │   │   ├── grpc_server.rs
│   │   │   └── ipc_handler.rs
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── crypto.rs
│   │   │   ├── file_utils.rs
│   │   │   └── logging.rs
│   │   └── error/
│   │       ├── mod.rs
│   │       └── types.rs
│   ├── tests/
│   │   ├── integration/
│   │   │   ├── scanner_tests.rs
│   │   │   └── api_tests.rs
│   │   └── unit/
│   │       ├── detection_tests.rs
│   │       └── engine_tests.rs
│   └── benches/
│       ├── scanner_bench.rs
│       └── detection_bench.rs
│
├── system-integration/              # C++ System Layer
│   ├── CMakeLists.txt
│   ├── include/
│   │   ├── system_api.h
│   │   ├── file_watcher.h
│   │   ├── process_watcher.h
│   │   ├── network_interceptor.h
│   │   ├── driver_interface.h
│   │   └── platform/
│   │       ├── windows/
│   │       │   ├── win_file_monitor.h
│   │       │   ├── win_process_monitor.h
│   │       │   └── win_registry.h
│   │       └── linux/
│   │           ├── linux_file_monitor.h
│   │           ├── linux_process_monitor.h
│   │           └── linux_inotify.h
│   ├── src/
│   │   ├── system_api.cpp
│   │   ├── file_watcher.cpp
│   │   ├── process_watcher.cpp
│   │   ├── network_interceptor.cpp
│   │   ├── driver_interface.cpp
│   │   └── platform/
│   │       ├── windows/
│   │       │   ├── win_file_monitor.cpp
│   │       │   ├── win_process_monitor.cpp
│   │       │   ├── win_registry.cpp
│   │       │   └── win_service.cpp
│   │       └── linux/
│   │           ├── linux_file_monitor.cpp
│   │           ├── linux_process_monitor.cpp
│   │           └── linux_inotify.cpp
│   ├── drivers/                    # Kernel Drivers
│   │   ├── windows/
│   │   │   ├── file_filter/
│   │   │   │   ├── file_filter.inf
│   │   │   │   ├── file_filter.c
│   │   │   │   └── file_filter.h
│   │   │   └── process_filter/
│   │   │       ├── process_filter.inf
│   │   │       ├── process_filter.c
│   │   │       └── process_filter.h
│   │   └── linux/
│   │       ├── file_monitor.ko
│   │       └── process_monitor.ko
│   └── tests/
│       ├── system_tests.cpp
│       ├── file_watcher_tests.cpp
│       └── mock/
│           ├── mock_filesystem.h
│           └── mock_processes.h
│
├── ai-engine/                      # Python AI/ML Engine
│   ├── requirements.txt
│   ├── setup.py
│   ├── pyproject.toml
│   ├── src/
│   │   ├── __init__.py
│   │   ├── main.py
│   │   ├── config/
│   │   │   ├── __init__.py
│   │   │   ├── settings.py
│   │   │   └── logging_config.py
│   │   ├── models/
│   │   │   ├── __init__.py
│   │   │   ├── malware_classifier.py
│   │   │   ├── behavior_analyzer.py
│   │   │   ├── anomaly_detector.py
│   │   │   └── ensemble_model.py
│   │   ├── data/
│   │   │   ├── __init__.py
│   │   │   ├── preprocessor.py
│   │   │   ├── feature_extractor.py
│   │   │   └── dataset_loader.py
│   │   ├── training/
│   │   │   ├── __init__.py
│   │   │   ├── trainer.py
│   │   │   ├── validator.py
│   │   │   └── hyperparameter_tuner.py
│   │   ├── inference/
│   │   │   ├── __init__.py
│   │   │   ├── predictor.py
│   │   │   ├── onnx_converter.py
│   │   │   └── model_server.py
│   │   ├── api/
│   │   │   ├── __init__.py
│   │   │   ├── fastapi_server.py
│   │   │   ├── grpc_server.py
│   │   │   └── schemas.py
│   │   └── utils/
│   │       ├── __init__.py
│   │       ├── file_analyzer.py
│   │       ├── crypto_utils.py
│   │       └── metrics.py
│   ├── tests/
│   │   ├── __init__.py
│   │   ├── test_models.py
│   │   ├── test_preprocessing.py
│   │   ├── test_api.py
│   │   └── fixtures/
│   │       ├── sample_malware/
│   │       └── sample_clean/
│   ├── notebooks/
│   │   ├── data_exploration.ipynb
│   │   ├── model_development.ipynb
│   │   └── performance_analysis.ipynb
│   └── models/                     # Trained Models
│       ├── malware_classifier.onnx
│       ├── behavior_analyzer.onnx
│       ├── anomaly_detector.onnx
│       └── metadata/
│           ├── model_info.json
│           └── feature_names.json
│
├── frontend/                       # Qt C++ GUI
│   ├── CMakeLists.txt
│   ├── main.cpp
│   ├── src/
│   │   ├── application.cpp
│   │   ├── application.h
│   │   ├── mainwindow/
│   │   │   ├── mainwindow.cpp
│   │   │   ├── mainwindow.h
│   │   │   └── mainwindow.ui
│   │   ├── dashboard/
│   │   │   ├── dashboard_widget.cpp
│   │   │   ├── dashboard_widget.h
│   │   │   └── dashboard_widget.ui
│   │   ├── scanner/
│   │   │   ├── scan_widget.cpp
│   │   │   ├── scan_widget.h
│   │   │   ├── scan_widget.ui
│   │   │   └── scan_progress_dialog.cpp
│   │   ├── quarantine/
│   │   │   ├── quarantine_widget.cpp
│   │   │   ├── quarantine_widget.h
│   │   │   └── quarantine_widget.ui
│   │   ├── settings/
│   │   │   ├── settings_dialog.cpp
│   │   │   ├── settings_dialog.h
│   │   │   └── settings_dialog.ui
│   │   ├── logs/
│   │   │   ├── log_viewer.cpp
│   │   │   ├── log_viewer.h
│   │   │   └── log_viewer.ui
│   │   ├── tray/
│   │   │   ├── system_tray.cpp
│   │   │   └── system_tray.h
│   │   ├── widgets/
│   │   │   ├── threat_item_widget.cpp
│   │   │   ├── threat_item_widget.h
│   │   │   ├── status_indicator.cpp
│   │   │   └── status_indicator.h
│   │   └── api/
│   │       ├── api_client.cpp
│   │       ├── api_client.h
│   │       └── models/
│   │           ├── scan_result.h
│   │           └── threat_info.h
│   ├── resources/
│   │   ├── icons/
│   │   │   ├── app_icon.ico
│   │   │   ├── scan.png
│   │   │   ├── shield.png
│   │   │   └── warning.png
│   │   ├── styles/
│   │   │   ├── dark_theme.qss
│   │   │   └── light_theme.qss
│   │   └── translations/
│   │       ├── app_tr.ts
│   │       ├── app_en.ts
│   │       └── app_de.ts
│   └── tests/
│       ├── ui_tests.cpp
│       └── widget_tests.cpp
│
├── cloud-services/                 # Rust Cloud Backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── api/
│   │   │   ├── mod.rs
│   │   │   ├── threat_intelligence.rs
│   │   │   ├── signature_updates.rs
│   │   │   └── telemetry.rs
│   │   ├── database/
│   │   │   ├── mod.rs
│   │   │   ├── models.rs
│   │   │   ├── schema.rs
│   │   │   └── migrations/
│   │   │       ├── 2024-01-01-create-threats.sql
│   │   │       └── 2024-01-02-create-signatures.sql
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── threat_analyzer.rs
│   │   │   ├── signature_generator.rs
│   │   │   └── reputation_service.rs
│   │   └── utils/
│   │       ├── mod.rs
│   │       ├── crypto.rs
│   │       └── rate_limiter.rs
│   ├── migrations/
│   ├── tests/
│   └── docker/
│       ├── Dockerfile
│       └── docker-compose.yml
│
├── shared/                         # Shared Libraries
│   ├── proto/                      # Protocol Buffers
│   │   ├── scanner.proto
│   │   ├── ai_service.proto
│   │   └── threat_intel.proto
│   ├── schemas/                    # JSON Schemas
│   │   ├── threat_report.json
│   │   ├── scan_result.json
│   │   └── config_schema.json
│   └── ffi/                        # Foreign Function Interface
│       ├── rust_to_cpp.h
│       ├── cpp_to_rust.rs
│       └── python_bindings.py
│
├── database/
│   ├── schemas/
│   │   ├── postgresql/
│   │   │   ├── init.sql
│   │   │   ├── threats.sql
│   │   │   └── signatures.sql
│   │   └── sqlite/
│   │       ├── local.sql
│   │       └── cache.sql
│   └── seed_data/
│       ├── initial_signatures.sql
│       └── test_threats.sql
│
├── installer/
│   ├── windows/
│   │   ├── installer.nsi          # NSIS installer script
│   │   ├── uninstaller.nsi
│   │   └── resources/
│   │       ├── license.txt
│   │       └── icon.ico
│   ├── linux/
│   │   ├── debian/
│   │   │   ├── control
│   │   │   ├── postinst
│   │   │   └── prerm
│   │   └── rpm/
│   │       └── aiav.spec
│   └── macos/
│       └── Info.plist
│
├── tests/
│   ├── integration/
│   │   ├── full_system_test.py
│   │   ├── performance_test.py
│   │   └── security_test.py
│   ├── e2e/
│   │   ├── scan_workflow_test.py
│   │   └── ui_automation_test.py
│   └── fixtures/
│       ├── malware_samples/
│       │   ├── eicar.com
│       │   └── test_virus.exe
│       └── clean_files/
│           ├── document.pdf
│           └── image.jpg
│
├── tools/
│   ├── signature_generator/
│   │   ├── main.py
│   │   └── hash_generator.py
│   ├── model_converter/
│   │   ├── pytorch_to_onnx.py
│   │   └── tensorflow_to_onnx.py
│   └── performance_profiler/
│       ├── memory_profiler.py
│       └── cpu_profiler.py
│
├── third-party/
│   ├── licenses/
│   │   ├── qt_license.txt
│   │   ├── pytorch_license.txt
│   │   └── rust_crates_licenses.txt
│   └── binaries/
│       ├── windows/
│       └── linux/
│
└── deployment/
    ├── docker/
    │   ├── Dockerfile.core
    │   ├── Dockerfile.ai
    │   ├── Dockerfile.frontend
    │   └── docker-compose.yml
    ├── kubernetes/
    │   ├── namespace.yaml
    │   ├── deployment.yaml
    │   ├── service.yaml
    │   └── ingress.yaml
    └── terraform/
        ├── main.tf
        ├── variables.tf
        └── outputs.tf


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
| 📊 Web dashboard veya GUI                             | ⏳ tamam  | Web arayüz veya Tauri/GTK ile masaüstü uygulaması geliştirilebilir.                                        |
| 🔧 `config/default.toml`'dan tüm ayarların yüklenmesi |⏳ Kısmen | Şu anda `Settings` struct'ı kullanılıyor ama TOML üzerinden her parametre dinamik mi kontrol edilmeli?     |
| 📜 Log dosyasına yazma                                | ✅ tamam  | Şu anda terminalde gösteriliyor. `log.txt` gibi bir dosyaya yazılabilir.                                   |
| 📡 Sistem entegrasyonu (watcher + tray icon)          | ✅ tamam  | Dosya sistemi dinleyicisi (`notify`) aktif değil. Tray ile sistem entegrasyonu yapılabilir.                |
| 🧪 Gerçek AI modeli ve özellik çıkarımı               | ✅ tamam  | Şu an `predictor.py` içinde `np.zeros((1,256))` kullanılıyor. Gerçek feature extraction ile desteklenmeli. |
| 🔐 Karantina dosyalarını şifreleme veya imzalama      | ✅ tamam  | Gelişmiş güvenlik için zip+AES veya PGP desteği eklenebilir.                                               |


Tamamlanan Ağaç yapısı
Folder PATH listing
Volume serial number is 9859-D489
C:.
│   .env
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   create_eicar.py
│   create_suspicious_file.py
│   create_yara_test_file.py
│   dummy.py
│   hash_checker.py
│   Readme.md
│   tree.txt
│
├───.vscode
│       settings.json
│
├───ai-engine
│   │   pyproject.toml
│   │   requirements.txt
│   │
│   ├───aiav_engine.egg-info
│   │       dependency_links.txt
│   │       PKG-INFO
│   │       requires.txt
│   │       SOURCES.txt
│   │       top_level.txt
│   │
│   ├───ai_engine
│   │   │   feature_extractor.py
│   │   │   main.py
│   │   │   predictor.py
│   │   │   train_model.py
│   │   │   __init__.py
│   │   │
│   │   ├───models
│   │   │       malware_classifier.onnx
│   │   │
│   │   └───__pycache__
│   │           feature_extractor.cpython-312.pyc
│   │           main.cpython-312.pyc
│   │           predictor.cpython-312.pyc
│   │           train_model.cpython-312.pyc
│   │           __init__.cpython-312.pyc
│   │
│   └───ember_data
│           metadata.csv
│           test_metadata.csv
│           train_metadata.csv
│           X_test.dat
│           X_train.dat
│           y_test.dat
│           y_train.dat
│
├───cloud-services
│   │   Cargo.toml
│   │
│   └───src
│           lib.rs
│
├───config
│   │   default.toml
│   │
│   └───signatures
│           adware_hosts.txt
│           adware_hosts.txt.meta.json
│           cache_metadata.json
│           malware_hosts.txt
│           malware_patterns.yar
│           tracker_hosts.txt
│           tracker_hosts.txt.meta.json
│           virus-signatures.db
│
├───core
│   │   build.rs
│   │   Cargo.toml
│   │
│   ├───config
│   │       development.toml
│   │
│   ├───logs
│   │       aiav_core.log.2025-06-24
│   │
│   └───src
│       │   api.rs
│       │   lib.rs
│       │   main.rs
│       │
│       ├───config
│       │       mod.rs
│       │       settings.rs
│       │
│       ├───detection
│       │       ai.rs
│       │       mod.rs
│       │       signature.rs
│       │       watcher.rs
│       │       web_security.rs
│       │
│       ├───engine
│       │       file_monitor.rs
│       │       mod.rs
│       │       mod.rs.tmp
│       │       quarantine.rs
│       │       scanner.rs
│       │
│       ├───error
│       │       mod.rs
│       │       types.rs
│       │
│       ├───tests
│       │   │   lib.rs
│       │   │
│       │   └───unit
│       │           detection_tests.rs
│       │
│       └───utils
│               crypto.rs
│               logging.rs
│               mod.rs
│
├───frontend
│   │   CMakeLists.txt
│   │
│   ├───build
│   │   │   AIAntivirus.vcxproj
│   │   │   AIAntivirus.vcxproj.filters
│   │   │   AIAntivirusFrontend.sln
│   │   │   ALL_BUILD.vcxproj
│   │   │   ALL_BUILD.vcxproj.filters
│   │   │   CMakeCache.txt
│   │   │   cmake_install.cmake
│   │   │   ZERO_CHECK.vcxproj
│   │   │   ZERO_CHECK.vcxproj.filters
│   │   │
│   │   ├───.qt
│   │   │       QtDeploySupport-Debug.cmake
│   │   │       QtDeploySupport-MinSizeRel.cmake
│   │   │       QtDeploySupport-Release.cmake
│   │   │       QtDeploySupport-RelWithDebInfo.cmake
│   │   │       QtDeployTargets-Debug.cmake
│   │   │       QtDeployTargets-MinSizeRel.cmake
│   │   │       QtDeployTargets-Release.cmake
│   │   │       QtDeployTargets-RelWithDebInfo.cmake
│   │   │
│   │   ├───AIAntivirus.dir
│   │   │   └───Release
│   │   │       │   AIAntivirus.exe.recipe
│   │   │       │   api_client.obj
│   │   │       │   dashboard_page.obj
│   │   │       │   futuristic_dropzone.obj
│   │   │       │   main.obj
│   │   │       │   mainwindow.obj
│   │   │       │   mocs_compilation_Release.obj
│   │   │       │   navigationbar.obj
│   │   │       │   protection_page.obj
│   │   │       │   quarantine_item_delegate.obj
│   │   │       │   quarantine_page.obj
│   │   │       │   scan_page.obj
│   │   │       │   settings_page.obj
│   │   │       │   shield_widget.obj
│   │   │       │   status_card_widget.obj
│   │   │       │
│   │   │       ├───AIAntivirus.tlog
│   │   │       │       AIAntivirus.lastbuildstate
│   │   │       │       CL.command.1.tlog
│   │   │       │       Cl.items.tlog
│   │   │       │       CL.read.1.tlog
│   │   │       │       CL.write.1.tlog
│   │   │       │       CustomBuild.command.1.tlog
│   │   │       │       CustomBuild.read.1.tlog
│   │   │       │       CustomBuild.write.1.tlog
│   │   │       │       link.command.1.tlog
│   │   │       │       link.read.1.tlog
│   │   │       │       link.secondary.1.tlog
│   │   │       │       link.write.1.tlog
│   │   │       │
│   │   │       └───AIAntivirus_autogen
│   │   │           └───3YJK5W5UP7_Release
│   │   │                   qrc_resources.cpp.obj
│   │   │
│   │   ├───AIAntivirus_autogen
│   │   │   │   mocs_compilation_Release.cpp
│   │   │   │
│   │   │   ├───3YJK5W5UP7_Release
│   │   │   │       qrc_resources.cpp
│   │   │   │
│   │   │   └───include_Release
│   │   │       ├───3YJK5W5UP7
│   │   │       │       qrc_resources_CMAKE_.cpp
│   │   │       │
│   │   │       ├───PA7CP7W2FZ
│   │   │       │       moc_api_client.cpp
│   │   │       │       moc_api_client.cpp.d
│   │   │       │
│   │   │       ├───RZ4CM6YYJ6
│   │   │       │       moc_futuristic_dropzone.cpp
│   │   │       │       moc_futuristic_dropzone.cpp.d
│   │   │       │       moc_navigationbar.cpp
│   │   │       │       moc_navigationbar.cpp.d
│   │   │       │       moc_shield_widget.cpp
│   │   │       │       moc_shield_widget.cpp.d
│   │   │       │       moc_status_card_widget.cpp
│   │   │       │       moc_status_card_widget.cpp.d
│   │   │       │
│   │   │       ├───UVLADIE3JM
│   │   │       │       moc_mainwindow.cpp
│   │   │       │       moc_mainwindow.cpp.d
│   │   │       │
│   │   │       ├───V5LNE4NMWJ
│   │   │       │       moc_dashboard_page.cpp
│   │   │       │       moc_dashboard_page.cpp.d
│   │   │       │       moc_protection_page.cpp
│   │   │       │       moc_protection_page.cpp.d
│   │   │       │       moc_quarantine_page.cpp
│   │   │       │       moc_quarantine_page.cpp.d
│   │   │       │       moc_scan_page.cpp
│   │   │       │       moc_scan_page.cpp.d
│   │   │       │       moc_settings_page.cpp
│   │   │       │       moc_settings_page.cpp.d
│   │   │       │
│   │   │       └───W4I2IAWOYD
│   │   │               moc_quarantine_item_delegate.cpp
│   │   │               moc_quarantine_item_delegate.cpp.d
│   │   │
│   │   ├───CMakeFiles
│   │   │   │   cmake.check_cache
│   │   │   │   CMakeConfigureLog.yaml
│   │   │   │   generate.stamp
│   │   │   │   generate.stamp.depend
│   │   │   │   generate.stamp.list
│   │   │   │   InstallScripts.json
│   │   │   │   TargetDirectories.txt
│   │   │   │
│   │   │   ├───3a6e9520c982a9e38b4acd57cef37efd
│   │   │   │       generate.stamp.rule
│   │   │   │
│   │   │   ├───4.0.3
│   │   │   │   │   CMakeCCompiler.cmake
│   │   │   │   │   CMakeCXXCompiler.cmake
│   │   │   │   │   CMakeDetermineCompilerABI_C.bin
│   │   │   │   │   CMakeDetermineCompilerABI_CXX.bin
│   │   │   │   │   CMakeRCCompiler.cmake
│   │   │   │   │   CMakeSystem.cmake
│   │   │   │   │   VCTargetsPath.txt
│   │   │   │   │   VCTargetsPath.vcxproj
│   │   │   │   │
│   │   │   │   ├───CompilerIdC
│   │   │   │   │   │   CMakeCCompilerId.c
│   │   │   │   │   │   CompilerIdC.exe
│   │   │   │   │   │   CompilerIdC.vcxproj
│   │   │   │   │   │
│   │   │   │   │   ├───Debug
│   │   │   │   │   │   │   CMakeCCompilerId.obj
│   │   │   │   │   │   │   CompilerIdC.exe.recipe
│   │   │   │   │   │   │
│   │   │   │   │   │   └───CompilerIdC.tlog
│   │   │   │   │   │           CL.command.1.tlog
│   │   │   │   │   │           Cl.items.tlog
│   │   │   │   │   │           CL.read.1.tlog
│   │   │   │   │   │           CL.write.1.tlog
│   │   │   │   │   │           CompilerIdC.lastbuildstate
│   │   │   │   │   │           link.command.1.tlog
│   │   │   │   │   │           link.read.1.tlog
│   │   │   │   │   │           link.secondary.1.tlog
│   │   │   │   │   │           link.write.1.tlog
│   │   │   │   │   │
│   │   │   │   │   └───tmp
│   │   │   │   ├───CompilerIdCXX
│   │   │   │   │   │   CMakeCXXCompilerId.cpp
│   │   │   │   │   │   CompilerIdCXX.exe
│   │   │   │   │   │   CompilerIdCXX.vcxproj
│   │   │   │   │   │
│   │   │   │   │   ├───Debug
│   │   │   │   │   │   │   CMakeCXXCompilerId.obj
│   │   │   │   │   │   │   CompilerIdCXX.exe.recipe
│   │   │   │   │   │   │
│   │   │   │   │   │   └───CompilerIdCXX.tlog
│   │   │   │   │   │           CL.command.1.tlog
│   │   │   │   │   │           Cl.items.tlog
│   │   │   │   │   │           CL.read.1.tlog
│   │   │   │   │   │           CL.write.1.tlog
│   │   │   │   │   │           CompilerIdCXX.lastbuildstate
│   │   │   │   │   │           link.command.1.tlog
│   │   │   │   │   │           link.read.1.tlog
│   │   │   │   │   │           link.secondary.1.tlog
│   │   │   │   │   │           link.write.1.tlog
│   │   │   │   │   │
│   │   │   │   │   └───tmp
│   │   │   │   ├───VCTargetsPath
│   │   │   │   │   └───x64
│   │   │   │   │       └───Debug
│   │   │   │   │           │   VCTargetsPath.recipe
│   │   │   │   │           │
│   │   │   │   │           └───VCTargetsPath.tlog
│   │   │   │   │                   VCTargetsPath.lastbuildstate
│   │   │   │   │
│   │   │   │   └───x64
│   │   │   │       └───Debug
│   │   │   ├───6445a5f323c4bb181d37a6583c302a1c
│   │   │   │       qrc_resources.cpp.rule
│   │   │   │
│   │   │   ├───AIAntivirus_autogen.dir
│   │   │   │       AutogenInfo.json
│   │   │   │       AutogenUsed_Release.txt
│   │   │   │       AutoRcc_resources_3YJK5W5UP7_Info.json
│   │   │   │       AutoRcc_resources_3YJK5W5UP7_Lock.lock
│   │   │   │       AutoRcc_resources_3YJK5W5UP7_Used_Release.txt
│   │   │   │       ParseCache_Release.txt
│   │   │   │
│   │   │   ├───CMakeScratch
│   │   │   └───pkgRedirects
│   │   ├───Release
│   │   │   │   AIAntivirus.exe
│   │   │   │   d3dcompiler_47.dll
│   │   │   │   dxcompiler.dll
│   │   │   │   dxil.dll
│   │   │   │   opengl32sw.dll
│   │   │   │   Qt6Core.dll
│   │   │   │   Qt6Gui.dll
│   │   │   │   Qt6Network.dll
│   │   │   │   Qt6Svg.dll
│   │   │   │   Qt6WebSockets.dll
│   │   │   │   Qt6Widgets.dll
│   │   │   │   vc_redist.x64.exe
│   │   │   │
│   │   │   ├───generic
│   │   │   │       qtuiotouchplugin.dll
│   │   │   │
│   │   │   ├───iconengines
│   │   │   │       qsvgicon.dll
│   │   │   │
│   │   │   ├───imageformats
│   │   │   │       qgif.dll
│   │   │   │       qico.dll
│   │   │   │       qjpeg.dll
│   │   │   │       qsvg.dll
│   │   │   │
│   │   │   ├───networkinformation
│   │   │   │       qnetworklistmanager.dll
│   │   │   │
│   │   │   ├───platforms
│   │   │   │       qwindows.dll
│   │   │   │
│   │   │   ├───styles
│   │   │   │       qmodernwindowsstyle.dll
│   │   │   │
│   │   │   ├───tls
│   │   │   │       qcertonlybackend.dll
│   │   │   │       qschannelbackend.dll
│   │   │   │
│   │   │   └───translations
│   │   │           qt_ar.qm
│   │   │           qt_bg.qm
│   │   │           qt_ca.qm
│   │   │           qt_cs.qm
│   │   │           qt_da.qm
│   │   │           qt_de.qm
│   │   │           qt_en.qm
│   │   │           qt_es.qm
│   │   │           qt_fa.qm
│   │   │           qt_fi.qm
│   │   │           qt_fr.qm
│   │   │           qt_gd.qm
│   │   │           qt_he.qm
│   │   │           qt_hr.qm
│   │   │           qt_hu.qm
│   │   │           qt_it.qm
│   │   │           qt_ja.qm
│   │   │           qt_ka.qm
│   │   │           qt_ko.qm
│   │   │           qt_lv.qm
│   │   │           qt_nl.qm
│   │   │           qt_nn.qm
│   │   │           qt_pl.qm
│   │   │           qt_pt_BR.qm
│   │   │           qt_ru.qm
│   │   │           qt_sk.qm
│   │   │           qt_tr.qm
│   │   │           qt_uk.qm
│   │   │           qt_zh_CN.qm
│   │   │           qt_zh_TW.qm
│   │   │
│   │   └───x64
│   │       └───Release
│   │           ├───ALL_BUILD
│   │           │   │   ALL_BUILD.recipe
│   │           │   │
│   │           │   └───ALL_BUILD.tlog
│   │           │           ALL_BUILD.lastbuildstate
│   │           │           CustomBuild.command.1.tlog
│   │           │           CustomBuild.read.1.tlog
│   │           │           CustomBuild.write.1.tlog
│   │           │
│   │           └───ZERO_CHECK
│   │               │   ZERO_CHECK.recipe
│   │               │
│   │               └───ZERO_CHECK.tlog
│   │                       CustomBuild.command.1.tlog
│   │                       CustomBuild.read.1.tlog
│   │                       CustomBuild.write.1.tlog
│   │                       ZERO_CHECK.lastbuildstate
│   │
│   ├───resources
│   │   │   resources.qrc
│   │   │
│   │   ├───icons
│   │   │       home.svg
│   │   │       overview.svg
│   │   │       protection.svg
│   │   │       scan.svg
│   │   │       settings.svg
│   │   │       shield_status.svg
│   │   │       wifi.svg
│   │   │
│   │   └───styles
│   │           dark_theme.qss
│   │
│   └───src
│       │   main.cpp
│       │   mainwindow.cpp
│       │   mainwindow.h
│       │
│       ├───api
│       │       api_client.cpp
│       │       api_client.h
│       │
│       ├───delegates
│       │       quarantine_item_delegate.cpp
│       │       quarantine_item_delegate.h
│       │
│       ├───pages
│       │       dashboard_page.cpp
│       │       dashboard_page.h
│       │       protection_page.cpp
│       │       protection_page.h
│       │       quarantine_page.cpp
│       │       quarantine_page.h
│       │       scan_page.cpp
│       │       scan_page.h
│       │       settings_page.cpp
│       │       settings_page.h
│       │
│       └───widgets
│               commandcore_widget.cpp
│               commandcore_widget.h
│               futuristic_dropzone.cpp
│               futuristic_dropzone.h
│               navigationbar.cpp
│               navigationbar.h
│               shield_widget.cpp
│               shield_widget.h
│               status_card_widget.cpp
│               status_card_widget.h
│
├───logs
│       aiav_core.log.2025-06-20
│       aiav_core.log.2025-06-22
│       aiav_core.log.2025-06-23
│       aiav_core.log.2025-06-24
│       aiav_core.log.2025-06-25
│       aiav_core.log.2025-06-26
│       aiav_core.log.2025-06-27
│       aiav_core.log.2025-06-28
│
├───system-integration
│   │   Cargo.toml
│   │   CMakeLists.txt
│   │
│   ├───build
│   │   │   ALL_BUILD.vcxproj
│   │   │   ALL_BUILD.vcxproj.filters
│   │   │   CMakeCache.txt
│   │   │   cmake_install.cmake
│   │   │   system_integration.sln
│   │   │   system_integration.vcxproj
│   │   │   system_integration.vcxproj.filters
│   │   │   ZERO_CHECK.vcxproj
│   │   │   ZERO_CHECK.vcxproj.filters
│   │   │
│   │   ├───CMakeFiles
│   │   │   │   cmake.check_cache
│   │   │   │   CMakeConfigureLog.yaml
│   │   │   │   generate.stamp
│   │   │   │   generate.stamp.depend
│   │   │   │   generate.stamp.list
│   │   │   │   InstallScripts.json
│   │   │   │   TargetDirectories.txt
│   │   │   │
│   │   │   ├───1c613aa7aa38557742f256635795a7e9
│   │   │   │       generate.stamp.rule
│   │   │   │
│   │   │   ├───4.0.3
│   │   │   │   │   CMakeCXXCompiler.cmake
│   │   │   │   │   CMakeDetermineCompilerABI_CXX.bin
│   │   │   │   │   CMakeRCCompiler.cmake
│   │   │   │   │   CMakeSystem.cmake
│   │   │   │   │   VCTargetsPath.txt
│   │   │   │   │   VCTargetsPath.vcxproj
│   │   │   │   │
│   │   │   │   ├───CompilerIdCXX
│   │   │   │   │   │   CMakeCXXCompilerId.cpp
│   │   │   │   │   │   CompilerIdCXX.exe
│   │   │   │   │   │   CompilerIdCXX.vcxproj
│   │   │   │   │   │
│   │   │   │   │   ├───Debug
│   │   │   │   │   │   │   CMakeCXXCompilerId.obj
│   │   │   │   │   │   │   CompilerIdCXX.exe.recipe
│   │   │   │   │   │   │
│   │   │   │   │   │   └───CompilerIdCXX.tlog
│   │   │   │   │   │           CL.command.1.tlog
│   │   │   │   │   │           Cl.items.tlog
│   │   │   │   │   │           CL.read.1.tlog
│   │   │   │   │   │           CL.write.1.tlog
│   │   │   │   │   │           CompilerIdCXX.lastbuildstate
│   │   │   │   │   │           link.command.1.tlog
│   │   │   │   │   │           link.read.1.tlog
│   │   │   │   │   │           link.secondary.1.tlog
│   │   │   │   │   │           link.write.1.tlog
│   │   │   │   │   │
│   │   │   │   │   └───tmp
│   │   │   │   ├───VCTargetsPath
│   │   │   │   │   └───x64
│   │   │   │   │       └───Debug
│   │   │   │   │           │   VCTargetsPath.recipe
│   │   │   │   │           │
│   │   │   │   │           └───VCTargetsPath.tlog
│   │   │   │   │                   VCTargetsPath.lastbuildstate
│   │   │   │   │
│   │   │   │   └───x64
│   │   │   │       └───Debug
│   │   │   ├───CMakeScratch
│   │   │   └───pkgRedirects
│   │   ├───Release
│   │   │       system_integration.dll
│   │   │       system_integration.exp
│   │   │       system_integration.lib
│   │   │
│   │   ├───system_integration.dir
│   │   │   └───Release
│   │   │       │   file_watcher.obj
│   │   │       │   system_integration.dll.recipe
│   │   │       │
│   │   │       └───system_i.CD3F849C.tlog
│   │   │               CL.command.1.tlog
│   │   │               Cl.items.tlog
│   │   │               CL.read.1.tlog
│   │   │               CL.write.1.tlog
│   │   │               CustomBuild.command.1.tlog
│   │   │               CustomBuild.read.1.tlog
│   │   │               CustomBuild.write.1.tlog
│   │   │               link.command.1.tlog
│   │   │               link.read.1.tlog
│   │   │               link.secondary.1.tlog
│   │   │               link.write.1.tlog
│   │   │               system_integration.lastbuildstate
│   │   │
│   │   └───x64
│   │       └───Release
│   │           ├───ALL_BUILD
│   │           │   │   ALL_BUILD.recipe
│   │           │   │
│   │           │   └───ALL_BUILD.tlog
│   │           │           ALL_BUILD.lastbuildstate
│   │           │           CustomBuild.command.1.tlog
│   │           │           CustomBuild.read.1.tlog
│   │           │           CustomBuild.write.1.tlog
│   │           │
│   │           └───ZERO_CHECK
│   │               │   ZERO_CHECK.recipe
│   │               │
│   │               └───ZERO_CHECK.tlog
│   │                       CustomBuild.command.1.tlog
│   │                       CustomBuild.read.1.tlog
│   │                       CustomBuild.write.1.tlog
│   │                       ZERO_CHECK.lastbuildstate
│   │
│   ├───include
│   │       file_watcher.h
│   │
│   └───src
│       │   dummy.cpp
│       │   lib.rs
│       │
│       ├───linux
│       │       file_watcher.cpp
│       │
│       └───windows
│               file_watcher.cpp
│               file_watcher.rs
│               mod.rs
│
└───target
    │   .rustc_info.json
    │
    ├───debug
    │   │   .cargo-lock
    │   │
    │   ├───.fingerprint
    │   │   ├───adler2-7e7adc8515193442
    │   │   │       dep-lib-adler2
    │   │   │       invoked.timestamp
    │   │   │       lib-adler2
    │   │   │       lib-adler2.json
    │   │   │
    │   │   ├───aiav_core-05718593555ba341
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───aiav_core-1a4dd919b5b27738
    │   │   │       dep-lib-aiav_core
    │   │   │       invoked.timestamp
    │   │   │       lib-aiav_core
    │   │   │       lib-aiav_core.json
    │   │   │       output-lib-aiav_core
    │   │   │
    │   │   ├───aiav_core-3ef2a037aacee29a
    │   │   │       dep-test-bin-aiav_core
    │   │   │       invoked.timestamp
    │   │   │       test-bin-aiav_core
    │   │   │       test-bin-aiav_core.json
    │   │   │
    │   │   ├───aiav_core-99f5eb28e30f0f30
    │   │   │       dep-test-lib-aiav_core
    │   │   │       invoked.timestamp
    │   │   │       output-test-lib-aiav_core
    │   │   │       test-lib-aiav_core
    │   │   │       test-lib-aiav_core.json
    │   │   │
    │   │   ├───aiav_core-a512489562778412
    │   │   │       bin-aiav_core
    │   │   │       bin-aiav_core.json
    │   │   │       dep-bin-aiav_core
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───aiav_core-f54d44609f09c590
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───anyhow-19d6502f89ce14f9
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───anyhow-21c2f7832620abf9
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───anyhow-b2beece60c91017e
    │   │   │       dep-lib-anyhow
    │   │   │       invoked.timestamp
    │   │   │       lib-anyhow
    │   │   │       lib-anyhow.json
    │   │   │
    │   │   ├───async-compression-ffde80ecf977ba8b
    │   │   │       dep-lib-async_compression
    │   │   │       invoked.timestamp
    │   │   │       lib-async_compression
    │   │   │       lib-async_compression.json
    │   │   │
    │   │   ├───async-trait-cd3dc4308a91e16d
    │   │   │       dep-lib-async_trait
    │   │   │       invoked.timestamp
    │   │   │       lib-async_trait
    │   │   │       lib-async_trait.json
    │   │   │
    │   │   ├───atomic-waker-01c8d4dd37710705
    │   │   │       dep-lib-atomic_waker
    │   │   │       invoked.timestamp
    │   │   │       lib-atomic_waker
    │   │   │       lib-atomic_waker.json
    │   │   │
    │   │   ├───autocfg-1fa2b1e7854770da
    │   │   │       dep-lib-autocfg
    │   │   │       invoked.timestamp
    │   │   │       lib-autocfg
    │   │   │       lib-autocfg.json
    │   │   │
    │   │   ├───axum-core-620785971f88f428
    │   │   │       dep-lib-axum_core
    │   │   │       invoked.timestamp
    │   │   │       lib-axum_core
    │   │   │       lib-axum_core.json
    │   │   │
    │   │   ├───axum-fb01155dca0787b7
    │   │   │       dep-lib-axum
    │   │   │       invoked.timestamp
    │   │   │       lib-axum
    │   │   │       lib-axum.json
    │   │   │
    │   │   ├───axum-macros-efa3d7794c485712
    │   │   │       dep-lib-axum_macros
    │   │   │       invoked.timestamp
    │   │   │       lib-axum_macros
    │   │   │       lib-axum_macros.json
    │   │   │
    │   │   ├───base64-60dbf2162231c154
    │   │   │       dep-lib-base64
    │   │   │       invoked.timestamp
    │   │   │       lib-base64
    │   │   │       lib-base64.json
    │   │   │
    │   │   ├───bitflags-27e7531dabfdfeb8
    │   │   │       dep-lib-bitflags
    │   │   │       invoked.timestamp
    │   │   │       lib-bitflags
    │   │   │       lib-bitflags.json
    │   │   │
    │   │   ├───block-buffer-c10c457849c4983f
    │   │   │       dep-lib-block_buffer
    │   │   │       invoked.timestamp
    │   │   │       lib-block_buffer
    │   │   │       lib-block_buffer.json
    │   │   │
    │   │   ├───bytes-6f1a6f12e445e6cc
    │   │   │       dep-lib-bytes
    │   │   │       invoked.timestamp
    │   │   │       lib-bytes
    │   │   │       lib-bytes.json
    │   │   │
    │   │   ├───cc-0e574b4d15c526bd
    │   │   │       dep-lib-cc
    │   │   │       invoked.timestamp
    │   │   │       lib-cc
    │   │   │       lib-cc.json
    │   │   │
    │   │   ├───cfg-if-b4957d577da748fa
    │   │   │       dep-lib-cfg_if
    │   │   │       invoked.timestamp
    │   │   │       lib-cfg_if
    │   │   │       lib-cfg_if.json
    │   │   │
    │   │   ├───chrono-35ce3019cab68291
    │   │   │       dep-lib-chrono
    │   │   │       invoked.timestamp
    │   │   │       lib-chrono
    │   │   │       lib-chrono.json
    │   │   │
    │   │   ├───cloud-services-494d90562e88a3a9
    │   │   │       dep-test-lib-cloud_services
    │   │   │       invoked.timestamp
    │   │   │       test-lib-cloud_services
    │   │   │       test-lib-cloud_services.json
    │   │   │
    │   │   ├───cloud-services-c86445f794665994
    │   │   │       dep-lib-cloud_services
    │   │   │       invoked.timestamp
    │   │   │       lib-cloud_services
    │   │   │       lib-cloud_services.json
    │   │   │
    │   │   ├───cpufeatures-a51bb6dd08a30a0e
    │   │   │       dep-lib-cpufeatures
    │   │   │       invoked.timestamp
    │   │   │       lib-cpufeatures
    │   │   │       lib-cpufeatures.json
    │   │   │
    │   │   ├───crc32fast-2ab078f62f7ab397
    │   │   │       dep-lib-crc32fast
    │   │   │       invoked.timestamp
    │   │   │       lib-crc32fast
    │   │   │       lib-crc32fast.json
    │   │   │
    │   │   ├───crossbeam-channel-33509ab0739dffe9
    │   │   │       dep-lib-crossbeam_channel
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_channel
    │   │   │       lib-crossbeam_channel.json
    │   │   │
    │   │   ├───crossbeam-deque-e81f03f549463d17
    │   │   │       dep-lib-crossbeam_deque
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_deque
    │   │   │       lib-crossbeam_deque.json
    │   │   │
    │   │   ├───crossbeam-epoch-460701beabd9777f
    │   │   │       dep-lib-crossbeam_epoch
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_epoch
    │   │   │       lib-crossbeam_epoch.json
    │   │   │
    │   │   ├───crossbeam-utils-a5df2f396e625723
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───crossbeam-utils-d634afeca1e68d44
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───crossbeam-utils-e2b0808d7545bafd
    │   │   │       dep-lib-crossbeam_utils
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_utils
    │   │   │       lib-crossbeam_utils.json
    │   │   │
    │   │   ├───crypto-common-1b3d3d2408c8effc
    │   │   │       dep-lib-crypto_common
    │   │   │       invoked.timestamp
    │   │   │       lib-crypto_common
    │   │   │       lib-crypto_common.json
    │   │   │
    │   │   ├───data-encoding-a1b3bdd44b98cca5
    │   │   │       dep-lib-data_encoding
    │   │   │       invoked.timestamp
    │   │   │       lib-data_encoding
    │   │   │       lib-data_encoding.json
    │   │   │
    │   │   ├───deranged-2555b3a184920c70
    │   │   │       dep-lib-deranged
    │   │   │       invoked.timestamp
    │   │   │       lib-deranged
    │   │   │       lib-deranged.json
    │   │   │
    │   │   ├───digest-0d4c1d24b6218e69
    │   │   │       dep-lib-digest
    │   │   │       invoked.timestamp
    │   │   │       lib-digest
    │   │   │       lib-digest.json
    │   │   │
    │   │   ├───displaydoc-79cc6545e2314b85
    │   │   │       dep-lib-displaydoc
    │   │   │       invoked.timestamp
    │   │   │       lib-displaydoc
    │   │   │       lib-displaydoc.json
    │   │   │
    │   │   ├───either-ae6dbdca34a9d337
    │   │   │       dep-lib-either
    │   │   │       invoked.timestamp
    │   │   │       lib-either
    │   │   │       lib-either.json
    │   │   │
    │   │   ├───encoding_rs-1e470f2ee6d25279
    │   │   │       dep-lib-encoding_rs
    │   │   │       invoked.timestamp
    │   │   │       lib-encoding_rs
    │   │   │       lib-encoding_rs.json
    │   │   │
    │   │   ├───equivalent-ba0c225f2f332fde
    │   │   │       dep-lib-equivalent
    │   │   │       invoked.timestamp
    │   │   │       lib-equivalent
    │   │   │       lib-equivalent.json
    │   │   │
    │   │   ├───fastrand-0242012a3cc1e8c6
    │   │   │       dep-lib-fastrand
    │   │   │       invoked.timestamp
    │   │   │       lib-fastrand
    │   │   │       lib-fastrand.json
    │   │   │
    │   │   ├───filetime-2d2932ed99e3866a
    │   │   │       dep-lib-filetime
    │   │   │       invoked.timestamp
    │   │   │       lib-filetime
    │   │   │       lib-filetime.json
    │   │   │
    │   │   ├───flate2-54da36c851e633e7
    │   │   │       dep-lib-flate2
    │   │   │       invoked.timestamp
    │   │   │       lib-flate2
    │   │   │       lib-flate2.json
    │   │   │
    │   │   ├───fnv-3d8ef674e6ff6ec4
    │   │   │       dep-lib-fnv
    │   │   │       invoked.timestamp
    │   │   │       lib-fnv
    │   │   │       lib-fnv.json
    │   │   │
    │   │   ├───form_urlencoded-98fc11844794d987
    │   │   │       dep-lib-form_urlencoded
    │   │   │       invoked.timestamp
    │   │   │       lib-form_urlencoded
    │   │   │       lib-form_urlencoded.json
    │   │   │
    │   │   ├───fs_extra-00dfc66abecc69f7
    │   │   │       dep-lib-fs_extra
    │   │   │       invoked.timestamp
    │   │   │       lib-fs_extra
    │   │   │       lib-fs_extra.json
    │   │   │
    │   │   ├───fs_extra-5c04533ce8beb64f
    │   │   │       dep-lib-fs_extra
    │   │   │       invoked.timestamp
    │   │   │       lib-fs_extra
    │   │   │       lib-fs_extra.json
    │   │   │
    │   │   ├───futures-channel-0aa17cf82779b64f
    │   │   │       dep-lib-futures_channel
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_channel
    │   │   │       lib-futures_channel.json
    │   │   │
    │   │   ├───futures-core-10b6e226dff9bf0e
    │   │   │       dep-lib-futures_core
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_core
    │   │   │       lib-futures_core.json
    │   │   │
    │   │   ├───futures-e0f98eb3474ddadd
    │   │   │       dep-lib-futures
    │   │   │       invoked.timestamp
    │   │   │       lib-futures
    │   │   │       lib-futures.json
    │   │   │
    │   │   ├───futures-executor-cd33e983e42d3e23
    │   │   │       dep-lib-futures_executor
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_executor
    │   │   │       lib-futures_executor.json
    │   │   │
    │   │   ├───futures-io-07071dc0a1ce2b00
    │   │   │       dep-lib-futures_io
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_io
    │   │   │       lib-futures_io.json
    │   │   │
    │   │   ├───futures-macro-aeb6aaca5024ef59
    │   │   │       dep-lib-futures_macro
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_macro
    │   │   │       lib-futures_macro.json
    │   │   │
    │   │   ├───futures-sink-d328231113ed7cc6
    │   │   │       dep-lib-futures_sink
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_sink
    │   │   │       lib-futures_sink.json
    │   │   │
    │   │   ├───futures-task-e9ec59bd49a29dbb
    │   │   │       dep-lib-futures_task
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_task
    │   │   │       lib-futures_task.json
    │   │   │
    │   │   ├───futures-util-e1a3116177e9177d
    │   │   │       dep-lib-futures_util
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_util
    │   │   │       lib-futures_util.json
    │   │   │
    │   │   ├───generic-array-a0f22693c1710e7e
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───generic-array-ee14a9da454cd5b7
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───generic-array-fd60979d0bc4cf39
    │   │   │       dep-lib-generic_array
    │   │   │       invoked.timestamp
    │   │   │       lib-generic_array
    │   │   │       lib-generic_array.json
    │   │   │
    │   │   ├───getrandom-3203b03998c9f0cd
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───getrandom-6d50dd364829bdb7
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───getrandom-c2585604dd79c6e6
    │   │   │       dep-lib-getrandom
    │   │   │       invoked.timestamp
    │   │   │       lib-getrandom
    │   │   │       lib-getrandom.json
    │   │   │
    │   │   ├───glob-a833c8eeb3b37e3b
    │   │   │       dep-lib-glob
    │   │   │       invoked.timestamp
    │   │   │       lib-glob
    │   │   │       lib-glob.json
    │   │   │
    │   │   ├───h2-5856b2c800458c51
    │   │   │       dep-lib-h2
    │   │   │       invoked.timestamp
    │   │   │       lib-h2
    │   │   │       lib-h2.json
    │   │   │
    │   │   ├───hashbrown-6d6277d712f36016
    │   │   │       dep-lib-hashbrown
    │   │   │       invoked.timestamp
    │   │   │       lib-hashbrown
    │   │   │       lib-hashbrown.json
    │   │   │
    │   │   ├───http-194c4970bf8d03e5
    │   │   │       dep-lib-http
    │   │   │       invoked.timestamp
    │   │   │       lib-http
    │   │   │       lib-http.json
    │   │   │
    │   │   ├───http-body-a699eb9b3b1d78fc
    │   │   │       dep-lib-http_body
    │   │   │       invoked.timestamp
    │   │   │       lib-http_body
    │   │   │       lib-http_body.json
    │   │   │
    │   │   ├───http-body-util-36f2f55f01df4d52
    │   │   │       dep-lib-http_body_util
    │   │   │       invoked.timestamp
    │   │   │       lib-http_body_util
    │   │   │       lib-http_body_util.json
    │   │   │
    │   │   ├───httparse-40a10619fa24a598
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───httparse-b37724c101282493
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───httparse-b4fb1babb990ded4
    │   │   │       dep-lib-httparse
    │   │   │       invoked.timestamp
    │   │   │       lib-httparse
    │   │   │       lib-httparse.json
    │   │   │
    │   │   ├───httpdate-3aca05d1c52163aa
    │   │   │       dep-lib-httpdate
    │   │   │       invoked.timestamp
    │   │   │       lib-httpdate
    │   │   │       lib-httpdate.json
    │   │   │
    │   │   ├───hyper-69e47b41336db0bb
    │   │   │       dep-lib-hyper
    │   │   │       invoked.timestamp
    │   │   │       lib-hyper
    │   │   │       lib-hyper.json
    │   │   │
    │   │   ├───hyper-tls-3c5fab58960deab5
    │   │   │       dep-lib-hyper_tls
    │   │   │       invoked.timestamp
    │   │   │       lib-hyper_tls
    │   │   │       lib-hyper_tls.json
    │   │   │
    │   │   ├───hyper-util-ca28f1e207e9405f
    │   │   │       dep-lib-hyper_util
    │   │   │       invoked.timestamp
    │   │   │       lib-hyper_util
    │   │   │       lib-hyper_util.json
    │   │   │
    │   │   ├───icu_collections-f28effc508a5242d
    │   │   │       dep-lib-icu_collections
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_collections
    │   │   │       lib-icu_collections.json
    │   │   │
    │   │   ├───icu_locale_core-76f927c3c8ca1c31
    │   │   │       dep-lib-icu_locale_core
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_locale_core
    │   │   │       lib-icu_locale_core.json
    │   │   │
    │   │   ├───icu_normalizer-31e3f3b8f4713f83
    │   │   │       dep-lib-icu_normalizer
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_normalizer
    │   │   │       lib-icu_normalizer.json
    │   │   │
    │   │   ├───icu_normalizer_data-153ac4042caa43ce
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───icu_normalizer_data-99b7b753be955813
    │   │   │       dep-lib-icu_normalizer_data
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_normalizer_data
    │   │   │       lib-icu_normalizer_data.json
    │   │   │
    │   │   ├───icu_normalizer_data-b4502e787760986a
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───icu_properties-b8d8922db1647abb
    │   │   │       dep-lib-icu_properties
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_properties
    │   │   │       lib-icu_properties.json
    │   │   │
    │   │   ├───icu_properties_data-5f2d3d09b3bee03d
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───icu_properties_data-9c14d4744353a58e
    │   │   │       dep-lib-icu_properties_data
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_properties_data
    │   │   │       lib-icu_properties_data.json
    │   │   │
    │   │   ├───icu_properties_data-ca168c736a483536
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───icu_provider-50717ef4a4743ecf
    │   │   │       dep-lib-icu_provider
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_provider
    │   │   │       lib-icu_provider.json
    │   │   │
    │   │   ├───idna-fd2a2901cc42a5d3
    │   │   │       dep-lib-idna
    │   │   │       invoked.timestamp
    │   │   │       lib-idna
    │   │   │       lib-idna.json
    │   │   │
    │   │   ├───idna_adapter-3b6fac3f49374e90
    │   │   │       dep-lib-idna_adapter
    │   │   │       invoked.timestamp
    │   │   │       lib-idna_adapter
    │   │   │       lib-idna_adapter.json
    │   │   │
    │   │   ├───indexmap-4b56ca83916d7ae7
    │   │   │       dep-lib-indexmap
    │   │   │       invoked.timestamp
    │   │   │       lib-indexmap
    │   │   │       lib-indexmap.json
    │   │   │
    │   │   ├───ipnet-b7f33d3ead5d12fb
    │   │   │       dep-lib-ipnet
    │   │   │       invoked.timestamp
    │   │   │       lib-ipnet
    │   │   │       lib-ipnet.json
    │   │   │
    │   │   ├───iri-string-72b78bf5585c7f08
    │   │   │       dep-lib-iri_string
    │   │   │       invoked.timestamp
    │   │   │       lib-iri_string
    │   │   │       lib-iri_string.json
    │   │   │
    │   │   ├───itoa-69297a500bb545a7
    │   │   │       dep-lib-itoa
    │   │   │       invoked.timestamp
    │   │   │       lib-itoa
    │   │   │       lib-itoa.json
    │   │   │
    │   │   ├───lazy_static-a00a19492b117dcc
    │   │   │       dep-lib-lazy_static
    │   │   │       invoked.timestamp
    │   │   │       lib-lazy_static
    │   │   │       lib-lazy_static.json
    │   │   │
    │   │   ├───libc-56cdc4a54f543ac6
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───libc-5fa4bf4c31f0f91d
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───libc-bb560bf5cffc4727
    │   │   │       dep-lib-libc
    │   │   │       invoked.timestamp
    │   │   │       lib-libc
    │   │   │       lib-libc.json
    │   │   │
    │   │   ├───litemap-99aba40979849771
    │   │   │       dep-lib-litemap
    │   │   │       invoked.timestamp
    │   │   │       lib-litemap
    │   │   │       lib-litemap.json
    │   │   │
    │   │   ├───lock_api-56afa19201392373
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───lock_api-5de86c30bdcd6951
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───lock_api-e9d8b5157e526545
    │   │   │       dep-lib-lock_api
    │   │   │       invoked.timestamp
    │   │   │       lib-lock_api
    │   │   │       lib-lock_api.json
    │   │   │
    │   │   ├───log-4fd96ec37443cbba
    │   │   │       dep-lib-log
    │   │   │       invoked.timestamp
    │   │   │       lib-log
    │   │   │       lib-log.json
    │   │   │
    │   │   ├───matchers-98113fb8bc145d9f
    │   │   │       dep-lib-matchers
    │   │   │       invoked.timestamp
    │   │   │       lib-matchers
    │   │   │       lib-matchers.json
    │   │   │
    │   │   ├───matchit-8e5c37df1ea8ce26
    │   │   │       dep-lib-matchit
    │   │   │       invoked.timestamp
    │   │   │       lib-matchit
    │   │   │       lib-matchit.json
    │   │   │
    │   │   ├───memchr-d21ed81bcc5c2a12
    │   │   │       dep-lib-memchr
    │   │   │       invoked.timestamp
    │   │   │       lib-memchr
    │   │   │       lib-memchr.json
    │   │   │
    │   │   ├───mime-f51d29a9076e2fef
    │   │   │       dep-lib-mime
    │   │   │       invoked.timestamp
    │   │   │       lib-mime
    │   │   │       lib-mime.json
    │   │   │
    │   │   ├───miniz_oxide-5de3e2f572df22da
    │   │   │       dep-lib-miniz_oxide
    │   │   │       invoked.timestamp
    │   │   │       lib-miniz_oxide
    │   │   │       lib-miniz_oxide.json
    │   │   │
    │   │   ├───mio-b74fc8fbf29e9adb
    │   │   │       dep-lib-mio
    │   │   │       invoked.timestamp
    │   │   │       lib-mio
    │   │   │       lib-mio.json
    │   │   │
    │   │   ├───native-tls-17b597484dc1f060
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───native-tls-64ae2d18b82be3d3
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───native-tls-91f085e89949adf0
    │   │   │       dep-lib-native_tls
    │   │   │       invoked.timestamp
    │   │   │       lib-native_tls
    │   │   │       lib-native_tls.json
    │   │   │
    │   │   ├───notify-7514b83b65fb1344
    │   │   │       dep-lib-notify
    │   │   │       invoked.timestamp
    │   │   │       lib-notify
    │   │   │       lib-notify.json
    │   │   │
    │   │   ├───notify-9511bfd2115a3c48
    │   │   │       dep-lib-notify
    │   │   │       invoked.timestamp
    │   │   │       lib-notify
    │   │   │       lib-notify.json
    │   │   │
    │   │   ├───notify-types-cf90db815a4b4217
    │   │   │       dep-lib-notify_types
    │   │   │       invoked.timestamp
    │   │   │       lib-notify_types
    │   │   │       lib-notify_types.json
    │   │   │
    │   │   ├───nu-ansi-term-e57b887bb81d1805
    │   │   │       dep-lib-nu_ansi_term
    │   │   │       invoked.timestamp
    │   │   │       lib-nu_ansi_term
    │   │   │       lib-nu_ansi_term.json
    │   │   │
    │   │   ├───num-conv-55ec824b6f8f9351
    │   │   │       dep-lib-num_conv
    │   │   │       invoked.timestamp
    │   │   │       lib-num_conv
    │   │   │       lib-num_conv.json
    │   │   │
    │   │   ├───num-traits-0019386860619968
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───num-traits-8c538d1c0fb7c5a1
    │   │   │       dep-lib-num_traits
    │   │   │       invoked.timestamp
    │   │   │       lib-num_traits
    │   │   │       lib-num_traits.json
    │   │   │
    │   │   ├───num-traits-b509cd018bb3b96d
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───once_cell-02d9b1423b4c3a99
    │   │   │       dep-lib-once_cell
    │   │   │       invoked.timestamp
    │   │   │       lib-once_cell
    │   │   │       lib-once_cell.json
    │   │   │
    │   │   ├───overload-39bd931dd46f4fb1
    │   │   │       dep-lib-overload
    │   │   │       invoked.timestamp
    │   │   │       lib-overload
    │   │   │       lib-overload.json
    │   │   │
    │   │   ├───parking_lot-5320cbe173ddeca5
    │   │   │       dep-lib-parking_lot
    │   │   │       invoked.timestamp
    │   │   │       lib-parking_lot
    │   │   │       lib-parking_lot.json
    │   │   │
    │   │   ├───parking_lot_core-87879622a5efa30a
    │   │   │       dep-lib-parking_lot_core
    │   │   │       invoked.timestamp
    │   │   │       lib-parking_lot_core
    │   │   │       lib-parking_lot_core.json
    │   │   │
    │   │   ├───parking_lot_core-8f0d24a5b39e99b7
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───parking_lot_core-9ba37945ad05bba4
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───percent-encoding-226c9e1a82702448
    │   │   │       dep-lib-percent_encoding
    │   │   │       invoked.timestamp
    │   │   │       lib-percent_encoding
    │   │   │       lib-percent_encoding.json
    │   │   │
    │   │   ├───pin-project-lite-360f5ac79d6490a8
    │   │   │       dep-lib-pin_project_lite
    │   │   │       invoked.timestamp
    │   │   │       lib-pin_project_lite
    │   │   │       lib-pin_project_lite.json
    │   │   │
    │   │   ├───pin-utils-6131d7e882b3d82e
    │   │   │       dep-lib-pin_utils
    │   │   │       invoked.timestamp
    │   │   │       lib-pin_utils
    │   │   │       lib-pin_utils.json
    │   │   │
    │   │   ├───potential_utf-46997aa979ed06de
    │   │   │       dep-lib-potential_utf
    │   │   │       invoked.timestamp
    │   │   │       lib-potential_utf
    │   │   │       lib-potential_utf.json
    │   │   │
    │   │   ├───powerfmt-19f2ce97eac604ff
    │   │   │       dep-lib-powerfmt
    │   │   │       invoked.timestamp
    │   │   │       lib-powerfmt
    │   │   │       lib-powerfmt.json
    │   │   │
    │   │   ├───ppv-lite86-968964c6cb7b2a5c
    │   │   │       dep-lib-ppv_lite86
    │   │   │       invoked.timestamp
    │   │   │       lib-ppv_lite86
    │   │   │       lib-ppv_lite86.json
    │   │   │
    │   │   ├───proc-macro2-3b1f514462257677
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───proc-macro2-6a20e463e4a4ca20
    │   │   │       dep-lib-proc_macro2
    │   │   │       invoked.timestamp
    │   │   │       lib-proc_macro2
    │   │   │       lib-proc_macro2.json
    │   │   │
    │   │   ├───proc-macro2-8ecdc549423c9909
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───quote-fdeee9df46234d2d
    │   │   │       dep-lib-quote
    │   │   │       invoked.timestamp
    │   │   │       lib-quote
    │   │   │       lib-quote.json
    │   │   │
    │   │   ├───rand-1359bed459c03497
    │   │   │       dep-lib-rand
    │   │   │       invoked.timestamp
    │   │   │       lib-rand
    │   │   │       lib-rand.json
    │   │   │
    │   │   ├───rand_chacha-6d2e0a3d1d0cf058
    │   │   │       dep-lib-rand_chacha
    │   │   │       invoked.timestamp
    │   │   │       lib-rand_chacha
    │   │   │       lib-rand_chacha.json
    │   │   │
    │   │   ├───rand_core-9dcb78c9132a4656
    │   │   │       dep-lib-rand_core
    │   │   │       invoked.timestamp
    │   │   │       lib-rand_core
    │   │   │       lib-rand_core.json
    │   │   │
    │   │   ├───rayon-core-58356cac7752356c
    │   │   │       dep-lib-rayon_core
    │   │   │       invoked.timestamp
    │   │   │       lib-rayon_core
    │   │   │       lib-rayon_core.json
    │   │   │
    │   │   ├───rayon-core-991b80ea2d6a82dc
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───rayon-core-f628916875db1b51
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───rayon-dfab54025ddc28fc
    │   │   │       dep-lib-rayon
    │   │   │       invoked.timestamp
    │   │   │       lib-rayon
    │   │   │       lib-rayon.json
    │   │   │
    │   │   ├───regex-7126983b5ef91054
    │   │   │       dep-lib-regex
    │   │   │       invoked.timestamp
    │   │   │       lib-regex
    │   │   │       lib-regex.json
    │   │   │
    │   │   ├───regex-automata-91efb59fcc2b2c23
    │   │   │       dep-lib-regex_automata
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_automata
    │   │   │       lib-regex_automata.json
    │   │   │
    │   │   ├───regex-automata-ae3084067eb369ab
    │   │   │       dep-lib-regex_automata
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_automata
    │   │   │       lib-regex_automata.json
    │   │   │
    │   │   ├───regex-syntax-2ddbb26afe1c0a64
    │   │   │       dep-lib-regex_syntax
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_syntax
    │   │   │       lib-regex_syntax.json
    │   │   │
    │   │   ├───regex-syntax-bfd7e1ff807f1064
    │   │   │       dep-lib-regex_syntax
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_syntax
    │   │   │       lib-regex_syntax.json
    │   │   │
    │   │   ├───reqwest-a31f51c3b0b65d42
    │   │   │       dep-lib-reqwest
    │   │   │       invoked.timestamp
    │   │   │       lib-reqwest
    │   │   │       lib-reqwest.json
    │   │   │
    │   │   ├───rustls-pki-types-d9f71a2bd6fc478f
    │   │   │       dep-lib-rustls_pki_types
    │   │   │       invoked.timestamp
    │   │   │       lib-rustls_pki_types
    │   │   │       lib-rustls_pki_types.json
    │   │   │
    │   │   ├───rustversion-0e6f9310c8b30083
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───rustversion-15ab6800fc85e5bd
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───rustversion-d5408a2b808eb648
    │   │   │       dep-lib-rustversion
    │   │   │       invoked.timestamp
    │   │   │       lib-rustversion
    │   │   │       lib-rustversion.json
    │   │   │
    │   │   ├───ryu-3442447a74614b27
    │   │   │       dep-lib-ryu
    │   │   │       invoked.timestamp
    │   │   │       lib-ryu
    │   │   │       lib-ryu.json
    │   │   │
    │   │   ├───same-file-2da6eccfbf8519af
    │   │   │       dep-lib-same_file
    │   │   │       invoked.timestamp
    │   │   │       lib-same_file
    │   │   │       lib-same_file.json
    │   │   │
    │   │   ├───schannel-8868a50aa66715dc
    │   │   │       dep-lib-schannel
    │   │   │       invoked.timestamp
    │   │   │       lib-schannel
    │   │   │       lib-schannel.json
    │   │   │
    │   │   ├───scopeguard-66a9ca5b98e4d6bf
    │   │   │       dep-lib-scopeguard
    │   │   │       invoked.timestamp
    │   │   │       lib-scopeguard
    │   │   │       lib-scopeguard.json
    │   │   │
    │   │   ├───serde-738b9b7ceb6d464b
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───serde-e2c5fc6a4f673cbb
    │   │   │       dep-lib-serde
    │   │   │       invoked.timestamp
    │   │   │       lib-serde
    │   │   │       lib-serde.json
    │   │   │
    │   │   ├───serde-f0e00f2ffbef6b75
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───serde_derive-ba6d0f103c04c0d5
    │   │   │       dep-lib-serde_derive
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_derive
    │   │   │       lib-serde_derive.json
    │   │   │
    │   │   ├───serde_json-1d7dd5643a7e2f74
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───serde_json-4f4830174827aea7
    │   │   │       dep-lib-serde_json
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_json
    │   │   │       lib-serde_json.json
    │   │   │
    │   │   ├───serde_json-d326e683abaf143e
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───serde_path_to_error-bc040f3aaac335b7
    │   │   │       dep-lib-serde_path_to_error
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_path_to_error
    │   │   │       lib-serde_path_to_error.json
    │   │   │
    │   │   ├───serde_spanned-070ad526d1db460c
    │   │   │       dep-lib-serde_spanned
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_spanned
    │   │   │       lib-serde_spanned.json
    │   │   │
    │   │   ├───serde_urlencoded-980a8cf95a86cfca
    │   │   │       dep-lib-serde_urlencoded
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_urlencoded
    │   │   │       lib-serde_urlencoded.json
    │   │   │
    │   │   ├───sha1-4483d9d32339807e
    │   │   │       dep-lib-sha1
    │   │   │       invoked.timestamp
    │   │   │       lib-sha1
    │   │   │       lib-sha1.json
    │   │   │
    │   │   ├───sha2-64b0ec1eee2551f5
    │   │   │       dep-lib-sha2
    │   │   │       invoked.timestamp
    │   │   │       lib-sha2
    │   │   │       lib-sha2.json
    │   │   │
    │   │   ├───sharded-slab-74a2b9b853e8fc13
    │   │   │       dep-lib-sharded_slab
    │   │   │       invoked.timestamp
    │   │   │       lib-sharded_slab
    │   │   │       lib-sharded_slab.json
    │   │   │
    │   │   ├───shlex-27ba53e6f2866fee
    │   │   │       dep-lib-shlex
    │   │   │       invoked.timestamp
    │   │   │       lib-shlex
    │   │   │       lib-shlex.json
    │   │   │
    │   │   ├───slab-6403e07a840fb0da
    │   │   │       dep-lib-slab
    │   │   │       invoked.timestamp
    │   │   │       lib-slab
    │   │   │       lib-slab.json
    │   │   │
    │   │   ├───smallvec-fd9fc186d6d6180f
    │   │   │       dep-lib-smallvec
    │   │   │       invoked.timestamp
    │   │   │       lib-smallvec
    │   │   │       lib-smallvec.json
    │   │   │
    │   │   ├───socket2-7074eedfe5b08109
    │   │   │       dep-lib-socket2
    │   │   │       invoked.timestamp
    │   │   │       lib-socket2
    │   │   │       lib-socket2.json
    │   │   │
    │   │   ├───stable_deref_trait-a5623622a362b8d6
    │   │   │       dep-lib-stable_deref_trait
    │   │   │       invoked.timestamp
    │   │   │       lib-stable_deref_trait
    │   │   │       lib-stable_deref_trait.json
    │   │   │
    │   │   ├───syn-a3600ff6e0b8e983
    │   │   │       dep-lib-syn
    │   │   │       invoked.timestamp
    │   │   │       lib-syn
    │   │   │       lib-syn.json
    │   │   │
    │   │   ├───sync_wrapper-b8ddcd24d18460c7
    │   │   │       dep-lib-sync_wrapper
    │   │   │       invoked.timestamp
    │   │   │       lib-sync_wrapper
    │   │   │       lib-sync_wrapper.json
    │   │   │
    │   │   ├───synstructure-75469bf036e5ef30
    │   │   │       dep-lib-synstructure
    │   │   │       invoked.timestamp
    │   │   │       lib-synstructure
    │   │   │       lib-synstructure.json
    │   │   │
    │   │   ├───system-integration-0d7081e5500ceadc
    │   │   │       dep-lib-system_integration
    │   │   │       invoked.timestamp
    │   │   │       lib-system_integration
    │   │   │       lib-system_integration.json
    │   │   │
    │   │   ├───system-integration-ce3a0595cc56995e
    │   │   │       dep-test-lib-system_integration
    │   │   │       invoked.timestamp
    │   │   │       test-lib-system_integration
    │   │   │       test-lib-system_integration.json
    │   │   │
    │   │   ├───tempfile-d2038a299aaf87c9
    │   │   │       dep-lib-tempfile
    │   │   │       invoked.timestamp
    │   │   │       lib-tempfile
    │   │   │       lib-tempfile.json
    │   │   │
    │   │   ├───thiserror-2b3c7dabc4d19fad
    │   │   │       dep-lib-thiserror
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror
    │   │   │       lib-thiserror.json
    │   │   │
    │   │   ├───thiserror-4396a7df873958dd
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───thiserror-60db7b5760e08792
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───thiserror-7a538730508d99d9
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───thiserror-aeed0cd22ee35f1b
    │   │   │       dep-lib-thiserror
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror
    │   │   │       lib-thiserror.json
    │   │   │
    │   │   ├───thiserror-b6cf77148e1e9b33
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───thiserror-impl-08d57bf759d359db
    │   │   │       dep-lib-thiserror_impl
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror_impl
    │   │   │       lib-thiserror_impl.json
    │   │   │
    │   │   ├───thiserror-impl-d09ea1689e762b18
    │   │   │       dep-lib-thiserror_impl
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror_impl
    │   │   │       lib-thiserror_impl.json
    │   │   │
    │   │   ├───thread_local-88b751e2b9939dde
    │   │   │       dep-lib-thread_local
    │   │   │       invoked.timestamp
    │   │   │       lib-thread_local
    │   │   │       lib-thread_local.json
    │   │   │
    │   │   ├───time-4f9d624dfe7f47ca
    │   │   │       dep-lib-time
    │   │   │       invoked.timestamp
    │   │   │       lib-time
    │   │   │       lib-time.json
    │   │   │
    │   │   ├───time-core-6c3a95f088e22c10
    │   │   │       dep-lib-time_core
    │   │   │       invoked.timestamp
    │   │   │       lib-time_core
    │   │   │       lib-time_core.json
    │   │   │
    │   │   ├───time-fmt-56b43a9ab887d666
    │   │   │       dep-lib-time_fmt
    │   │   │       invoked.timestamp
    │   │   │       lib-time_fmt
    │   │   │       lib-time_fmt.json
    │   │   │
    │   │   ├───tinystr-95da5c26ea10feb2
    │   │   │       dep-lib-tinystr
    │   │   │       invoked.timestamp
    │   │   │       lib-tinystr
    │   │   │       lib-tinystr.json
    │   │   │
    │   │   ├───tokio-9813a62fc61b5656
    │   │   │       dep-lib-tokio
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio
    │   │   │       lib-tokio.json
    │   │   │
    │   │   ├───tokio-macros-b106dd764d7a03ff
    │   │   │       dep-lib-tokio_macros
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_macros
    │   │   │       lib-tokio_macros.json
    │   │   │
    │   │   ├───tokio-native-tls-1ad6c090cf32fb29
    │   │   │       dep-lib-tokio_native_tls
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_native_tls
    │   │   │       lib-tokio_native_tls.json
    │   │   │
    │   │   ├───tokio-tungstenite-4dcc290b6a4dc2e1
    │   │   │       dep-lib-tokio_tungstenite
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_tungstenite
    │   │   │       lib-tokio_tungstenite.json
    │   │   │
    │   │   ├───tokio-util-70e574008207cea0
    │   │   │       dep-lib-tokio_util
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_util
    │   │   │       lib-tokio_util.json
    │   │   │
    │   │   ├───toml-536cef40ee9c60a1
    │   │   │       dep-lib-toml
    │   │   │       invoked.timestamp
    │   │   │       lib-toml
    │   │   │       lib-toml.json
    │   │   │
    │   │   ├───toml_datetime-fe29046d2032b3d6
    │   │   │       dep-lib-toml_datetime
    │   │   │       invoked.timestamp
    │   │   │       lib-toml_datetime
    │   │   │       lib-toml_datetime.json
    │   │   │
    │   │   ├───toml_edit-afc4149e6bfee789
    │   │   │       dep-lib-toml_edit
    │   │   │       invoked.timestamp
    │   │   │       lib-toml_edit
    │   │   │       lib-toml_edit.json
    │   │   │
    │   │   ├───toml_write-d358a4b373d090f4
    │   │   │       dep-lib-toml_write
    │   │   │       invoked.timestamp
    │   │   │       lib-toml_write
    │   │   │       lib-toml_write.json
    │   │   │
    │   │   ├───tower-9394206f949c9c86
    │   │   │       dep-lib-tower
    │   │   │       invoked.timestamp
    │   │   │       lib-tower
    │   │   │       lib-tower.json
    │   │   │
    │   │   ├───tower-http-94deac2c047dc809
    │   │   │       dep-lib-tower_http
    │   │   │       invoked.timestamp
    │   │   │       lib-tower_http
    │   │   │       lib-tower_http.json
    │   │   │
    │   │   ├───tower-layer-7e185b956ba54a13
    │   │   │       dep-lib-tower_layer
    │   │   │       invoked.timestamp
    │   │   │       lib-tower_layer
    │   │   │       lib-tower_layer.json
    │   │   │
    │   │   ├───tower-service-2927cc59f3517aa6
    │   │   │       dep-lib-tower_service
    │   │   │       invoked.timestamp
    │   │   │       lib-tower_service
    │   │   │       lib-tower_service.json
    │   │   │
    │   │   ├───tracing-56d9c2f009985c2c
    │   │   │       dep-lib-tracing
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing
    │   │   │       lib-tracing.json
    │   │   │
    │   │   ├───tracing-appender-c605e37596d64275
    │   │   │       dep-lib-tracing_appender
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_appender
    │   │   │       lib-tracing_appender.json
    │   │   │
    │   │   ├───tracing-attributes-6763a40155b38c11
    │   │   │       dep-lib-tracing_attributes
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_attributes
    │   │   │       lib-tracing_attributes.json
    │   │   │
    │   │   ├───tracing-core-98f4e7a70ae1824a
    │   │   │       dep-lib-tracing_core
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_core
    │   │   │       lib-tracing_core.json
    │   │   │
    │   │   ├───tracing-log-80b1016ed78c7b75
    │   │   │       dep-lib-tracing_log
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_log
    │   │   │       lib-tracing_log.json
    │   │   │
    │   │   ├───tracing-serde-3e74548d99c607de
    │   │   │       dep-lib-tracing_serde
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_serde
    │   │   │       lib-tracing_serde.json
    │   │   │
    │   │   ├───tracing-subscriber-95ae81b028daa2fa
    │   │   │       dep-lib-tracing_subscriber
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_subscriber
    │   │   │       lib-tracing_subscriber.json
    │   │   │
    │   │   ├───try-lock-2bee410b7992e624
    │   │   │       dep-lib-try_lock
    │   │   │       invoked.timestamp
    │   │   │       lib-try_lock
    │   │   │       lib-try_lock.json
    │   │   │
    │   │   ├───tungstenite-a72e30b99c65c2c4
    │   │   │       dep-lib-tungstenite
    │   │   │       invoked.timestamp
    │   │   │       lib-tungstenite
    │   │   │       lib-tungstenite.json
    │   │   │
    │   │   ├───typenum-4e45f7be503c3a9e
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───typenum-961b033b496ec94c
    │   │   │       dep-lib-typenum
    │   │   │       invoked.timestamp
    │   │   │       lib-typenum
    │   │   │       lib-typenum.json
    │   │   │
    │   │   ├───typenum-d94741fc46dc64f4
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───unicode-ident-a3528efee345a255
    │   │   │       dep-lib-unicode_ident
    │   │   │       invoked.timestamp
    │   │   │       lib-unicode_ident
    │   │   │       lib-unicode_ident.json
    │   │   │
    │   │   ├───url-a1207ccce8aa94f1
    │   │   │       dep-lib-url
    │   │   │       invoked.timestamp
    │   │   │       lib-url
    │   │   │       lib-url.json
    │   │   │
    │   │   ├───utf-8-33a96f27ac303f0c
    │   │   │       dep-lib-utf8
    │   │   │       invoked.timestamp
    │   │   │       lib-utf8
    │   │   │       lib-utf8.json
    │   │   │
    │   │   ├───utf8_iter-67205964bf5d7618
    │   │   │       dep-lib-utf8_iter
    │   │   │       invoked.timestamp
    │   │   │       lib-utf8_iter
    │   │   │       lib-utf8_iter.json
    │   │   │
    │   │   ├───uuid-75423bf9870a83c5
    │   │   │       dep-lib-uuid
    │   │   │       invoked.timestamp
    │   │   │       lib-uuid
    │   │   │       lib-uuid.json
    │   │   │
    │   │   ├───version_check-793d72f0c6a3aed0
    │   │   │       dep-lib-version_check
    │   │   │       invoked.timestamp
    │   │   │       lib-version_check
    │   │   │       lib-version_check.json
    │   │   │
    │   │   ├───walkdir-32991585e463f7bb
    │   │   │       dep-lib-walkdir
    │   │   │       invoked.timestamp
    │   │   │       lib-walkdir
    │   │   │       lib-walkdir.json
    │   │   │
    │   │   ├───want-c4492102deb9fa34
    │   │   │       dep-lib-want
    │   │   │       invoked.timestamp
    │   │   │       lib-want
    │   │   │       lib-want.json
    │   │   │
    │   │   ├───winapi-1c985f109ede21a0
    │   │   │       dep-lib-winapi
    │   │   │       invoked.timestamp
    │   │   │       lib-winapi
    │   │   │       lib-winapi.json
    │   │   │
    │   │   ├───winapi-69aa28f840d42b71
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───winapi-f996034bfae824c0
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───winapi-util-bf148f29d8680adc
    │   │   │       dep-lib-winapi_util
    │   │   │       invoked.timestamp
    │   │   │       lib-winapi_util
    │   │   │       lib-winapi_util.json
    │   │   │
    │   │   ├───windows-link-21d82eae9c854e31
    │   │   │       dep-lib-windows_link
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_link
    │   │   │       lib-windows_link.json
    │   │   │
    │   │   ├───windows-registry-596af5a0f5ffe08d
    │   │   │       dep-lib-windows_registry
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_registry
    │   │   │       lib-windows_registry.json
    │   │   │
    │   │   ├───windows-result-5a3169ef8407aca2
    │   │   │       dep-lib-windows_result
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_result
    │   │   │       lib-windows_result.json
    │   │   │
    │   │   ├───windows-strings-12d818ac320dacd5
    │   │   │       dep-lib-windows_strings
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_strings
    │   │   │       lib-windows_strings.json
    │   │   │
    │   │   ├───windows-sys-373280222169ebae
    │   │   │       dep-lib-windows_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_sys
    │   │   │       lib-windows_sys.json
    │   │   │
    │   │   ├───windows-sys-cfb02564d4c64123
    │   │   │       dep-lib-windows_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_sys
    │   │   │       lib-windows_sys.json
    │   │   │
    │   │   ├───windows-sys-e73f6f53a92904f0
    │   │   │       dep-lib-windows_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_sys
    │   │   │       lib-windows_sys.json
    │   │   │
    │   │   ├───windows-targets-685851b4a7182906
    │   │   │       dep-lib-windows_targets
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_targets
    │   │   │       lib-windows_targets.json
    │   │   │
    │   │   ├───windows-targets-b5624e17f0a1ad8b
    │   │   │       dep-lib-windows_targets
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_targets
    │   │   │       lib-windows_targets.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-24e925349af9f453
    │   │   │       dep-lib-windows_x86_64_msvc
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_x86_64_msvc
    │   │   │       lib-windows_x86_64_msvc.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-3e6e4ddbeeceebc3
    │   │   │       dep-lib-windows_x86_64_msvc
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_x86_64_msvc
    │   │   │       lib-windows_x86_64_msvc.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-a13bad8f0db74227
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-a21d0cd880e0be5c
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───windows_x86_64_msvc-e8e607e3652a8546
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-fbba072248a2e0fb
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───winnow-c3ec360aed9dee22
    │   │   │       dep-lib-winnow
    │   │   │       invoked.timestamp
    │   │   │       lib-winnow
    │   │   │       lib-winnow.json
    │   │   │
    │   │   ├───writeable-f3ed0b6a2407068f
    │   │   │       dep-lib-writeable
    │   │   │       invoked.timestamp
    │   │   │       lib-writeable
    │   │   │       lib-writeable.json
    │   │   │
    │   │   ├───yara-792cb22a67194d72
    │   │   │       dep-lib-yara
    │   │   │       invoked.timestamp
    │   │   │       lib-yara
    │   │   │       lib-yara.json
    │   │   │
    │   │   ├───yara-sys-082d91747e0e5067
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───yara-sys-21bd3d982b34f2ec
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───yara-sys-d0dccd4b2047a8fe
    │   │   │       dep-lib-yara_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-yara_sys
    │   │   │       lib-yara_sys.json
    │   │   │
    │   │   ├───yoke-1e93e6e17e5bc6fb
    │   │   │       dep-lib-yoke
    │   │   │       invoked.timestamp
    │   │   │       lib-yoke
    │   │   │       lib-yoke.json
    │   │   │
    │   │   ├───yoke-derive-6bb03d86b96f97db
    │   │   │       dep-lib-yoke_derive
    │   │   │       invoked.timestamp
    │   │   │       lib-yoke_derive
    │   │   │       lib-yoke_derive.json
    │   │   │
    │   │   ├───zerocopy-16d3069820863144
    │   │   │       dep-lib-zerocopy
    │   │   │       invoked.timestamp
    │   │   │       lib-zerocopy
    │   │   │       lib-zerocopy.json
    │   │   │
    │   │   ├───zerocopy-191912f0e0e75127
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───zerocopy-dbd6279178c04a94
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───zerofrom-7b4119952d0a1a19
    │   │   │       dep-lib-zerofrom
    │   │   │       invoked.timestamp
    │   │   │       lib-zerofrom
    │   │   │       lib-zerofrom.json
    │   │   │
    │   │   ├───zerofrom-derive-426526b1eb347b0d
    │   │   │       dep-lib-zerofrom_derive
    │   │   │       invoked.timestamp
    │   │   │       lib-zerofrom_derive
    │   │   │       lib-zerofrom_derive.json
    │   │   │
    │   │   ├───zeroize-f2a9032fdcbcbf6b
    │   │   │       dep-lib-zeroize
    │   │   │       invoked.timestamp
    │   │   │       lib-zeroize
    │   │   │       lib-zeroize.json
    │   │   │
    │   │   ├───zerotrie-f4827d4265141eeb
    │   │   │       dep-lib-zerotrie
    │   │   │       invoked.timestamp
    │   │   │       lib-zerotrie
    │   │   │       lib-zerotrie.json
    │   │   │
    │   │   ├───zerovec-0e42ca7652e42ae4
    │   │   │       dep-lib-zerovec
    │   │   │       invoked.timestamp
    │   │   │       lib-zerovec
    │   │   │       lib-zerovec.json
    │   │   │
    │   │   └───zerovec-derive-fb4c963738a13f95
    │   │           dep-lib-zerovec_derive
    │   │           invoked.timestamp
    │   │           lib-zerovec_derive
    │   │           lib-zerovec_derive.json
    │   │
    │   ├───build
    │   │   ├───aiav_core-05718593555ba341
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-05718593555ba341.d
    │   │   │       build_script_build-05718593555ba341.exe
    │   │   │       build_script_build-05718593555ba341.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───aiav_core-f54d44609f09c590
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───anyhow-19d6502f89ce14f9
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───anyhow-21c2f7832620abf9
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-21c2f7832620abf9.d
    │   │   │       build_script_build-21c2f7832620abf9.exe
    │   │   │       build_script_build-21c2f7832620abf9.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───crossbeam-utils-a5df2f396e625723
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───crossbeam-utils-d634afeca1e68d44
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-d634afeca1e68d44.d
    │   │   │       build_script_build-d634afeca1e68d44.exe
    │   │   │       build_script_build-d634afeca1e68d44.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───generic-array-a0f22693c1710e7e
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───generic-array-ee14a9da454cd5b7
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-ee14a9da454cd5b7.d
    │   │   │       build_script_build-ee14a9da454cd5b7.exe
    │   │   │       build_script_build-ee14a9da454cd5b7.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───getrandom-3203b03998c9f0cd
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-3203b03998c9f0cd.d
    │   │   │       build_script_build-3203b03998c9f0cd.exe
    │   │   │       build_script_build-3203b03998c9f0cd.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───getrandom-6d50dd364829bdb7
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───httparse-40a10619fa24a598
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───httparse-b37724c101282493
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-b37724c101282493.d
    │   │   │       build_script_build-b37724c101282493.exe
    │   │   │       build_script_build-b37724c101282493.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───icu_normalizer_data-153ac4042caa43ce
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───icu_normalizer_data-b4502e787760986a
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-b4502e787760986a.d
    │   │   │       build_script_build-b4502e787760986a.exe
    │   │   │       build_script_build-b4502e787760986a.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───icu_properties_data-5f2d3d09b3bee03d
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───icu_properties_data-ca168c736a483536
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-ca168c736a483536.d
    │   │   │       build_script_build-ca168c736a483536.exe
    │   │   │       build_script_build-ca168c736a483536.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───libc-56cdc4a54f543ac6
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-56cdc4a54f543ac6.d
    │   │   │       build_script_build-56cdc4a54f543ac6.exe
    │   │   │       build_script_build-56cdc4a54f543ac6.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───libc-5fa4bf4c31f0f91d
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───lock_api-56afa19201392373
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───lock_api-5de86c30bdcd6951
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-5de86c30bdcd6951.d
    │   │   │       build_script_build-5de86c30bdcd6951.exe
    │   │   │       build_script_build-5de86c30bdcd6951.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───native-tls-17b597484dc1f060
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───native-tls-64ae2d18b82be3d3
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-64ae2d18b82be3d3.d
    │   │   │       build_script_build-64ae2d18b82be3d3.exe
    │   │   │       build_script_build-64ae2d18b82be3d3.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───num-traits-0019386860619968
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-0019386860619968.d
    │   │   │       build_script_build-0019386860619968.exe
    │   │   │       build_script_build-0019386860619968.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───num-traits-b509cd018bb3b96d
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───parking_lot_core-8f0d24a5b39e99b7
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-8f0d24a5b39e99b7.d
    │   │   │       build_script_build-8f0d24a5b39e99b7.exe
    │   │   │       build_script_build-8f0d24a5b39e99b7.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───parking_lot_core-9ba37945ad05bba4
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───proc-macro2-3b1f514462257677
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───proc-macro2-8ecdc549423c9909
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-8ecdc549423c9909.d
    │   │   │       build_script_build-8ecdc549423c9909.exe
    │   │   │       build_script_build-8ecdc549423c9909.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───rayon-core-991b80ea2d6a82dc
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-991b80ea2d6a82dc.d
    │   │   │       build_script_build-991b80ea2d6a82dc.exe
    │   │   │       build_script_build-991b80ea2d6a82dc.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───rayon-core-f628916875db1b51
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───rustversion-0e6f9310c8b30083
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   │           version.expr
    │   │   │
    │   │   ├───rustversion-15ab6800fc85e5bd
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-15ab6800fc85e5bd.d
    │   │   │       build_script_build-15ab6800fc85e5bd.exe
    │   │   │       build_script_build-15ab6800fc85e5bd.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───serde-738b9b7ceb6d464b
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───serde-f0e00f2ffbef6b75
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-f0e00f2ffbef6b75.d
    │   │   │       build_script_build-f0e00f2ffbef6b75.exe
    │   │   │       build_script_build-f0e00f2ffbef6b75.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───serde_json-1d7dd5643a7e2f74
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-1d7dd5643a7e2f74.d
    │   │   │       build_script_build-1d7dd5643a7e2f74.exe
    │   │   │       build_script_build-1d7dd5643a7e2f74.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───serde_json-d326e683abaf143e
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───thiserror-4396a7df873958dd
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───thiserror-60db7b5760e08792
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───thiserror-7a538730508d99d9
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-7a538730508d99d9.d
    │   │   │       build_script_build-7a538730508d99d9.exe
    │   │   │       build_script_build-7a538730508d99d9.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───thiserror-b6cf77148e1e9b33
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-b6cf77148e1e9b33.d
    │   │   │       build_script_build-b6cf77148e1e9b33.exe
    │   │   │       build_script_build-b6cf77148e1e9b33.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───typenum-4e45f7be503c3a9e
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   │           tests.rs
    │   │   │
    │   │   ├───typenum-d94741fc46dc64f4
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-d94741fc46dc64f4.d
    │   │   │       build_script_build-d94741fc46dc64f4.exe
    │   │   │       build_script_build-d94741fc46dc64f4.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───winapi-69aa28f840d42b71
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───winapi-f996034bfae824c0
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-f996034bfae824c0.d
    │   │   │       build_script_build-f996034bfae824c0.exe
    │   │   │       build_script_build-f996034bfae824c0.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───windows_x86_64_msvc-a13bad8f0db74227
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───windows_x86_64_msvc-a21d0cd880e0be5c
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-a21d0cd880e0be5c.d
    │   │   │       build_script_build-a21d0cd880e0be5c.exe
    │   │   │       build_script_build-a21d0cd880e0be5c.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───windows_x86_64_msvc-e8e607e3652a8546
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───windows_x86_64_msvc-fbba072248a2e0fb
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-fbba072248a2e0fb.d
    │   │   │       build_script_build-fbba072248a2e0fb.exe
    │   │   │       build_script_build-fbba072248a2e0fb.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───yara-sys-082d91747e0e5067
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-082d91747e0e5067.d
    │   │   │       build_script_build-082d91747e0e5067.exe
    │   │   │       build_script_build-082d91747e0e5067.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───yara-sys-21bd3d982b34f2ec
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   │       │   0afd2d690da3ff7a-pe.o
    │   │   │       │   0afd2d690da3ff7a-pe_utils.o
    │   │   │       │   0fd738dfc2f37cca-math.o
    │   │   │       │   373642edb94521a1-string.o
    │   │   │       │   60c750d476bf44a9-tests.o
    │   │   │       │   772ca34c4cf01a3d-windows.o
    │   │   │       │   83bb16aaea92d19b-ahocorasick.o
    │   │   │       │   83bb16aaea92d19b-arena.o
    │   │   │       │   83bb16aaea92d19b-atoms.o
    │   │   │       │   83bb16aaea92d19b-base64.o
    │   │   │       │   83bb16aaea92d19b-bitmask.o
    │   │   │       │   83bb16aaea92d19b-compiler.o
    │   │   │       │   83bb16aaea92d19b-endian.o
    │   │   │       │   83bb16aaea92d19b-exec.o
    │   │   │       │   83bb16aaea92d19b-exefiles.o
    │   │   │       │   83bb16aaea92d19b-filemap.o
    │   │   │       │   83bb16aaea92d19b-grammar.o
    │   │   │       │   83bb16aaea92d19b-hash.o
    │   │   │       │   83bb16aaea92d19b-hex_grammar.o
    │   │   │       │   83bb16aaea92d19b-hex_lexer.o
    │   │   │       │   83bb16aaea92d19b-lexer.o
    │   │   │       │   83bb16aaea92d19b-libyara.o
    │   │   │       │   83bb16aaea92d19b-mem.o
    │   │   │       │   83bb16aaea92d19b-modules.o
    │   │   │       │   83bb16aaea92d19b-notebook.o
    │   │   │       │   83bb16aaea92d19b-object.o
    │   │   │       │   83bb16aaea92d19b-parser.o
    │   │   │       │   83bb16aaea92d19b-proc.o
    │   │   │       │   83bb16aaea92d19b-re.o
    │   │   │       │   83bb16aaea92d19b-re_grammar.o
    │   │   │       │   83bb16aaea92d19b-re_lexer.o
    │   │   │       │   83bb16aaea92d19b-rules.o
    │   │   │       │   83bb16aaea92d19b-scan.o
    │   │   │       │   83bb16aaea92d19b-scanner.o
    │   │   │       │   83bb16aaea92d19b-simple_str.o
    │   │   │       │   83bb16aaea92d19b-sizedstr.o
    │   │   │       │   83bb16aaea92d19b-stack.o
    │   │   │       │   83bb16aaea92d19b-stopwatch.o
    │   │   │       │   83bb16aaea92d19b-stream.o
    │   │   │       │   83bb16aaea92d19b-strutils.o
    │   │   │       │   83bb16aaea92d19b-threading.o
    │   │   │       │   8d96f1adb1d402d7-elf.o
    │   │   │       │   b6872b7363fbd3d2-time.o
    │   │   │       │   bindings.rs
    │   │   │       │   d373f22bc0eb9255-tlsh.o
    │   │   │       │   d373f22bc0eb9255-tlsh_impl.o
    │   │   │       │   d373f22bc0eb9255-tlsh_util.o
    │   │   │       │   d4303d8940dfca0c-console.o
    │   │   │       │   flag_check.c
    │   │   │       │   libyara.a
    │   │   │       │   yara.lib
    │   │   │       │
    │   │   │       └───yara
    │   │   │           │   .bazelrc
    │   │   │           │   .clang-format
    │   │   │           │   .gitattributes
    │   │   │           │   .gitignore
    │   │   │           │   .readthedocs.yaml
    │   │   │           │   appveyor.yml
    │   │   │           │   AUTHORS
    │   │   │           │   bootstrap.sh
    │   │   │           │   BUILD.bazel
    │   │   │           │   build.sh
    │   │   │           │   configure.ac
    │   │   │           │   CONTRIBUTORS
    │   │   │           │   COPYING
    │   │   │           │   Makefile.am
    │   │   │           │   README.md
    │   │   │           │   sample.file
    │   │   │           │   sample.rules
    │   │   │           │   SECURITY.md
    │   │   │           │   WORKSPACE.bazel
    │   │   │           │   yara.man
    │   │   │           │   yara.pc.in
    │   │   │           │   yarac.man
    │   │   │           │
    │   │   │           ├───.github
    │   │   │           │   ├───ISSUE_TEMPLATE
    │   │   │           │   │       bug_report.md
    │   │   │           │   │       feature_request.md
    │   │   │           │   │
    │   │   │           │   └───workflows
    │   │   │           │           build.yml
    │   │   │           │           coverity.yml
    │   │   │           │           oss-fuzz.yml
    │   │   │           │
    │   │   │           ├───bazel
    │   │   │           │       jansson.BUILD
    │   │   │           │       jansson.bzl
    │   │   │           │       magic.BUILD
    │   │   │           │       openssl.BUILD
    │   │   │           │       yara.bzl
    │   │   │           │       yara_deps.bzl
    │   │   │           │
    │   │   │           ├───cli
    │   │   │           │       args.c
    │   │   │           │       args.h
    │   │   │           │       common.c
    │   │   │           │       common.h
    │   │   │           │       threading.c
    │   │   │           │       threading.h
    │   │   │           │       unicode.h
    │   │   │           │       yara.c
    │   │   │           │       yarac.c
    │   │   │           │
    │   │   │           ├───dist
    │   │   │           │       yara-python.spec
    │   │   │           │       yara.spec
    │   │   │           │
    │   │   │           ├───docs
    │   │   │           │   │   capi.rst
    │   │   │           │   │   commandline.rst
    │   │   │           │   │   conf.py
    │   │   │           │   │   docutils.conf
    │   │   │           │   │   gettingstarted.rst
    │   │   │           │   │   index.rst
    │   │   │           │   │   make.bat
    │   │   │           │   │   Makefile
    │   │   │           │   │   modules.rst
    │   │   │           │   │   requirements.txt
    │   │   │           │   │   writingmodules.rst
    │   │   │           │   │   writingrules.rst
    │   │   │           │   │   yarapython.rst
    │   │   │           │   │
    │   │   │           │   └───modules
    │   │   │           │           console.rst
    │   │   │           │           cuckoo.rst
    │   │   │           │           dotnet.rst
    │   │   │           │           elf.rst
    │   │   │           │           hash.rst
    │   │   │           │           magic.rst
    │   │   │           │           math.rst
    │   │   │           │           pe.rst
    │   │   │           │           string.rst
    │   │   │           │           time.rst
    │   │   │           │
    │   │   │           ├───extra
    │   │   │           │   │   logo.ai
    │   │   │           │   │   logo.svg
    │   │   │           │   │   old-logo.png
    │   │   │           │   │   old-logo.psd
    │   │   │           │   │   UltraEdit-wordfile.txt
    │   │   │           │   │
    │   │   │           │   └───codemirror
    │   │   │           │           index.html
    │   │   │           │           yara.js
    │   │   │           │
    │   │   │           ├───libyara
    │   │   │           │   │   ahocorasick.c
    │   │   │           │   │   arena.c
    │   │   │           │   │   atoms.c
    │   │   │           │   │   base64.c
    │   │   │           │   │   bitmask.c
    │   │   │           │   │   compiler.c
    │   │   │           │   │   crypto.h
    │   │   │           │   │   endian.c
    │   │   │           │   │   exception.h
    │   │   │           │   │   exec.c
    │   │   │           │   │   exefiles.c
    │   │   │           │   │   filemap.c
    │   │   │           │   │   grammar.c
    │   │   │           │   │   grammar.h
    │   │   │           │   │   grammar.y
    │   │   │           │   │   hash.c
    │   │   │           │   │   hex_grammar.c
    │   │   │           │   │   hex_grammar.h
    │   │   │           │   │   hex_grammar.y
    │   │   │           │   │   hex_lexer.c
    │   │   │           │   │   hex_lexer.l
    │   │   │           │   │   lexer.c
    │   │   │           │   │   lexer.l
    │   │   │           │   │   libyara.c
    │   │   │           │   │   mem.c
    │   │   │           │   │   modules.c
    │   │   │           │   │   notebook.c
    │   │   │           │   │   object.c
    │   │   │           │   │   parser.c
    │   │   │           │   │   proc.c
    │   │   │           │   │   re.c
    │   │   │           │   │   re_grammar.c
    │   │   │           │   │   re_grammar.h
    │   │   │           │   │   re_grammar.y
    │   │   │           │   │   re_lexer.c
    │   │   │           │   │   re_lexer.l
    │   │   │           │   │   rules.c
    │   │   │           │   │   scan.c
    │   │   │           │   │   scanner.c
    │   │   │           │   │   simple_str.c
    │   │   │           │   │   sizedstr.c
    │   │   │           │   │   stack.c
    │   │   │           │   │   stino.settings
    │   │   │           │   │   stopwatch.c
    │   │   │           │   │   stream.c
    │   │   │           │   │   strutils.c
    │   │   │           │   │   threading.c
    │   │   │           │   │
    │   │   │           │   ├───include
    │   │   │           │   │   │   yara.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───authenticode-parser
    │   │   │           │   │   │       authenticode.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───tlshc
    │   │   │           │   │   │       tlsh.h
    │   │   │           │   │   │
    │   │   │           │   │   └───yara
    │   │   │           │   │           ahocorasick.h
    │   │   │           │   │           arena.h
    │   │   │           │   │           atoms.h
    │   │   │           │   │           base64.h
    │   │   │           │   │           bitmask.h
    │   │   │           │   │           compiler.h
    │   │   │           │   │           dex.h
    │   │   │           │   │           dotnet.h
    │   │   │           │   │           elf.h
    │   │   │           │   │           elf_utils.h
    │   │   │           │   │           endian.h
    │   │   │           │   │           error.h
    │   │   │           │   │           exec.h
    │   │   │           │   │           exefiles.h
    │   │   │           │   │           filemap.h
    │   │   │           │   │           globals.h
    │   │   │           │   │           hash.h
    │   │   │           │   │           hex_lexer.h
    │   │   │           │   │           integers.h
    │   │   │           │   │           lexer.h
    │   │   │           │   │           libyara.h
    │   │   │           │   │           limits.h
    │   │   │           │   │           macho.h
    │   │   │           │   │           mem.h
    │   │   │           │   │           modules.h
    │   │   │           │   │           notebook.h
    │   │   │           │   │           object.h
    │   │   │           │   │           parser.h
    │   │   │           │   │           pe.h
    │   │   │           │   │           pe_utils.h
    │   │   │           │   │           proc.h
    │   │   │           │   │           re.h
    │   │   │           │   │           re_lexer.h
    │   │   │           │   │           rules.h
    │   │   │           │   │           scan.h
    │   │   │           │   │           scanner.h
    │   │   │           │   │           simple_str.h
    │   │   │           │   │           sizedstr.h
    │   │   │           │   │           stack.h
    │   │   │           │   │           stopwatch.h
    │   │   │           │   │           stream.h
    │   │   │           │   │           strutils.h
    │   │   │           │   │           threading.h
    │   │   │           │   │           types.h
    │   │   │           │   │           unaligned.h
    │   │   │           │   │           utils.h
    │   │   │           │   │
    │   │   │           │   ├───modules
    │   │   │           │   │   │   module_list
    │   │   │           │   │   │   pb_to_module.rst
    │   │   │           │   │   │
    │   │   │           │   │   ├───console
    │   │   │           │   │   │       console.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───cuckoo
    │   │   │           │   │   │       cuckoo.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───demo
    │   │   │           │   │   │       demo.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───dex
    │   │   │           │   │   │       dex.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───dotnet
    │   │   │           │   │   │       dotnet.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───elf
    │   │   │           │   │   │       elf.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───hash
    │   │   │           │   │   │       hash.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───macho
    │   │   │           │   │   │       macho.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───magic
    │   │   │           │   │   │       magic.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───math
    │   │   │           │   │   │       math.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───pb_tests
    │   │   │           │   │   │       pb_tests.c
    │   │   │           │   │   │       pb_tests.pb-c.c
    │   │   │           │   │   │       pb_tests.pb-c.h
    │   │   │           │   │   │       pb_tests.proto
    │   │   │           │   │   │       yara.pb-c.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───pe
    │   │   │           │   │   │   │   pe.c
    │   │   │           │   │   │   │   pe_utils.c
    │   │   │           │   │   │   │
    │   │   │           │   │   │   └───authenticode-parser
    │   │   │           │   │   │           authenticode.c
    │   │   │           │   │   │           certificate.c
    │   │   │           │   │   │           certificate.h
    │   │   │           │   │   │           countersignature.c
    │   │   │           │   │   │           countersignature.h
    │   │   │           │   │   │           helper.c
    │   │   │           │   │   │           helper.h
    │   │   │           │   │   │           structs.c
    │   │   │           │   │   │           structs.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───string
    │   │   │           │   │   │       string.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───tests
    │   │   │           │   │   │       tests.c
    │   │   │           │   │   │
    │   │   │           │   │   └───time
    │   │   │           │   │           time.c
    │   │   │           │   │
    │   │   │           │   ├───pb
    │   │   │           │   │       yara.proto
    │   │   │           │   │
    │   │   │           │   ├───proc
    │   │   │           │   │       freebsd.c
    │   │   │           │   │       linux.c
    │   │   │           │   │       mach.c
    │   │   │           │   │       none.c
    │   │   │           │   │       openbsd.c
    │   │   │           │   │       windows.c
    │   │   │           │   │
    │   │   │           │   └───tlshc
    │   │   │           │           tlsh.c
    │   │   │           │           tlsh_impl.c
    │   │   │           │           tlsh_impl.h
    │   │   │           │           tlsh_util.c
    │   │   │           │           tlsh_util.h
    │   │   │           │
    │   │   │           ├───m4
    │   │   │           │       acx_pthread.m4
    │   │   │           │
    │   │   │           ├───sandbox
    │   │   │           │       BUILD.bazel
    │   │   │           │       collect_matches.cc
    │   │   │           │       collect_matches.h
    │   │   │           │       sandboxed_yara.cc
    │   │   │           │       yara_entry_points.cc
    │   │   │           │       yara_matches.proto
    │   │   │           │       yara_transaction.cc
    │   │   │           │       yara_transaction.h
    │   │   │           │       yara_transaction_test.cc
    │   │   │           │
    │   │   │           ├───tests
    │   │   │           │   │   blob.h
    │   │   │           │   │   BUILD.bazel
    │   │   │           │   │   convention-portable-modifiers
    │   │   │           │   │   gcov-summary
    │   │   │           │   │   mapper.c
    │   │   │           │   │   test-alignment.c
    │   │   │           │   │   test-api.c
    │   │   │           │   │   test-arena.c
    │   │   │           │   │   test-async.c
    │   │   │           │   │   test-atoms.c
    │   │   │           │   │   test-bitmask.c
    │   │   │           │   │   test-dex.c
    │   │   │           │   │   test-dotnet.c
    │   │   │           │   │   test-elf.c
    │   │   │           │   │   test-exception.c
    │   │   │           │   │   test-macho.c
    │   │   │           │   │   test-magic.c
    │   │   │           │   │   test-math.c
    │   │   │           │   │   test-pb.c
    │   │   │           │   │   test-pe.c
    │   │   │           │   │   test-re-split.c
    │   │   │           │   │   test-rules.c
    │   │   │           │   │   test-stack.c
    │   │   │           │   │   test-string.c
    │   │   │           │   │   test-version.c
    │   │   │           │   │   util.c
    │   │   │           │   │   util.h
    │   │   │           │   │
    │   │   │           │   ├───data
    │   │   │           │   │   │   05cd06e6a202e12be22a02700ed6f1604e803ca8867277d852e8971efded0650
    │   │   │           │   │   │   079a472d22290a94ebb212aa8015cdc8dd28a968c6b4d3b88acdd58ce2d3b885
    │   │   │           │   │   │   079a472d22290a94ebb212aa8015cdc8dd28a968c6b4d3b88acdd58ce2d3b885.upx
    │   │   │           │   │   │   0ca09bde7602769120fadc4f7a4147347a7a97271370583586c9e587fd396171
    │   │   │           │   │   │   33fc70f99be6d2833ae48852d611c8048d0c053ed0b2c626db4dbe902832a08b
    │   │   │           │   │   │   3b8b90159fa9b6048cc5410c5d53f116943564e4d05b04a843f9b3d0540d0c1c
    │   │   │           │   │   │   6c2abf4b80a87e63eee2996e5cea8f004d49ec0c1806080fa72e960529cba14c
    │   │   │           │   │   │   756684f4017ba7e931a26724ae61606b16b5f8cc84ed38a260a34e50c5016f59
    │   │   │           │   │   │   bad_dotnet_pe
    │   │   │           │   │   │   base64
    │   │   │           │   │   │   baz.yar
    │   │   │           │   │   │   c6f9709feccf42f2d9e22057182fe185f177fb9daaa2649b4669a24f2ee7e3ba_0h_410h
    │   │   │           │   │   │   ca21e1c32065352d352be6cde97f89c141d7737ea92434831f998080783d5386
    │   │   │           │   │   │   ChipTune.efi
    │   │   │           │   │   │   e3d45a2865818756068757d7e319258fef40dad54532ee4355b86bc129f27345
    │   │   │           │   │   │   elf_with_imports
    │   │   │           │   │   │   foo.yar
    │   │   │           │   │   │   mtxex.dll
    │   │   │           │   │   │   mtxex_modified_rsrc_rva.dll
    │   │   │           │   │   │   pe_imports
    │   │   │           │   │   │   pe_mingw
    │   │   │           │   │   │   test-pb.data
    │   │   │           │   │   │   test-pb.data.bin
    │   │   │           │   │   │   tiny
    │   │   │           │   │   │   tiny-idata-51ff
    │   │   │           │   │   │   tiny-idata-5200
    │   │   │           │   │   │   tiny-macho
    │   │   │           │   │   │   tiny-overlay
    │   │   │           │   │   │   tiny-universal
    │   │   │           │   │   │   tiny.notes
    │   │   │           │   │   │   tiny_empty_import_name
    │   │   │           │   │   │   weird_rich
    │   │   │           │   │   │   x.txt
    │   │   │           │   │   │   xor.out
    │   │   │           │   │   │   xorwide.out
    │   │   │           │   │   │   xorwideandascii.out
    │   │   │           │   │   │
    │   │   │           │   │   └───include
    │   │   │           │   │           bar.yar
    │   │   │           │   │
    │   │   │           │   └───oss-fuzz
    │   │   │           │       │   dex_fuzzer.cc
    │   │   │           │       │   dotnet_fuzzer.cc
    │   │   │           │       │   elf_fuzzer.cc
    │   │   │           │       │   macho_fuzzer.cc
    │   │   │           │       │   pe_fuzzer.cc
    │   │   │           │       │   rules_fuzzer.cc
    │   │   │           │       │   rules_fuzzer.dict
    │   │   │           │       │   rules_fuzzer.options
    │   │   │           │       │
    │   │   │           │       ├───dex_fuzzer_corpus
    │   │   │           │       │       1cf540db2f048bb21bd89379a57279b9ff4c308558715a3baee666a47393d86e
    │   │   │           │       │       25ef27f9543444652f0c68fe412d3da627a1d2a590b0a2b30e47466c1e962136
    │   │   │           │       │       27fb31059503773723597edb875c937af971a6c15f91aac8c03c1fbdfa9e918c
    │   │   │           │       │       3ba9c082050f62e725c87ce4cf9f592fe9f177faf3a0c879f8fbe87312ca4b2c
    │   │   │           │       │       b1203d95c56f02e7e6dbea714275cc05b47ac2510958b85f436571b801af44e7
    │   │   │           │       │       b343d1058063e6e4b652ccf0589f93d0dbb6b092960e4aebc3c3c58894831359
    │   │   │           │       │       crash.poc
    │   │   │           │       │
    │   │   │           │       ├───dotnet_fuzzer_corpus
    │   │   │           │       │       buggy_stream_names
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5105966966636544
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5195285818507264
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5636481138556928
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5725060321509376
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5880393521430528
    │   │   │           │       │       obfuscated
    │   │   │           │       │
    │   │   │           │       ├───elf_fuzzer_corpus
    │   │   │           │       │       crash-03bca75466ee42801a8bff280de04afc3d1a3637
    │   │   │           │       │       crash-086300bbce1c6537573057336a343a82d483e2c0
    │   │   │           │       │       crash-2cafe4de66d87a83d83aaf65d8e4cea48f2c1144
    │   │   │           │       │       crash-370485c5b087f780a2447a03d775f7188e323d31
    │   │   │           │       │       crash-49bb55d669fda0683f945b89396a6bd458caf2d8
    │   │   │           │       │       crash-49d00b6b033eaeb07cd39809dbc1d7ba2df196ec
    │   │   │           │       │       crash-723296cdc1c0dba83ea767d69286429e608c46c3
    │   │   │           │       │       crash-7dc27920ae1cb85333e7f2735a45014488134673
    │   │   │           │       │       crash-7e945ce5f43f515ea078c558a2e3205089d414e5
    │   │   │           │       │       crash-a809561e75b94bd5d4d8cf7488d9e2663fc1ccdc
    │   │   │           │       │       crash-a8715a38a94161c9509309f5dbb5a7936aba8376
    │   │   │           │       │       crash-aee928239444a7b039500d4499035e6d30cb89da
    │   │   │           │       │       crash-c4002396c52065d21fe1c1f05f8937aab8d59c18
    │   │   │           │       │       crash-c610b3036f195ad7fb05248a530278aad37b438d
    │   │   │           │       │       crash-c6569e6e28f0a18bb2f3bf49c982333a359bed67
    │   │   │           │       │       crash-cc6844f44825a785de1b079c88f728e1c0f779fb
    │   │   │           │       │       crash-f1fd008da535b110853885221ebfaac3f262a1c1e280f10929f7b353c44996c8
    │   │   │           │       │       poc-6bf54fca69bb5029676d747b12c74b597dd8c5939343ea8f2cbfea9e666dd6b1
    │   │   │           │       │       poc-789fc6da83de39c3ff394a950b0831f6fe5b63a85a46aaa236048b5c1dcf0e59
    │   │   │           │       │       poc-939e9cd87b0d80834210fbf54edc66341aebf416d7509f6633f1d49766978b22
    │   │   │           │       │       poc-93a9fd1909dd49fc2a9b654333504f249cdac58126d3cfc4728577e78cb3eb89
    │   │   │           │       │       poc-b5b03a1f305b2cc1c158e01fee6c08c65145325d4e073f04d969329577077862
    │   │   │           │       │       poc-fa8bbacb5a12f057a0ed3999c37d78b4991e6b201bda4dc9a75a7c7970c7690d
    │   │   │           │       │
    │   │   │           │       ├───macho_fuzzer_corpus
    │   │   │           │       │       1443c3cfb47c5eb41022a7063c24ab1bc9e45bfc31e98d5e6d3aa8377599b983
    │   │   │           │       │       589f7b0e30d885ed91229646e58ccc7615007d2fab06451fef8785c6126adba7
    │   │   │           │       │       5eefacbe52990526e4953802249447dd8c0a4b537459ca41e005a7173ca46138
    │   │   │           │       │       6164a837fd33574f37464a765ab461fff94b52e659b114fb6109f2635678c564
    │   │   │           │       │       66528aeb35dd705cc26a7daf4b8eda684f620efebfa0740fab84043e371ed566
    │   │   │           │       │       678d89b32eecd7d01390aaaf3507935b27854f4f3a7055e3f6b1b0ccf0ca5072
    │   │   │           │       │       6af5d157184d9144f86668f83e81760898df5db3c9e209596eb5fd9a91a7eeba
    │   │   │           │       │       797d1d450421b771482c0cc03f472e4eccbc9e4f544b6c12c1d4f070dec3c381
    │   │   │           │       │       85494d8cb5753f1ad09be39428135feb35eb4ef44f39d6e1e75e2ad30d93e158
    │   │   │           │       │       b225048e85b14f08a43dd4752b9bb4b20840f5a8726eac0ff765d45c9e619828
    │   │   │           │       │       fda81421d7403180923717a94e77aade8c9286d5b8de3ae0e2812343b666c6a7
    │   │   │           │       │
    │   │   │           │       ├───pe_fuzzer_corpus
    │   │   │           │       │       00388b550a2603a9e219bcb48acaf8cc115653cb1ea84cb4bccceb1aabe755b6
    │   │   │           │       │       12f50a7dbf0c42f61ae1c351b2a9f75e8edb3bb55e582619edc7ece4eb0a3094
    │   │   │           │       │       967af267b4124bada8f507cebf25f2192d146a4d63be71b45bfc03c5da7f21a7
    │   │   │           │       │       99e98cb7096dee974e28fea0f76f1c30bc44fd5762cb12b2702910a28b28f95f
    │   │   │           │       │       clusterfuzz-testcase-minimized-5211130361282560
    │   │   │           │       │       clusterfuzz-testcase-minimized-5839717883969536
    │   │   │           │       │       clusterfuzz-testcase-minimized-pe_fuzzer-5671228022718464
    │   │   │           │       │       clusterfuzz-testcase-minimized-pe_fuzzer-5741846293643264
    │   │   │           │       │       e5af0352010b1879ac1c63a69d3d9a02d577fa834165f855bd5ebee0f1105de1
    │   │   │           │       │
    │   │   │           │       └───rules_fuzzer_corpus
    │   │   │           │               1
    │   │   │           │               2
    │   │   │           │               3
    │   │   │           │               4
    │   │   │           │               5
    │   │   │           │               6
    │   │   │           │               7
    │   │   │           │               8
    │   │   │           │
    │   │   │           └───windows
    │   │   │               ├───vs2015
    │   │   │               │   │   NuGet.Config
    │   │   │               │   │   yara.sln
    │   │   │               │   │
    │   │   │               │   ├───libyara
    │   │   │               │   │       libyara.vcxproj
    │   │   │               │   │       packages.config
    │   │   │               │   │
    │   │   │               │   ├───test-alignment
    │   │   │               │   │       test-alignment.vcxproj
    │   │   │               │   │
    │   │   │               │   ├───yara
    │   │   │               │   │       yara.vcxproj
    │   │   │               │   │
    │   │   │               │   └───yarac
    │   │   │               │           yarac.vcxproj
    │   │   │               │
    │   │   │               ├───vs2017
    │   │   │               │   │   NuGet.Config
    │   │   │               │   │   yara.sln
    │   │   │               │   │
    │   │   │               │   ├───libyara
    │   │   │               │   │       libyara.vcxproj
    │   │   │               │   │       libyara.vcxproj.user
    │   │   │               │   │       packages.config
    │   │   │               │   │
    │   │   │               │   ├───yara
    │   │   │               │   │       yara.vcxproj
    │   │   │               │   │       yara.vcxproj.user
    │   │   │               │   │
    │   │   │               │   └───yarac
    │   │   │               │           yarac.vcxproj
    │   │   │               │           yarac.vcxproj.user
    │   │   │               │
    │   │   │               └───vs2019
    │   │   │                   │   NuGet.Config
    │   │   │                   │   yara.sln
    │   │   │                   │
    │   │   │                   ├───libyara
    │   │   │                   │       libyara.vcxproj
    │   │   │                   │       libyara.vcxproj.user
    │   │   │                   │       packages.config
    │   │   │                   │
    │   │   │                   ├───yara
    │   │   │                   │       yara.vcxproj
    │   │   │                   │       yara.vcxproj.user
    │   │   │                   │
    │   │   │                   └───yarac
    │   │   │                           yarac.vcxproj
    │   │   │                           yarac.vcxproj.user
    │   │   │
    │   │   ├───zerocopy-191912f0e0e75127
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   └───zerocopy-dbd6279178c04a94
    │   │           build-script-build.exe
    │   │           build_script_build-dbd6279178c04a94.d
    │   │           build_script_build-dbd6279178c04a94.exe
    │   │           build_script_build-dbd6279178c04a94.pdb
    │   │           build_script_build.pdb
    │   │
    │   ├───deps
    │   │       adler2-7e7adc8515193442.d
    │   │       aiav_core-1a4dd919b5b27738.d
    │   │       aiav_core-3ef2a037aacee29a.d
    │   │       aiav_core-99f5eb28e30f0f30.d
    │   │       aiav_core-a512489562778412.d
    │   │       anyhow-b2beece60c91017e.d
    │   │       async_compression-ffde80ecf977ba8b.d
    │   │       async_trait-cd3dc4308a91e16d.d
    │   │       async_trait-cd3dc4308a91e16d.dll
    │   │       async_trait-cd3dc4308a91e16d.dll.exp
    │   │       async_trait-cd3dc4308a91e16d.dll.lib
    │   │       async_trait-cd3dc4308a91e16d.pdb
    │   │       atomic_waker-01c8d4dd37710705.d
    │   │       autocfg-1fa2b1e7854770da.d
    │   │       axum-fb01155dca0787b7.d
    │   │       axum_core-620785971f88f428.d
    │   │       axum_macros-efa3d7794c485712.d
    │   │       axum_macros-efa3d7794c485712.dll
    │   │       axum_macros-efa3d7794c485712.dll.exp
    │   │       axum_macros-efa3d7794c485712.dll.lib
    │   │       axum_macros-efa3d7794c485712.pdb
    │   │       base64-60dbf2162231c154.d
    │   │       bitflags-27e7531dabfdfeb8.d
    │   │       block_buffer-c10c457849c4983f.d
    │   │       bytes-6f1a6f12e445e6cc.d
    │   │       cc-0e574b4d15c526bd.d
    │   │       cfg_if-b4957d577da748fa.d
    │   │       chrono-35ce3019cab68291.d
    │   │       cloud_services-494d90562e88a3a9.d
    │   │       cloud_services-c86445f794665994.d
    │   │       cpufeatures-a51bb6dd08a30a0e.d
    │   │       crc32fast-2ab078f62f7ab397.d
    │   │       crossbeam_channel-33509ab0739dffe9.d
    │   │       crossbeam_deque-e81f03f549463d17.d
    │   │       crossbeam_epoch-460701beabd9777f.d
    │   │       crossbeam_utils-e2b0808d7545bafd.d
    │   │       crypto_common-1b3d3d2408c8effc.d
    │   │       data_encoding-a1b3bdd44b98cca5.d
    │   │       deranged-2555b3a184920c70.d
    │   │       digest-0d4c1d24b6218e69.d
    │   │       displaydoc-79cc6545e2314b85.d
    │   │       displaydoc-79cc6545e2314b85.dll
    │   │       displaydoc-79cc6545e2314b85.dll.exp
    │   │       displaydoc-79cc6545e2314b85.dll.lib
    │   │       displaydoc-79cc6545e2314b85.pdb
    │   │       either-ae6dbdca34a9d337.d
    │   │       encoding_rs-1e470f2ee6d25279.d
    │   │       equivalent-ba0c225f2f332fde.d
    │   │       fastrand-0242012a3cc1e8c6.d
    │   │       filetime-2d2932ed99e3866a.d
    │   │       flate2-54da36c851e633e7.d
    │   │       fnv-3d8ef674e6ff6ec4.d
    │   │       form_urlencoded-98fc11844794d987.d
    │   │       fs_extra-00dfc66abecc69f7.d
    │   │       fs_extra-5c04533ce8beb64f.d
    │   │       futures-e0f98eb3474ddadd.d
    │   │       futures_channel-0aa17cf82779b64f.d
    │   │       futures_core-10b6e226dff9bf0e.d
    │   │       futures_executor-cd33e983e42d3e23.d
    │   │       futures_io-07071dc0a1ce2b00.d
    │   │       futures_macro-aeb6aaca5024ef59.d
    │   │       futures_macro-aeb6aaca5024ef59.dll
    │   │       futures_macro-aeb6aaca5024ef59.dll.exp
    │   │       futures_macro-aeb6aaca5024ef59.dll.lib
    │   │       futures_macro-aeb6aaca5024ef59.pdb
    │   │       futures_sink-d328231113ed7cc6.d
    │   │       futures_task-e9ec59bd49a29dbb.d
    │   │       futures_util-e1a3116177e9177d.d
    │   │       generic_array-fd60979d0bc4cf39.d
    │   │       getrandom-c2585604dd79c6e6.d
    │   │       glob-a833c8eeb3b37e3b.d
    │   │       h2-5856b2c800458c51.d
    │   │       hashbrown-6d6277d712f36016.d
    │   │       http-194c4970bf8d03e5.d
    │   │       httparse-b4fb1babb990ded4.d
    │   │       httpdate-3aca05d1c52163aa.d
    │   │       http_body-a699eb9b3b1d78fc.d
    │   │       http_body_util-36f2f55f01df4d52.d
    │   │       hyper-69e47b41336db0bb.d
    │   │       hyper_tls-3c5fab58960deab5.d
    │   │       hyper_util-ca28f1e207e9405f.d
    │   │       icu_collections-f28effc508a5242d.d
    │   │       icu_locale_core-76f927c3c8ca1c31.d
    │   │       icu_normalizer-31e3f3b8f4713f83.d
    │   │       icu_normalizer_data-99b7b753be955813.d
    │   │       icu_properties-b8d8922db1647abb.d
    │   │       icu_properties_data-9c14d4744353a58e.d
    │   │       icu_provider-50717ef4a4743ecf.d
    │   │       idna-fd2a2901cc42a5d3.d
    │   │       idna_adapter-3b6fac3f49374e90.d
    │   │       indexmap-4b56ca83916d7ae7.d
    │   │       ipnet-b7f33d3ead5d12fb.d
    │   │       iri_string-72b78bf5585c7f08.d
    │   │       itoa-69297a500bb545a7.d
    │   │       lazy_static-a00a19492b117dcc.d
    │   │       libadler2-7e7adc8515193442.rmeta
    │   │       libaiav_core-1a4dd919b5b27738.rmeta
    │   │       libaiav_core-3ef2a037aacee29a.rmeta
    │   │       libaiav_core-99f5eb28e30f0f30.rmeta
    │   │       libaiav_core-a512489562778412.rmeta
    │   │       libanyhow-b2beece60c91017e.rmeta
    │   │       libasync_compression-ffde80ecf977ba8b.rmeta
    │   │       libatomic_waker-01c8d4dd37710705.rmeta
    │   │       libautocfg-1fa2b1e7854770da.rlib
    │   │       libautocfg-1fa2b1e7854770da.rmeta
    │   │       libaxum-fb01155dca0787b7.rmeta
    │   │       libaxum_core-620785971f88f428.rmeta
    │   │       libbase64-60dbf2162231c154.rmeta
    │   │       libbitflags-27e7531dabfdfeb8.rmeta
    │   │       libblock_buffer-c10c457849c4983f.rmeta
    │   │       libbytes-6f1a6f12e445e6cc.rmeta
    │   │       libc-bb560bf5cffc4727.d
    │   │       libcc-0e574b4d15c526bd.rlib
    │   │       libcc-0e574b4d15c526bd.rmeta
    │   │       libcfg_if-b4957d577da748fa.rmeta
    │   │       libchrono-35ce3019cab68291.rmeta
    │   │       libcloud_services-494d90562e88a3a9.rmeta
    │   │       libcloud_services-c86445f794665994.rmeta
    │   │       libcpufeatures-a51bb6dd08a30a0e.rmeta
    │   │       libcrc32fast-2ab078f62f7ab397.rmeta
    │   │       libcrossbeam_channel-33509ab0739dffe9.rmeta
    │   │       libcrossbeam_deque-e81f03f549463d17.rmeta
    │   │       libcrossbeam_epoch-460701beabd9777f.rmeta
    │   │       libcrossbeam_utils-e2b0808d7545bafd.rmeta
    │   │       libcrypto_common-1b3d3d2408c8effc.rmeta
    │   │       libdata_encoding-a1b3bdd44b98cca5.rmeta
    │   │       libderanged-2555b3a184920c70.rmeta
    │   │       libdigest-0d4c1d24b6218e69.rmeta
    │   │       libeither-ae6dbdca34a9d337.rmeta
    │   │       libencoding_rs-1e470f2ee6d25279.rmeta
    │   │       libequivalent-ba0c225f2f332fde.rmeta
    │   │       libfastrand-0242012a3cc1e8c6.rmeta
    │   │       libfiletime-2d2932ed99e3866a.rmeta
    │   │       libflate2-54da36c851e633e7.rmeta
    │   │       libfnv-3d8ef674e6ff6ec4.rmeta
    │   │       libform_urlencoded-98fc11844794d987.rmeta
    │   │       libfs_extra-00dfc66abecc69f7.rmeta
    │   │       libfs_extra-5c04533ce8beb64f.rlib
    │   │       libfs_extra-5c04533ce8beb64f.rmeta
    │   │       libfutures-e0f98eb3474ddadd.rmeta
    │   │       libfutures_channel-0aa17cf82779b64f.rmeta
    │   │       libfutures_core-10b6e226dff9bf0e.rmeta
    │   │       libfutures_executor-cd33e983e42d3e23.rmeta
    │   │       libfutures_io-07071dc0a1ce2b00.rmeta
    │   │       libfutures_sink-d328231113ed7cc6.rmeta
    │   │       libfutures_task-e9ec59bd49a29dbb.rmeta
    │   │       libfutures_util-e1a3116177e9177d.rmeta
    │   │       libgeneric_array-fd60979d0bc4cf39.rmeta
    │   │       libgetrandom-c2585604dd79c6e6.rmeta
    │   │       libglob-a833c8eeb3b37e3b.rlib
    │   │       libglob-a833c8eeb3b37e3b.rmeta
    │   │       libh2-5856b2c800458c51.rmeta
    │   │       libhashbrown-6d6277d712f36016.rmeta
    │   │       libhttp-194c4970bf8d03e5.rmeta
    │   │       libhttparse-b4fb1babb990ded4.rmeta
    │   │       libhttpdate-3aca05d1c52163aa.rmeta
    │   │       libhttp_body-a699eb9b3b1d78fc.rmeta
    │   │       libhttp_body_util-36f2f55f01df4d52.rmeta
    │   │       libhyper-69e47b41336db0bb.rmeta
    │   │       libhyper_tls-3c5fab58960deab5.rmeta
    │   │       libhyper_util-ca28f1e207e9405f.rmeta
    │   │       libicu_collections-f28effc508a5242d.rmeta
    │   │       libicu_locale_core-76f927c3c8ca1c31.rmeta
    │   │       libicu_normalizer-31e3f3b8f4713f83.rmeta
    │   │       libicu_normalizer_data-99b7b753be955813.rmeta
    │   │       libicu_properties-b8d8922db1647abb.rmeta
    │   │       libicu_properties_data-9c14d4744353a58e.rmeta
    │   │       libicu_provider-50717ef4a4743ecf.rmeta
    │   │       libidna-fd2a2901cc42a5d3.rmeta
    │   │       libidna_adapter-3b6fac3f49374e90.rmeta
    │   │       libindexmap-4b56ca83916d7ae7.rmeta
    │   │       libipnet-b7f33d3ead5d12fb.rmeta
    │   │       libiri_string-72b78bf5585c7f08.rmeta
    │   │       libitoa-69297a500bb545a7.rmeta
    │   │       liblazy_static-a00a19492b117dcc.rmeta
    │   │       liblibc-bb560bf5cffc4727.rmeta
    │   │       liblitemap-99aba40979849771.rmeta
    │   │       liblock_api-e9d8b5157e526545.rmeta
    │   │       liblog-4fd96ec37443cbba.rmeta
    │   │       libmatchers-98113fb8bc145d9f.rmeta
    │   │       libmatchit-8e5c37df1ea8ce26.rmeta
    │   │       libmemchr-d21ed81bcc5c2a12.rmeta
    │   │       libmime-f51d29a9076e2fef.rmeta
    │   │       libminiz_oxide-5de3e2f572df22da.rmeta
    │   │       libmio-b74fc8fbf29e9adb.rmeta
    │   │       libnative_tls-91f085e89949adf0.rmeta
    │   │       libnotify-7514b83b65fb1344.rmeta
    │   │       libnotify-9511bfd2115a3c48.rmeta
    │   │       libnotify_types-cf90db815a4b4217.rmeta
    │   │       libnum_conv-55ec824b6f8f9351.rmeta
    │   │       libnum_traits-8c538d1c0fb7c5a1.rmeta
    │   │       libnu_ansi_term-e57b887bb81d1805.rmeta
    │   │       libonce_cell-02d9b1423b4c3a99.rmeta
    │   │       liboverload-39bd931dd46f4fb1.rmeta
    │   │       libparking_lot-5320cbe173ddeca5.rmeta
    │   │       libparking_lot_core-87879622a5efa30a.rmeta
    │   │       libpercent_encoding-226c9e1a82702448.rmeta
    │   │       libpin_project_lite-360f5ac79d6490a8.rmeta
    │   │       libpin_utils-6131d7e882b3d82e.rmeta
    │   │       libpotential_utf-46997aa979ed06de.rmeta
    │   │       libpowerfmt-19f2ce97eac604ff.rmeta
    │   │       libppv_lite86-968964c6cb7b2a5c.rmeta
    │   │       libproc_macro2-6a20e463e4a4ca20.rlib
    │   │       libproc_macro2-6a20e463e4a4ca20.rmeta
    │   │       libquote-fdeee9df46234d2d.rlib
    │   │       libquote-fdeee9df46234d2d.rmeta
    │   │       librand-1359bed459c03497.rmeta
    │   │       librand_chacha-6d2e0a3d1d0cf058.rmeta
    │   │       librand_core-9dcb78c9132a4656.rmeta
    │   │       librayon-dfab54025ddc28fc.rmeta
    │   │       librayon_core-58356cac7752356c.rmeta
    │   │       libregex-7126983b5ef91054.rmeta
    │   │       libregex_automata-91efb59fcc2b2c23.rmeta
    │   │       libregex_automata-ae3084067eb369ab.rmeta
    │   │       libregex_syntax-2ddbb26afe1c0a64.rmeta
    │   │       libregex_syntax-bfd7e1ff807f1064.rmeta
    │   │       libreqwest-a31f51c3b0b65d42.rmeta
    │   │       librustls_pki_types-d9f71a2bd6fc478f.rmeta
    │   │       libryu-3442447a74614b27.rmeta
    │   │       libsame_file-2da6eccfbf8519af.rmeta
    │   │       libschannel-8868a50aa66715dc.rmeta
    │   │       libscopeguard-66a9ca5b98e4d6bf.rmeta
    │   │       libserde-e2c5fc6a4f673cbb.rmeta
    │   │       libserde_json-4f4830174827aea7.rmeta
    │   │       libserde_path_to_error-bc040f3aaac335b7.rmeta
    │   │       libserde_spanned-070ad526d1db460c.rmeta
    │   │       libserde_urlencoded-980a8cf95a86cfca.rmeta
    │   │       libsha1-4483d9d32339807e.rmeta
    │   │       libsha2-64b0ec1eee2551f5.rmeta
    │   │       libsharded_slab-74a2b9b853e8fc13.rmeta
    │   │       libshlex-27ba53e6f2866fee.rlib
    │   │       libshlex-27ba53e6f2866fee.rmeta
    │   │       libslab-6403e07a840fb0da.rmeta
    │   │       libsmallvec-fd9fc186d6d6180f.rmeta
    │   │       libsocket2-7074eedfe5b08109.rmeta
    │   │       libstable_deref_trait-a5623622a362b8d6.rmeta
    │   │       libsyn-a3600ff6e0b8e983.rlib
    │   │       libsyn-a3600ff6e0b8e983.rmeta
    │   │       libsync_wrapper-b8ddcd24d18460c7.rmeta
    │   │       libsynstructure-75469bf036e5ef30.rlib
    │   │       libsynstructure-75469bf036e5ef30.rmeta
    │   │       libsystem_integration-0d7081e5500ceadc.rmeta
    │   │       libsystem_integration-ce3a0595cc56995e.rmeta
    │   │       libtempfile-d2038a299aaf87c9.rmeta
    │   │       libthiserror-2b3c7dabc4d19fad.rmeta
    │   │       libthiserror-aeed0cd22ee35f1b.rmeta
    │   │       libthread_local-88b751e2b9939dde.rmeta
    │   │       libtime-4f9d624dfe7f47ca.rmeta
    │   │       libtime_core-6c3a95f088e22c10.rmeta
    │   │       libtime_fmt-56b43a9ab887d666.rmeta
    │   │       libtinystr-95da5c26ea10feb2.rmeta
    │   │       libtokio-9813a62fc61b5656.rmeta
    │   │       libtokio_native_tls-1ad6c090cf32fb29.rmeta
    │   │       libtokio_tungstenite-4dcc290b6a4dc2e1.rmeta
    │   │       libtokio_util-70e574008207cea0.rmeta
    │   │       libtoml-536cef40ee9c60a1.rmeta
    │   │       libtoml_datetime-fe29046d2032b3d6.rmeta
    │   │       libtoml_edit-afc4149e6bfee789.rmeta
    │   │       libtoml_write-d358a4b373d090f4.rmeta
    │   │       libtower-9394206f949c9c86.rmeta
    │   │       libtower_http-94deac2c047dc809.rmeta
    │   │       libtower_layer-7e185b956ba54a13.rmeta
    │   │       libtower_service-2927cc59f3517aa6.rmeta
    │   │       libtracing-56d9c2f009985c2c.rmeta
    │   │       libtracing_appender-c605e37596d64275.rmeta
    │   │       libtracing_core-98f4e7a70ae1824a.rmeta
    │   │       libtracing_log-80b1016ed78c7b75.rmeta
    │   │       libtracing_serde-3e74548d99c607de.rmeta
    │   │       libtracing_subscriber-95ae81b028daa2fa.rmeta
    │   │       libtry_lock-2bee410b7992e624.rmeta
    │   │       libtungstenite-a72e30b99c65c2c4.rmeta
    │   │       libtypenum-961b033b496ec94c.rmeta
    │   │       libunicode_ident-a3528efee345a255.rlib
    │   │       libunicode_ident-a3528efee345a255.rmeta
    │   │       liburl-a1207ccce8aa94f1.rmeta
    │   │       libutf8-33a96f27ac303f0c.rmeta
    │   │       libutf8_iter-67205964bf5d7618.rmeta
    │   │       libuuid-75423bf9870a83c5.rmeta
    │   │       libversion_check-793d72f0c6a3aed0.rlib
    │   │       libversion_check-793d72f0c6a3aed0.rmeta
    │   │       libwalkdir-32991585e463f7bb.rmeta
    │   │       libwant-c4492102deb9fa34.rmeta
    │   │       libwinapi-1c985f109ede21a0.rmeta
    │   │       libwinapi_util-bf148f29d8680adc.rmeta
    │   │       libwindows_link-21d82eae9c854e31.rmeta
    │   │       libwindows_registry-596af5a0f5ffe08d.rmeta
    │   │       libwindows_result-5a3169ef8407aca2.rmeta
    │   │       libwindows_strings-12d818ac320dacd5.rmeta
    │   │       libwindows_sys-373280222169ebae.rmeta
    │   │       libwindows_sys-cfb02564d4c64123.rmeta
    │   │       libwindows_sys-e73f6f53a92904f0.rmeta
    │   │       libwindows_targets-685851b4a7182906.rmeta
    │   │       libwindows_targets-b5624e17f0a1ad8b.rmeta
    │   │       libwindows_x86_64_msvc-24e925349af9f453.rmeta
    │   │       libwindows_x86_64_msvc-3e6e4ddbeeceebc3.rmeta
    │   │       libwinnow-c3ec360aed9dee22.rmeta
    │   │       libwriteable-f3ed0b6a2407068f.rmeta
    │   │       libyara-792cb22a67194d72.rmeta
    │   │       libyara_sys-d0dccd4b2047a8fe.rmeta
    │   │       libyoke-1e93e6e17e5bc6fb.rmeta
    │   │       libzerocopy-16d3069820863144.rmeta
    │   │       libzerofrom-7b4119952d0a1a19.rmeta
    │   │       libzeroize-f2a9032fdcbcbf6b.rmeta
    │   │       libzerotrie-f4827d4265141eeb.rmeta
    │   │       libzerovec-0e42ca7652e42ae4.rmeta
    │   │       litemap-99aba40979849771.d
    │   │       lock_api-e9d8b5157e526545.d
    │   │       log-4fd96ec37443cbba.d
    │   │       matchers-98113fb8bc145d9f.d
    │   │       matchit-8e5c37df1ea8ce26.d
    │   │       memchr-d21ed81bcc5c2a12.d
    │   │       mime-f51d29a9076e2fef.d
    │   │       miniz_oxide-5de3e2f572df22da.d
    │   │       mio-b74fc8fbf29e9adb.d
    │   │       native_tls-91f085e89949adf0.d
    │   │       notify-7514b83b65fb1344.d
    │   │       notify-9511bfd2115a3c48.d
    │   │       notify_types-cf90db815a4b4217.d
    │   │       num_conv-55ec824b6f8f9351.d
    │   │       num_traits-8c538d1c0fb7c5a1.d
    │   │       nu_ansi_term-e57b887bb81d1805.d
    │   │       once_cell-02d9b1423b4c3a99.d
    │   │       overload-39bd931dd46f4fb1.d
    │   │       parking_lot-5320cbe173ddeca5.d
    │   │       parking_lot_core-87879622a5efa30a.d
    │   │       percent_encoding-226c9e1a82702448.d
    │   │       pin_project_lite-360f5ac79d6490a8.d
    │   │       pin_utils-6131d7e882b3d82e.d
    │   │       potential_utf-46997aa979ed06de.d
    │   │       powerfmt-19f2ce97eac604ff.d
    │   │       ppv_lite86-968964c6cb7b2a5c.d
    │   │       proc_macro2-6a20e463e4a4ca20.d
    │   │       quote-fdeee9df46234d2d.d
    │   │       rand-1359bed459c03497.d
    │   │       rand_chacha-6d2e0a3d1d0cf058.d
    │   │       rand_core-9dcb78c9132a4656.d
    │   │       rayon-dfab54025ddc28fc.d
    │   │       rayon_core-58356cac7752356c.d
    │   │       regex-7126983b5ef91054.d
    │   │       regex_automata-91efb59fcc2b2c23.d
    │   │       regex_automata-ae3084067eb369ab.d
    │   │       regex_syntax-2ddbb26afe1c0a64.d
    │   │       regex_syntax-bfd7e1ff807f1064.d
    │   │       reqwest-a31f51c3b0b65d42.d
    │   │       rustls_pki_types-d9f71a2bd6fc478f.d
    │   │       rustversion-d5408a2b808eb648.d
    │   │       rustversion-d5408a2b808eb648.dll
    │   │       rustversion-d5408a2b808eb648.dll.exp
    │   │       rustversion-d5408a2b808eb648.dll.lib
    │   │       rustversion-d5408a2b808eb648.pdb
    │   │       ryu-3442447a74614b27.d
    │   │       same_file-2da6eccfbf8519af.d
    │   │       schannel-8868a50aa66715dc.d
    │   │       scopeguard-66a9ca5b98e4d6bf.d
    │   │       serde-e2c5fc6a4f673cbb.d
    │   │       serde_derive-ba6d0f103c04c0d5.d
    │   │       serde_derive-ba6d0f103c04c0d5.dll
    │   │       serde_derive-ba6d0f103c04c0d5.dll.exp
    │   │       serde_derive-ba6d0f103c04c0d5.dll.lib
    │   │       serde_derive-ba6d0f103c04c0d5.pdb
    │   │       serde_json-4f4830174827aea7.d
    │   │       serde_path_to_error-bc040f3aaac335b7.d
    │   │       serde_spanned-070ad526d1db460c.d
    │   │       serde_urlencoded-980a8cf95a86cfca.d
    │   │       sha1-4483d9d32339807e.d
    │   │       sha2-64b0ec1eee2551f5.d
    │   │       sharded_slab-74a2b9b853e8fc13.d
    │   │       shlex-27ba53e6f2866fee.d
    │   │       slab-6403e07a840fb0da.d
    │   │       smallvec-fd9fc186d6d6180f.d
    │   │       socket2-7074eedfe5b08109.d
    │   │       stable_deref_trait-a5623622a362b8d6.d
    │   │       syn-a3600ff6e0b8e983.d
    │   │       sync_wrapper-b8ddcd24d18460c7.d
    │   │       synstructure-75469bf036e5ef30.d
    │   │       system_integration-0d7081e5500ceadc.d
    │   │       system_integration-ce3a0595cc56995e.d
    │   │       tempfile-d2038a299aaf87c9.d
    │   │       thiserror-2b3c7dabc4d19fad.d
    │   │       thiserror-aeed0cd22ee35f1b.d
    │   │       thiserror_impl-08d57bf759d359db.d
    │   │       thiserror_impl-08d57bf759d359db.dll
    │   │       thiserror_impl-08d57bf759d359db.dll.exp
    │   │       thiserror_impl-08d57bf759d359db.dll.lib
    │   │       thiserror_impl-08d57bf759d359db.pdb
    │   │       thiserror_impl-d09ea1689e762b18.d
    │   │       thiserror_impl-d09ea1689e762b18.dll
    │   │       thiserror_impl-d09ea1689e762b18.dll.exp
    │   │       thiserror_impl-d09ea1689e762b18.dll.lib
    │   │       thiserror_impl-d09ea1689e762b18.pdb
    │   │       thread_local-88b751e2b9939dde.d
    │   │       time-4f9d624dfe7f47ca.d
    │   │       time_core-6c3a95f088e22c10.d
    │   │       time_fmt-56b43a9ab887d666.d
    │   │       tinystr-95da5c26ea10feb2.d
    │   │       tokio-9813a62fc61b5656.d
    │   │       tokio_macros-b106dd764d7a03ff.d
    │   │       tokio_macros-b106dd764d7a03ff.dll
    │   │       tokio_macros-b106dd764d7a03ff.dll.exp
    │   │       tokio_macros-b106dd764d7a03ff.dll.lib
    │   │       tokio_macros-b106dd764d7a03ff.pdb
    │   │       tokio_native_tls-1ad6c090cf32fb29.d
    │   │       tokio_tungstenite-4dcc290b6a4dc2e1.d
    │   │       tokio_util-70e574008207cea0.d
    │   │       toml-536cef40ee9c60a1.d
    │   │       toml_datetime-fe29046d2032b3d6.d
    │   │       toml_edit-afc4149e6bfee789.d
    │   │       toml_write-d358a4b373d090f4.d
    │   │       tower-9394206f949c9c86.d
    │   │       tower_http-94deac2c047dc809.d
    │   │       tower_layer-7e185b956ba54a13.d
    │   │       tower_service-2927cc59f3517aa6.d
    │   │       tracing-56d9c2f009985c2c.d
    │   │       tracing_appender-c605e37596d64275.d
    │   │       tracing_attributes-6763a40155b38c11.d
    │   │       tracing_attributes-6763a40155b38c11.dll
    │   │       tracing_attributes-6763a40155b38c11.dll.exp
    │   │       tracing_attributes-6763a40155b38c11.dll.lib
    │   │       tracing_attributes-6763a40155b38c11.pdb
    │   │       tracing_core-98f4e7a70ae1824a.d
    │   │       tracing_log-80b1016ed78c7b75.d
    │   │       tracing_serde-3e74548d99c607de.d
    │   │       tracing_subscriber-95ae81b028daa2fa.d
    │   │       try_lock-2bee410b7992e624.d
    │   │       tungstenite-a72e30b99c65c2c4.d
    │   │       typenum-961b033b496ec94c.d
    │   │       unicode_ident-a3528efee345a255.d
    │   │       url-a1207ccce8aa94f1.d
    │   │       utf8-33a96f27ac303f0c.d
    │   │       utf8_iter-67205964bf5d7618.d
    │   │       uuid-75423bf9870a83c5.d
    │   │       version_check-793d72f0c6a3aed0.d
    │   │       walkdir-32991585e463f7bb.d
    │   │       want-c4492102deb9fa34.d
    │   │       winapi-1c985f109ede21a0.d
    │   │       winapi_util-bf148f29d8680adc.d
    │   │       windows_link-21d82eae9c854e31.d
    │   │       windows_registry-596af5a0f5ffe08d.d
    │   │       windows_result-5a3169ef8407aca2.d
    │   │       windows_strings-12d818ac320dacd5.d
    │   │       windows_sys-373280222169ebae.d
    │   │       windows_sys-cfb02564d4c64123.d
    │   │       windows_sys-e73f6f53a92904f0.d
    │   │       windows_targets-685851b4a7182906.d
    │   │       windows_targets-b5624e17f0a1ad8b.d
    │   │       windows_x86_64_msvc-24e925349af9f453.d
    │   │       windows_x86_64_msvc-3e6e4ddbeeceebc3.d
    │   │       winnow-c3ec360aed9dee22.d
    │   │       writeable-f3ed0b6a2407068f.d
    │   │       yara-792cb22a67194d72.d
    │   │       yara_sys-d0dccd4b2047a8fe.d
    │   │       yoke-1e93e6e17e5bc6fb.d
    │   │       yoke_derive-6bb03d86b96f97db.d
    │   │       yoke_derive-6bb03d86b96f97db.dll
    │   │       yoke_derive-6bb03d86b96f97db.dll.exp
    │   │       yoke_derive-6bb03d86b96f97db.dll.lib
    │   │       yoke_derive-6bb03d86b96f97db.pdb
    │   │       zerocopy-16d3069820863144.d
    │   │       zerofrom-7b4119952d0a1a19.d
    │   │       zerofrom_derive-426526b1eb347b0d.d
    │   │       zerofrom_derive-426526b1eb347b0d.dll
    │   │       zerofrom_derive-426526b1eb347b0d.dll.exp
    │   │       zerofrom_derive-426526b1eb347b0d.dll.lib
    │   │       zerofrom_derive-426526b1eb347b0d.pdb
    │   │       zeroize-f2a9032fdcbcbf6b.d
    │   │       zerotrie-f4827d4265141eeb.d
    │   │       zerovec-0e42ca7652e42ae4.d
    │   │       zerovec_derive-fb4c963738a13f95.d
    │   │       zerovec_derive-fb4c963738a13f95.dll
    │   │       zerovec_derive-fb4c963738a13f95.dll.exp
    │   │       zerovec_derive-fb4c963738a13f95.dll.lib
    │   │       zerovec_derive-fb4c963738a13f95.pdb
    │   │
    │   ├───examples
    │   └───incremental
    │       ├───aiav_core-1s5earqz4mc8i
    │       │   │   s-h8or3c4h1k-04xqixg.lock
    │       │   │
    │       │   └───s-h8or3c4h1k-04xqixg-f4tqmhkxwhf1p4wos3da8ehp8
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───aiav_core-25nh2c283mess
    │       │   │   s-h8or3c4bb5-1bzs5t1.lock
    │       │   │
    │       │   └───s-h8or3c4bb5-1bzs5t1-dmv0jk5xj9dgrshfkcllgkcda
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───aiav_core-2mdszi5n0vgrb
    │       │   │   s-h8or3bgd7s-104asri.lock
    │       │   │
    │       │   └───s-h8or3bgd7s-104asri-aijuo6uhv1w31ceq30dqxfjbn
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───aiav_core-3ekbeon7kwprz
    │       │   │   s-h8or3bg607-1w5243w.lock
    │       │   │
    │       │   └───s-h8or3bg607-1w5243w-2hb2hl90gpq43o19oj23uhm8k
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───build_script_build-2g0jrrmi8nfc8
    │       │   │   s-h8or2odu67-0a5zdox.lock
    │       │   │
    │       │   └───s-h8or2odu67-0a5zdox-7kmwipec1rpjsamuy17sd3mo5
    │       │           039ux8j3rsr52o6c7glhlfr9u.o
    │       │           0ndqsh1p5n2fu1yv75o7mqk43.o
    │       │           1pvgjl5a7ear3x3vfa1gwye33.o
    │       │           1r00dqdy0afacnydmeup4j95i.o
    │       │           2q8a0ongqndj25w3nlay46ny9.o
    │       │           3iachmp7tau3ptoasg8h84sfy.o
    │       │           3kn7rvbofr0jfv7i3hbn4996g.o
    │       │           4z26djky7kbjd1mfafodkjzbm.o
    │       │           51mpj55zcas72otklobnfnxfh.o
    │       │           57c5fu1pyt1xysimzyp27e1sh.o
    │       │           5dl6exo58dwfsao45yayonjm4.o
    │       │           6wy7egep7w85vagrsl6c7g0vo.o
    │       │           8cjjdrlboiu199h3xvvw9tl9j.o
    │       │           9ux7gaopqfm72x1ecrrs5cbu9.o
    │       │           9yjz5rm8fafuj22cs3156m85k.o
    │       │           9zdiyfw4xh8vf70v25h0qcswn.o
    │       │           ah4egfa2ka63wdbfv8izoh2u9.o
    │       │           c61laa1x29v2u6ofhrariwwj7.o
    │       │           cc4y8tmrepl12s1zw8ap6204d.o
    │       │           dep-graph.bin
    │       │           eegussr9360h6vvofu6pz80zt.o
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───cloud_services-14leimjjyfgqw
    │       │   │   s-h8or349guk-1iapiha.lock
    │       │   │
    │       │   └───s-h8or349guk-1iapiha-bgpw53oxzcnoabzz83s3no4lw
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───cloud_services-3uutber5nbv0u
    │       │   │   s-h8or34bxss-0g5wr2k.lock
    │       │   │
    │       │   └───s-h8or34bxss-0g5wr2k-16b3km3vy9956on8y4f8rdzkf
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       ├───system_integration-08px2pez5k0wo
    │       │   │   s-h8or37etck-0yk2oz5.lock
    │       │   │
    │       │   └───s-h8or37etck-0yk2oz5-8d1ftv8ja9wkrmisj6hyb3gtb
    │       │           dep-graph.bin
    │       │           query-cache.bin
    │       │           work-products.bin
    │       │
    │       └───system_integration-3vycsk6bidqd8
    │           │   s-h8or37eqb1-03a4vl0.lock
    │           │
    │           └───s-h8or37eqb1-03a4vl0-6w8mlao2xen4p51y61cqk95fn
    │                   dep-graph.bin
    │                   query-cache.bin
    │                   work-products.bin
    │
    ├───release
    │   │   .cargo-lock
    │   │   aiav_core.d
    │   │   aiav_core.exe
    │   │   aiav_core.pdb
    │   │   system_integration.dll
    │   │
    │   ├───.fingerprint
    │   │   ├───adler2-a91714e349360340
    │   │   │       dep-lib-adler2
    │   │   │       invoked.timestamp
    │   │   │       lib-adler2
    │   │   │       lib-adler2.json
    │   │   │
    │   │   ├───aiav_core-393064d9268f1f0f
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───aiav_core-417b0c0b34885589
    │   │   │       bin-aiav_core
    │   │   │       bin-aiav_core.json
    │   │   │       dep-bin-aiav_core
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───aiav_core-a337b5898fdf3f72
    │   │   │       dep-lib-aiav_core
    │   │   │       invoked.timestamp
    │   │   │       lib-aiav_core
    │   │   │       lib-aiav_core.json
    │   │   │       output-lib-aiav_core
    │   │   │
    │   │   ├───aiav_core-dd977d7560372212
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───anyhow-425029e29b70ed17
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───anyhow-ae8fdb3917046c53
    │   │   │       dep-lib-anyhow
    │   │   │       invoked.timestamp
    │   │   │       lib-anyhow
    │   │   │       lib-anyhow.json
    │   │   │
    │   │   ├───anyhow-ff38c781c72d4a9d
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───async-compression-29bb534d98ac5f73
    │   │   │       dep-lib-async_compression
    │   │   │       invoked.timestamp
    │   │   │       lib-async_compression
    │   │   │       lib-async_compression.json
    │   │   │
    │   │   ├───async-trait-6321abafdafd2053
    │   │   │       dep-lib-async_trait
    │   │   │       invoked.timestamp
    │   │   │       lib-async_trait
    │   │   │       lib-async_trait.json
    │   │   │
    │   │   ├───atomic-waker-f5d65327110a6e9f
    │   │   │       dep-lib-atomic_waker
    │   │   │       invoked.timestamp
    │   │   │       lib-atomic_waker
    │   │   │       lib-atomic_waker.json
    │   │   │
    │   │   ├───autocfg-61ec581c137d3022
    │   │   │       dep-lib-autocfg
    │   │   │       invoked.timestamp
    │   │   │       lib-autocfg
    │   │   │       lib-autocfg.json
    │   │   │
    │   │   ├───axum-cd81b020da7cbb0a
    │   │   │       dep-lib-axum
    │   │   │       invoked.timestamp
    │   │   │       lib-axum
    │   │   │       lib-axum.json
    │   │   │
    │   │   ├───axum-core-8999a5c6ee71991b
    │   │   │       dep-lib-axum_core
    │   │   │       invoked.timestamp
    │   │   │       lib-axum_core
    │   │   │       lib-axum_core.json
    │   │   │
    │   │   ├───axum-macros-4ab30c4b25c5e146
    │   │   │       dep-lib-axum_macros
    │   │   │       invoked.timestamp
    │   │   │       lib-axum_macros
    │   │   │       lib-axum_macros.json
    │   │   │
    │   │   ├───base64-e24b524e93fa5c62
    │   │   │       dep-lib-base64
    │   │   │       invoked.timestamp
    │   │   │       lib-base64
    │   │   │       lib-base64.json
    │   │   │
    │   │   ├───bitflags-007785627f569cda
    │   │   │       dep-lib-bitflags
    │   │   │       invoked.timestamp
    │   │   │       lib-bitflags
    │   │   │       lib-bitflags.json
    │   │   │
    │   │   ├───block-buffer-1a46078d05cfc6f8
    │   │   │       dep-lib-block_buffer
    │   │   │       invoked.timestamp
    │   │   │       lib-block_buffer
    │   │   │       lib-block_buffer.json
    │   │   │
    │   │   ├───bytes-0aa7efc2e977500e
    │   │   │       dep-lib-bytes
    │   │   │       invoked.timestamp
    │   │   │       lib-bytes
    │   │   │       lib-bytes.json
    │   │   │
    │   │   ├───cc-7f94fdebce48ffa8
    │   │   │       dep-lib-cc
    │   │   │       invoked.timestamp
    │   │   │       lib-cc
    │   │   │       lib-cc.json
    │   │   │
    │   │   ├───cfg-if-c6d0dda5c7284520
    │   │   │       dep-lib-cfg_if
    │   │   │       invoked.timestamp
    │   │   │       lib-cfg_if
    │   │   │       lib-cfg_if.json
    │   │   │
    │   │   ├───chrono-a7b24d5960f726dc
    │   │   │       dep-lib-chrono
    │   │   │       invoked.timestamp
    │   │   │       lib-chrono
    │   │   │       lib-chrono.json
    │   │   │
    │   │   ├───cpufeatures-1ce9d5c3cb3cbca2
    │   │   │       dep-lib-cpufeatures
    │   │   │       invoked.timestamp
    │   │   │       lib-cpufeatures
    │   │   │       lib-cpufeatures.json
    │   │   │
    │   │   ├───crc32fast-05075484bef76adb
    │   │   │       dep-lib-crc32fast
    │   │   │       invoked.timestamp
    │   │   │       lib-crc32fast
    │   │   │       lib-crc32fast.json
    │   │   │
    │   │   ├───crossbeam-channel-7bfbdbff6e091de2
    │   │   │       dep-lib-crossbeam_channel
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_channel
    │   │   │       lib-crossbeam_channel.json
    │   │   │
    │   │   ├───crossbeam-deque-455d67003937737b
    │   │   │       dep-lib-crossbeam_deque
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_deque
    │   │   │       lib-crossbeam_deque.json
    │   │   │
    │   │   ├───crossbeam-epoch-70309bde7fc2cb9c
    │   │   │       dep-lib-crossbeam_epoch
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_epoch
    │   │   │       lib-crossbeam_epoch.json
    │   │   │
    │   │   ├───crossbeam-utils-55eac9e19b413f19
    │   │   │       dep-lib-crossbeam_utils
    │   │   │       invoked.timestamp
    │   │   │       lib-crossbeam_utils
    │   │   │       lib-crossbeam_utils.json
    │   │   │
    │   │   ├───crossbeam-utils-a7e63e84aeb138f5
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───crossbeam-utils-b9bda329fd2a7fef
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───crypto-common-2473bc00cf73bd40
    │   │   │       dep-lib-crypto_common
    │   │   │       invoked.timestamp
    │   │   │       lib-crypto_common
    │   │   │       lib-crypto_common.json
    │   │   │
    │   │   ├───data-encoding-b81b8fac16eb9244
    │   │   │       dep-lib-data_encoding
    │   │   │       invoked.timestamp
    │   │   │       lib-data_encoding
    │   │   │       lib-data_encoding.json
    │   │   │
    │   │   ├───deranged-6ad2bd433f15ff3c
    │   │   │       dep-lib-deranged
    │   │   │       invoked.timestamp
    │   │   │       lib-deranged
    │   │   │       lib-deranged.json
    │   │   │
    │   │   ├───digest-163e3169032ff52a
    │   │   │       dep-lib-digest
    │   │   │       invoked.timestamp
    │   │   │       lib-digest
    │   │   │       lib-digest.json
    │   │   │
    │   │   ├───displaydoc-e1cd28d5b7ec5a5c
    │   │   │       dep-lib-displaydoc
    │   │   │       invoked.timestamp
    │   │   │       lib-displaydoc
    │   │   │       lib-displaydoc.json
    │   │   │
    │   │   ├───either-40e003cf640b5063
    │   │   │       dep-lib-either
    │   │   │       invoked.timestamp
    │   │   │       lib-either
    │   │   │       lib-either.json
    │   │   │
    │   │   ├───encoding_rs-336d7acf3906a098
    │   │   │       dep-lib-encoding_rs
    │   │   │       invoked.timestamp
    │   │   │       lib-encoding_rs
    │   │   │       lib-encoding_rs.json
    │   │   │
    │   │   ├───equivalent-3ce44829f870ed83
    │   │   │       dep-lib-equivalent
    │   │   │       invoked.timestamp
    │   │   │       lib-equivalent
    │   │   │       lib-equivalent.json
    │   │   │
    │   │   ├───fastrand-07c470db7b609c4b
    │   │   │       dep-lib-fastrand
    │   │   │       invoked.timestamp
    │   │   │       lib-fastrand
    │   │   │       lib-fastrand.json
    │   │   │
    │   │   ├───filetime-2e964a7bd4098a99
    │   │   │       dep-lib-filetime
    │   │   │       invoked.timestamp
    │   │   │       lib-filetime
    │   │   │       lib-filetime.json
    │   │   │
    │   │   ├───flate2-2bc9d9e2017fa912
    │   │   │       dep-lib-flate2
    │   │   │       invoked.timestamp
    │   │   │       lib-flate2
    │   │   │       lib-flate2.json
    │   │   │
    │   │   ├───fnv-54f892b845cf2ec0
    │   │   │       dep-lib-fnv
    │   │   │       invoked.timestamp
    │   │   │       lib-fnv
    │   │   │       lib-fnv.json
    │   │   │
    │   │   ├───form_urlencoded-11ea3024e3dea1f4
    │   │   │       dep-lib-form_urlencoded
    │   │   │       invoked.timestamp
    │   │   │       lib-form_urlencoded
    │   │   │       lib-form_urlencoded.json
    │   │   │
    │   │   ├───fs_extra-1c00be5779abf281
    │   │   │       dep-lib-fs_extra
    │   │   │       invoked.timestamp
    │   │   │       lib-fs_extra
    │   │   │       lib-fs_extra.json
    │   │   │
    │   │   ├───fs_extra-3d017a29d568cf1b
    │   │   │       dep-lib-fs_extra
    │   │   │       invoked.timestamp
    │   │   │       lib-fs_extra
    │   │   │       lib-fs_extra.json
    │   │   │
    │   │   ├───futures-96962199174e0217
    │   │   │       dep-lib-futures
    │   │   │       invoked.timestamp
    │   │   │       lib-futures
    │   │   │       lib-futures.json
    │   │   │
    │   │   ├───futures-channel-798cbad065ecdf4e
    │   │   │       dep-lib-futures_channel
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_channel
    │   │   │       lib-futures_channel.json
    │   │   │
    │   │   ├───futures-core-71c49047742881a2
    │   │   │       dep-lib-futures_core
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_core
    │   │   │       lib-futures_core.json
    │   │   │
    │   │   ├───futures-executor-879af228033e8e60
    │   │   │       dep-lib-futures_executor
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_executor
    │   │   │       lib-futures_executor.json
    │   │   │
    │   │   ├───futures-io-3435e85ac09014cd
    │   │   │       dep-lib-futures_io
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_io
    │   │   │       lib-futures_io.json
    │   │   │
    │   │   ├───futures-macro-7359decbc5abd909
    │   │   │       dep-lib-futures_macro
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_macro
    │   │   │       lib-futures_macro.json
    │   │   │
    │   │   ├───futures-sink-71cc1a12a67e1dc1
    │   │   │       dep-lib-futures_sink
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_sink
    │   │   │       lib-futures_sink.json
    │   │   │
    │   │   ├───futures-task-f082e4f446b7c4b1
    │   │   │       dep-lib-futures_task
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_task
    │   │   │       lib-futures_task.json
    │   │   │
    │   │   ├───futures-util-5e4f3538b4efff3d
    │   │   │       dep-lib-futures_util
    │   │   │       invoked.timestamp
    │   │   │       lib-futures_util
    │   │   │       lib-futures_util.json
    │   │   │
    │   │   ├───generic-array-2d66856823516b2e
    │   │   │       dep-lib-generic_array
    │   │   │       invoked.timestamp
    │   │   │       lib-generic_array
    │   │   │       lib-generic_array.json
    │   │   │
    │   │   ├───generic-array-994a7eb1c3915046
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───generic-array-da69c1e013da4c10
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───getrandom-0b655c80a5801821
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───getrandom-934b1dc09303d6c9
    │   │   │       dep-lib-getrandom
    │   │   │       invoked.timestamp
    │   │   │       lib-getrandom
    │   │   │       lib-getrandom.json
    │   │   │
    │   │   ├───getrandom-c6739071082e141a
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───glob-df36f6ca54e80cd6
    │   │   │       dep-lib-glob
    │   │   │       invoked.timestamp
    │   │   │       lib-glob
    │   │   │       lib-glob.json
    │   │   │
    │   │   ├───h2-1a2deab62510e8a5
    │   │   │       dep-lib-h2
    │   │   │       invoked.timestamp
    │   │   │       lib-h2
    │   │   │       lib-h2.json
    │   │   │
    │   │   ├───hashbrown-ce0d3f688599f137
    │   │   │       dep-lib-hashbrown
    │   │   │       invoked.timestamp
    │   │   │       lib-hashbrown
    │   │   │       lib-hashbrown.json
    │   │   │
    │   │   ├───http-ba7a0be6ad9f96f7
    │   │   │       dep-lib-http
    │   │   │       invoked.timestamp
    │   │   │       lib-http
    │   │   │       lib-http.json
    │   │   │
    │   │   ├───http-body-75aaca9760ee5014
    │   │   │       dep-lib-http_body
    │   │   │       invoked.timestamp
    │   │   │       lib-http_body
    │   │   │       lib-http_body.json
    │   │   │
    │   │   ├───http-body-util-e79668cfbcbc86c0
    │   │   │       dep-lib-http_body_util
    │   │   │       invoked.timestamp
    │   │   │       lib-http_body_util
    │   │   │       lib-http_body_util.json
    │   │   │
    │   │   ├───httparse-5218a787346b7095
    │   │   │       dep-lib-httparse
    │   │   │       invoked.timestamp
    │   │   │       lib-httparse
    │   │   │       lib-httparse.json
    │   │   │
    │   │   ├───httparse-602b32ea3cc20e33
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───httparse-c60a42d330efc568
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───httpdate-a9b4396cf5708d3a
    │   │   │       dep-lib-httpdate
    │   │   │       invoked.timestamp
    │   │   │       lib-httpdate
    │   │   │       lib-httpdate.json
    │   │   │
    │   │   ├───hyper-14c41c7e95e1c689
    │   │   │       dep-lib-hyper
    │   │   │       invoked.timestamp
    │   │   │       lib-hyper
    │   │   │       lib-hyper.json
    │   │   │
    │   │   ├───hyper-tls-baa8cb3ffd3a6f60
    │   │   │       dep-lib-hyper_tls
    │   │   │       invoked.timestamp
    │   │   │       lib-hyper_tls
    │   │   │       lib-hyper_tls.json
    │   │   │
    │   │   ├───hyper-util-19f64b530b5b88d2
    │   │   │       dep-lib-hyper_util
    │   │   │       invoked.timestamp
    │   │   │       lib-hyper_util
    │   │   │       lib-hyper_util.json
    │   │   │
    │   │   ├───icu_collections-8a07ef4b5bc3e9be
    │   │   │       dep-lib-icu_collections
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_collections
    │   │   │       lib-icu_collections.json
    │   │   │
    │   │   ├───icu_locale_core-ed2c547bfbb51fb3
    │   │   │       dep-lib-icu_locale_core
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_locale_core
    │   │   │       lib-icu_locale_core.json
    │   │   │
    │   │   ├───icu_normalizer-3da369b65bdb0ee4
    │   │   │       dep-lib-icu_normalizer
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_normalizer
    │   │   │       lib-icu_normalizer.json
    │   │   │
    │   │   ├───icu_normalizer_data-29374487902c9b82
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───icu_normalizer_data-383af800317a8a0d
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───icu_normalizer_data-6d51904dee78038d
    │   │   │       dep-lib-icu_normalizer_data
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_normalizer_data
    │   │   │       lib-icu_normalizer_data.json
    │   │   │
    │   │   ├───icu_properties-3a86f6a3a2b79904
    │   │   │       dep-lib-icu_properties
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_properties
    │   │   │       lib-icu_properties.json
    │   │   │
    │   │   ├───icu_properties_data-0ad32791a5f1c294
    │   │   │       dep-lib-icu_properties_data
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_properties_data
    │   │   │       lib-icu_properties_data.json
    │   │   │
    │   │   ├───icu_properties_data-a75c10c0b95ca3a5
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───icu_properties_data-d90b8efa038250d7
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───icu_provider-3991b2007ebb8514
    │   │   │       dep-lib-icu_provider
    │   │   │       invoked.timestamp
    │   │   │       lib-icu_provider
    │   │   │       lib-icu_provider.json
    │   │   │
    │   │   ├───idna-115608ccbac85c49
    │   │   │       dep-lib-idna
    │   │   │       invoked.timestamp
    │   │   │       lib-idna
    │   │   │       lib-idna.json
    │   │   │
    │   │   ├───idna_adapter-fde30a62a7cbd8c5
    │   │   │       dep-lib-idna_adapter
    │   │   │       invoked.timestamp
    │   │   │       lib-idna_adapter
    │   │   │       lib-idna_adapter.json
    │   │   │
    │   │   ├───indexmap-22087521c20af0ea
    │   │   │       dep-lib-indexmap
    │   │   │       invoked.timestamp
    │   │   │       lib-indexmap
    │   │   │       lib-indexmap.json
    │   │   │
    │   │   ├───ipnet-94ee01a5d2cc0de5
    │   │   │       dep-lib-ipnet
    │   │   │       invoked.timestamp
    │   │   │       lib-ipnet
    │   │   │       lib-ipnet.json
    │   │   │
    │   │   ├───iri-string-f8b07435f68655e5
    │   │   │       dep-lib-iri_string
    │   │   │       invoked.timestamp
    │   │   │       lib-iri_string
    │   │   │       lib-iri_string.json
    │   │   │
    │   │   ├───itoa-8b2ac52ae81e4d36
    │   │   │       dep-lib-itoa
    │   │   │       invoked.timestamp
    │   │   │       lib-itoa
    │   │   │       lib-itoa.json
    │   │   │
    │   │   ├───lazy_static-750ea293523c32be
    │   │   │       dep-lib-lazy_static
    │   │   │       invoked.timestamp
    │   │   │       lib-lazy_static
    │   │   │       lib-lazy_static.json
    │   │   │
    │   │   ├───libc-2ac05cc7209cd09d
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───libc-64067b9c004baca8
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───libc-e32e016b50ea0291
    │   │   │       dep-lib-libc
    │   │   │       invoked.timestamp
    │   │   │       lib-libc
    │   │   │       lib-libc.json
    │   │   │
    │   │   ├───litemap-79dcf64bb5ca3df6
    │   │   │       dep-lib-litemap
    │   │   │       invoked.timestamp
    │   │   │       lib-litemap
    │   │   │       lib-litemap.json
    │   │   │
    │   │   ├───lock_api-189a950c53bb6605
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───lock_api-be4fb932db2d767a
    │   │   │       dep-lib-lock_api
    │   │   │       invoked.timestamp
    │   │   │       lib-lock_api
    │   │   │       lib-lock_api.json
    │   │   │
    │   │   ├───lock_api-c9b872d74c70190e
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───log-2497ab1b7cc93e09
    │   │   │       dep-lib-log
    │   │   │       invoked.timestamp
    │   │   │       lib-log
    │   │   │       lib-log.json
    │   │   │
    │   │   ├───matchers-dc8f6c10d305fb26
    │   │   │       dep-lib-matchers
    │   │   │       invoked.timestamp
    │   │   │       lib-matchers
    │   │   │       lib-matchers.json
    │   │   │
    │   │   ├───matchit-c1c7c2af364eaf4e
    │   │   │       dep-lib-matchit
    │   │   │       invoked.timestamp
    │   │   │       lib-matchit
    │   │   │       lib-matchit.json
    │   │   │
    │   │   ├───memchr-9fbfe290db6c3667
    │   │   │       dep-lib-memchr
    │   │   │       invoked.timestamp
    │   │   │       lib-memchr
    │   │   │       lib-memchr.json
    │   │   │
    │   │   ├───mime-f9f1f4fa98a806fe
    │   │   │       dep-lib-mime
    │   │   │       invoked.timestamp
    │   │   │       lib-mime
    │   │   │       lib-mime.json
    │   │   │
    │   │   ├───miniz_oxide-08d57ffd8421ae71
    │   │   │       dep-lib-miniz_oxide
    │   │   │       invoked.timestamp
    │   │   │       lib-miniz_oxide
    │   │   │       lib-miniz_oxide.json
    │   │   │
    │   │   ├───mio-c13e74388dda844e
    │   │   │       dep-lib-mio
    │   │   │       invoked.timestamp
    │   │   │       lib-mio
    │   │   │       lib-mio.json
    │   │   │
    │   │   ├───native-tls-7baa5ea4b10c1b22
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───native-tls-923165eaa7dc6b05
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───native-tls-f4a9bae321da6386
    │   │   │       dep-lib-native_tls
    │   │   │       invoked.timestamp
    │   │   │       lib-native_tls
    │   │   │       lib-native_tls.json
    │   │   │
    │   │   ├───notify-024f05088ffcc172
    │   │   │       dep-lib-notify
    │   │   │       invoked.timestamp
    │   │   │       lib-notify
    │   │   │       lib-notify.json
    │   │   │
    │   │   ├───notify-d426d512ee2486fb
    │   │   │       dep-lib-notify
    │   │   │       invoked.timestamp
    │   │   │       lib-notify
    │   │   │       lib-notify.json
    │   │   │
    │   │   ├───notify-types-de4748f95dbaa80a
    │   │   │       dep-lib-notify_types
    │   │   │       invoked.timestamp
    │   │   │       lib-notify_types
    │   │   │       lib-notify_types.json
    │   │   │
    │   │   ├───nu-ansi-term-46c8b5a72b74a1d8
    │   │   │       dep-lib-nu_ansi_term
    │   │   │       invoked.timestamp
    │   │   │       lib-nu_ansi_term
    │   │   │       lib-nu_ansi_term.json
    │   │   │
    │   │   ├───num-conv-2f62ad16c9a619c7
    │   │   │       dep-lib-num_conv
    │   │   │       invoked.timestamp
    │   │   │       lib-num_conv
    │   │   │       lib-num_conv.json
    │   │   │
    │   │   ├───num-traits-625f23918a0338c4
    │   │   │       dep-lib-num_traits
    │   │   │       invoked.timestamp
    │   │   │       lib-num_traits
    │   │   │       lib-num_traits.json
    │   │   │
    │   │   ├───num-traits-722a33a5c81fd1bd
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───num-traits-9b4c43b0be0f13cf
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───once_cell-a95a06a88addb51d
    │   │   │       dep-lib-once_cell
    │   │   │       invoked.timestamp
    │   │   │       lib-once_cell
    │   │   │       lib-once_cell.json
    │   │   │
    │   │   ├───overload-80fa2aa9b2d958d6
    │   │   │       dep-lib-overload
    │   │   │       invoked.timestamp
    │   │   │       lib-overload
    │   │   │       lib-overload.json
    │   │   │
    │   │   ├───parking_lot-1eaa1adcecab2ade
    │   │   │       dep-lib-parking_lot
    │   │   │       invoked.timestamp
    │   │   │       lib-parking_lot
    │   │   │       lib-parking_lot.json
    │   │   │
    │   │   ├───parking_lot_core-7dab19c47cc17e84
    │   │   │       dep-lib-parking_lot_core
    │   │   │       invoked.timestamp
    │   │   │       lib-parking_lot_core
    │   │   │       lib-parking_lot_core.json
    │   │   │
    │   │   ├───parking_lot_core-a019e7dabc9c2e47
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───parking_lot_core-ca05e6752816678a
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───percent-encoding-fdf869015c3cb7aa
    │   │   │       dep-lib-percent_encoding
    │   │   │       invoked.timestamp
    │   │   │       lib-percent_encoding
    │   │   │       lib-percent_encoding.json
    │   │   │
    │   │   ├───pin-project-lite-e16f2974c4bc55f8
    │   │   │       dep-lib-pin_project_lite
    │   │   │       invoked.timestamp
    │   │   │       lib-pin_project_lite
    │   │   │       lib-pin_project_lite.json
    │   │   │
    │   │   ├───pin-utils-06448c9b18db7896
    │   │   │       dep-lib-pin_utils
    │   │   │       invoked.timestamp
    │   │   │       lib-pin_utils
    │   │   │       lib-pin_utils.json
    │   │   │
    │   │   ├───potential_utf-26fe591d16104503
    │   │   │       dep-lib-potential_utf
    │   │   │       invoked.timestamp
    │   │   │       lib-potential_utf
    │   │   │       lib-potential_utf.json
    │   │   │
    │   │   ├───powerfmt-97762f50161a5e65
    │   │   │       dep-lib-powerfmt
    │   │   │       invoked.timestamp
    │   │   │       lib-powerfmt
    │   │   │       lib-powerfmt.json
    │   │   │
    │   │   ├───ppv-lite86-d5fce4fbbbbd65aa
    │   │   │       dep-lib-ppv_lite86
    │   │   │       invoked.timestamp
    │   │   │       lib-ppv_lite86
    │   │   │       lib-ppv_lite86.json
    │   │   │
    │   │   ├───proc-macro2-03e603e5e82ce587
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───proc-macro2-42fca9d61906e7ae
    │   │   │       dep-lib-proc_macro2
    │   │   │       invoked.timestamp
    │   │   │       lib-proc_macro2
    │   │   │       lib-proc_macro2.json
    │   │   │
    │   │   ├───proc-macro2-55d43a5f543f612d
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───quote-dcab23192f5e54b0
    │   │   │       dep-lib-quote
    │   │   │       invoked.timestamp
    │   │   │       lib-quote
    │   │   │       lib-quote.json
    │   │   │
    │   │   ├───rand-fbe3eaaeacb62fc3
    │   │   │       dep-lib-rand
    │   │   │       invoked.timestamp
    │   │   │       lib-rand
    │   │   │       lib-rand.json
    │   │   │
    │   │   ├───rand_chacha-d92d2f7340e3661b
    │   │   │       dep-lib-rand_chacha
    │   │   │       invoked.timestamp
    │   │   │       lib-rand_chacha
    │   │   │       lib-rand_chacha.json
    │   │   │
    │   │   ├───rand_core-4cbe42c1a5d68020
    │   │   │       dep-lib-rand_core
    │   │   │       invoked.timestamp
    │   │   │       lib-rand_core
    │   │   │       lib-rand_core.json
    │   │   │
    │   │   ├───rayon-core-55eec917a3498dfc
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───rayon-core-a3b18bb1eeb6697d
    │   │   │       dep-lib-rayon_core
    │   │   │       invoked.timestamp
    │   │   │       lib-rayon_core
    │   │   │       lib-rayon_core.json
    │   │   │
    │   │   ├───rayon-core-b15fe4b07b6be01e
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───rayon-efb084663cd91c8d
    │   │   │       dep-lib-rayon
    │   │   │       invoked.timestamp
    │   │   │       lib-rayon
    │   │   │       lib-rayon.json
    │   │   │
    │   │   ├───regex-automata-bd93915af68bcb9b
    │   │   │       dep-lib-regex_automata
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_automata
    │   │   │       lib-regex_automata.json
    │   │   │
    │   │   ├───regex-automata-c687f0b4254dbe7f
    │   │   │       dep-lib-regex_automata
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_automata
    │   │   │       lib-regex_automata.json
    │   │   │
    │   │   ├───regex-d693fea602dbe448
    │   │   │       dep-lib-regex
    │   │   │       invoked.timestamp
    │   │   │       lib-regex
    │   │   │       lib-regex.json
    │   │   │
    │   │   ├───regex-syntax-15d13742d5118ba4
    │   │   │       dep-lib-regex_syntax
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_syntax
    │   │   │       lib-regex_syntax.json
    │   │   │
    │   │   ├───regex-syntax-b4c487db393429b1
    │   │   │       dep-lib-regex_syntax
    │   │   │       invoked.timestamp
    │   │   │       lib-regex_syntax
    │   │   │       lib-regex_syntax.json
    │   │   │
    │   │   ├───reqwest-66700b6e7c8d2798
    │   │   │       dep-lib-reqwest
    │   │   │       invoked.timestamp
    │   │   │       lib-reqwest
    │   │   │       lib-reqwest.json
    │   │   │
    │   │   ├───rustls-pki-types-7a8b2040bfae87f7
    │   │   │       dep-lib-rustls_pki_types
    │   │   │       invoked.timestamp
    │   │   │       lib-rustls_pki_types
    │   │   │       lib-rustls_pki_types.json
    │   │   │
    │   │   ├───rustversion-42883a32881fbff9
    │   │   │       dep-lib-rustversion
    │   │   │       invoked.timestamp
    │   │   │       lib-rustversion
    │   │   │       lib-rustversion.json
    │   │   │
    │   │   ├───rustversion-6ac19f131d307d13
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───rustversion-8c5f00cba4172933
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───ryu-82697acd6f72601a
    │   │   │       dep-lib-ryu
    │   │   │       invoked.timestamp
    │   │   │       lib-ryu
    │   │   │       lib-ryu.json
    │   │   │
    │   │   ├───same-file-b632a472373cc769
    │   │   │       dep-lib-same_file
    │   │   │       invoked.timestamp
    │   │   │       lib-same_file
    │   │   │       lib-same_file.json
    │   │   │
    │   │   ├───schannel-2105f33340163e46
    │   │   │       dep-lib-schannel
    │   │   │       invoked.timestamp
    │   │   │       lib-schannel
    │   │   │       lib-schannel.json
    │   │   │
    │   │   ├───scopeguard-921be4800d80aec2
    │   │   │       dep-lib-scopeguard
    │   │   │       invoked.timestamp
    │   │   │       lib-scopeguard
    │   │   │       lib-scopeguard.json
    │   │   │
    │   │   ├───serde-1f109dc638bcb436
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───serde-93df092846f03f95
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───serde-f31d0f5c9df33671
    │   │   │       dep-lib-serde
    │   │   │       invoked.timestamp
    │   │   │       lib-serde
    │   │   │       lib-serde.json
    │   │   │
    │   │   ├───serde_derive-249edb18d4123132
    │   │   │       dep-lib-serde_derive
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_derive
    │   │   │       lib-serde_derive.json
    │   │   │
    │   │   ├───serde_json-5364743ce22b6124
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───serde_json-7f8da8b252fdf290
    │   │   │       dep-lib-serde_json
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_json
    │   │   │       lib-serde_json.json
    │   │   │
    │   │   ├───serde_json-82589005b33858ea
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───serde_path_to_error-643790b17c8c51f1
    │   │   │       dep-lib-serde_path_to_error
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_path_to_error
    │   │   │       lib-serde_path_to_error.json
    │   │   │
    │   │   ├───serde_spanned-8dc007a025c1fbbf
    │   │   │       dep-lib-serde_spanned
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_spanned
    │   │   │       lib-serde_spanned.json
    │   │   │
    │   │   ├───serde_urlencoded-caf52712984e37a4
    │   │   │       dep-lib-serde_urlencoded
    │   │   │       invoked.timestamp
    │   │   │       lib-serde_urlencoded
    │   │   │       lib-serde_urlencoded.json
    │   │   │
    │   │   ├───sha1-fea51e311bf177da
    │   │   │       dep-lib-sha1
    │   │   │       invoked.timestamp
    │   │   │       lib-sha1
    │   │   │       lib-sha1.json
    │   │   │
    │   │   ├───sha2-d6e0805e01c7925b
    │   │   │       dep-lib-sha2
    │   │   │       invoked.timestamp
    │   │   │       lib-sha2
    │   │   │       lib-sha2.json
    │   │   │
    │   │   ├───sharded-slab-14c29f69614d1917
    │   │   │       dep-lib-sharded_slab
    │   │   │       invoked.timestamp
    │   │   │       lib-sharded_slab
    │   │   │       lib-sharded_slab.json
    │   │   │
    │   │   ├───shlex-35ab51f09a2e0161
    │   │   │       dep-lib-shlex
    │   │   │       invoked.timestamp
    │   │   │       lib-shlex
    │   │   │       lib-shlex.json
    │   │   │
    │   │   ├───slab-e89cced06ddf8236
    │   │   │       dep-lib-slab
    │   │   │       invoked.timestamp
    │   │   │       lib-slab
    │   │   │       lib-slab.json
    │   │   │
    │   │   ├───smallvec-fb3eec18a0274313
    │   │   │       dep-lib-smallvec
    │   │   │       invoked.timestamp
    │   │   │       lib-smallvec
    │   │   │       lib-smallvec.json
    │   │   │
    │   │   ├───socket2-50a226a339bf00ae
    │   │   │       dep-lib-socket2
    │   │   │       invoked.timestamp
    │   │   │       lib-socket2
    │   │   │       lib-socket2.json
    │   │   │
    │   │   ├───stable_deref_trait-0cddb107f804f61d
    │   │   │       dep-lib-stable_deref_trait
    │   │   │       invoked.timestamp
    │   │   │       lib-stable_deref_trait
    │   │   │       lib-stable_deref_trait.json
    │   │   │
    │   │   ├───syn-5c069c3175d43129
    │   │   │       dep-lib-syn
    │   │   │       invoked.timestamp
    │   │   │       lib-syn
    │   │   │       lib-syn.json
    │   │   │
    │   │   ├───sync_wrapper-d46e05c922eb68bc
    │   │   │       dep-lib-sync_wrapper
    │   │   │       invoked.timestamp
    │   │   │       lib-sync_wrapper
    │   │   │       lib-sync_wrapper.json
    │   │   │
    │   │   ├───synstructure-31a6b0306b685eb8
    │   │   │       dep-lib-synstructure
    │   │   │       invoked.timestamp
    │   │   │       lib-synstructure
    │   │   │       lib-synstructure.json
    │   │   │
    │   │   ├───system-integration-aeb8c93e088097c2
    │   │   │       dep-lib-system_integration
    │   │   │       invoked.timestamp
    │   │   │       lib-system_integration
    │   │   │       lib-system_integration.json
    │   │   │
    │   │   ├───tempfile-732618c10a6c61fe
    │   │   │       dep-lib-tempfile
    │   │   │       invoked.timestamp
    │   │   │       lib-tempfile
    │   │   │       lib-tempfile.json
    │   │   │
    │   │   ├───thiserror-38c9fc2effd7942c
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───thiserror-3c2dd2db9da33329
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───thiserror-45609ecdd1baf4ad
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───thiserror-675460888fa41240
    │   │   │       dep-lib-thiserror
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror
    │   │   │       lib-thiserror.json
    │   │   │
    │   │   ├───thiserror-9a3d3f408f026f99
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───thiserror-b93595d23c9cfbf2
    │   │   │       dep-lib-thiserror
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror
    │   │   │       lib-thiserror.json
    │   │   │
    │   │   ├───thiserror-impl-51959aa310fa4041
    │   │   │       dep-lib-thiserror_impl
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror_impl
    │   │   │       lib-thiserror_impl.json
    │   │   │
    │   │   ├───thiserror-impl-88b3cd32dd849905
    │   │   │       dep-lib-thiserror_impl
    │   │   │       invoked.timestamp
    │   │   │       lib-thiserror_impl
    │   │   │       lib-thiserror_impl.json
    │   │   │
    │   │   ├───thread_local-61bc988c23e04bb4
    │   │   │       dep-lib-thread_local
    │   │   │       invoked.timestamp
    │   │   │       lib-thread_local
    │   │   │       lib-thread_local.json
    │   │   │
    │   │   ├───time-core-db7f11a7c1b4ab87
    │   │   │       dep-lib-time_core
    │   │   │       invoked.timestamp
    │   │   │       lib-time_core
    │   │   │       lib-time_core.json
    │   │   │
    │   │   ├───time-e3ce1df07e77717a
    │   │   │       dep-lib-time
    │   │   │       invoked.timestamp
    │   │   │       lib-time
    │   │   │       lib-time.json
    │   │   │
    │   │   ├───time-fmt-1aa1696dda24cbca
    │   │   │       dep-lib-time_fmt
    │   │   │       invoked.timestamp
    │   │   │       lib-time_fmt
    │   │   │       lib-time_fmt.json
    │   │   │
    │   │   ├───tinystr-3c1d36b832f0bd76
    │   │   │       dep-lib-tinystr
    │   │   │       invoked.timestamp
    │   │   │       lib-tinystr
    │   │   │       lib-tinystr.json
    │   │   │
    │   │   ├───tokio-a1fa662547faa898
    │   │   │       dep-lib-tokio
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio
    │   │   │       lib-tokio.json
    │   │   │
    │   │   ├───tokio-macros-2ae99dd1b20b3610
    │   │   │       dep-lib-tokio_macros
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_macros
    │   │   │       lib-tokio_macros.json
    │   │   │
    │   │   ├───tokio-native-tls-a940531aa8771407
    │   │   │       dep-lib-tokio_native_tls
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_native_tls
    │   │   │       lib-tokio_native_tls.json
    │   │   │
    │   │   ├───tokio-tungstenite-70a72e8465b3b968
    │   │   │       dep-lib-tokio_tungstenite
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_tungstenite
    │   │   │       lib-tokio_tungstenite.json
    │   │   │
    │   │   ├───tokio-util-22c8505954a72cfa
    │   │   │       dep-lib-tokio_util
    │   │   │       invoked.timestamp
    │   │   │       lib-tokio_util
    │   │   │       lib-tokio_util.json
    │   │   │
    │   │   ├───toml-e81de2c71e72c590
    │   │   │       dep-lib-toml
    │   │   │       invoked.timestamp
    │   │   │       lib-toml
    │   │   │       lib-toml.json
    │   │   │
    │   │   ├───toml_datetime-50934ea1be4f1f9b
    │   │   │       dep-lib-toml_datetime
    │   │   │       invoked.timestamp
    │   │   │       lib-toml_datetime
    │   │   │       lib-toml_datetime.json
    │   │   │
    │   │   ├───toml_edit-37c31486875479f2
    │   │   │       dep-lib-toml_edit
    │   │   │       invoked.timestamp
    │   │   │       lib-toml_edit
    │   │   │       lib-toml_edit.json
    │   │   │
    │   │   ├───toml_write-fe3d6e3726091bae
    │   │   │       dep-lib-toml_write
    │   │   │       invoked.timestamp
    │   │   │       lib-toml_write
    │   │   │       lib-toml_write.json
    │   │   │
    │   │   ├───tower-6eca708608896875
    │   │   │       dep-lib-tower
    │   │   │       invoked.timestamp
    │   │   │       lib-tower
    │   │   │       lib-tower.json
    │   │   │
    │   │   ├───tower-http-d4c8e384623ec765
    │   │   │       dep-lib-tower_http
    │   │   │       invoked.timestamp
    │   │   │       lib-tower_http
    │   │   │       lib-tower_http.json
    │   │   │
    │   │   ├───tower-layer-a229a13a7dcf6942
    │   │   │       dep-lib-tower_layer
    │   │   │       invoked.timestamp
    │   │   │       lib-tower_layer
    │   │   │       lib-tower_layer.json
    │   │   │
    │   │   ├───tower-service-f0756d2210fe710d
    │   │   │       dep-lib-tower_service
    │   │   │       invoked.timestamp
    │   │   │       lib-tower_service
    │   │   │       lib-tower_service.json
    │   │   │
    │   │   ├───tracing-0f675fd6e47c0c47
    │   │   │       dep-lib-tracing
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing
    │   │   │       lib-tracing.json
    │   │   │
    │   │   ├───tracing-appender-aee8c81779e5994f
    │   │   │       dep-lib-tracing_appender
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_appender
    │   │   │       lib-tracing_appender.json
    │   │   │
    │   │   ├───tracing-attributes-91db32819f743128
    │   │   │       dep-lib-tracing_attributes
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_attributes
    │   │   │       lib-tracing_attributes.json
    │   │   │
    │   │   ├───tracing-core-14bc279be94be20d
    │   │   │       dep-lib-tracing_core
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_core
    │   │   │       lib-tracing_core.json
    │   │   │
    │   │   ├───tracing-log-dd7b0e85d053cfdd
    │   │   │       dep-lib-tracing_log
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_log
    │   │   │       lib-tracing_log.json
    │   │   │
    │   │   ├───tracing-serde-5b30069d68f97efc
    │   │   │       dep-lib-tracing_serde
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_serde
    │   │   │       lib-tracing_serde.json
    │   │   │
    │   │   ├───tracing-subscriber-186fa1093e61f8d1
    │   │   │       dep-lib-tracing_subscriber
    │   │   │       invoked.timestamp
    │   │   │       lib-tracing_subscriber
    │   │   │       lib-tracing_subscriber.json
    │   │   │
    │   │   ├───try-lock-c4119e0a5cc7a76e
    │   │   │       dep-lib-try_lock
    │   │   │       invoked.timestamp
    │   │   │       lib-try_lock
    │   │   │       lib-try_lock.json
    │   │   │
    │   │   ├───tungstenite-da8492dc2925ba7d
    │   │   │       dep-lib-tungstenite
    │   │   │       invoked.timestamp
    │   │   │       lib-tungstenite
    │   │   │       lib-tungstenite.json
    │   │   │
    │   │   ├───typenum-3f747e578d898652
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───typenum-c814792aa62bf50e
    │   │   │       dep-lib-typenum
    │   │   │       invoked.timestamp
    │   │   │       lib-typenum
    │   │   │       lib-typenum.json
    │   │   │
    │   │   ├───typenum-f51a0db1f8fb82c0
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───unicode-ident-f40376275c848f02
    │   │   │       dep-lib-unicode_ident
    │   │   │       invoked.timestamp
    │   │   │       lib-unicode_ident
    │   │   │       lib-unicode_ident.json
    │   │   │
    │   │   ├───url-fce2138162876c2b
    │   │   │       dep-lib-url
    │   │   │       invoked.timestamp
    │   │   │       lib-url
    │   │   │       lib-url.json
    │   │   │
    │   │   ├───utf-8-b7d75fb04535868b
    │   │   │       dep-lib-utf8
    │   │   │       invoked.timestamp
    │   │   │       lib-utf8
    │   │   │       lib-utf8.json
    │   │   │
    │   │   ├───utf8_iter-3ef5bd26c581d05f
    │   │   │       dep-lib-utf8_iter
    │   │   │       invoked.timestamp
    │   │   │       lib-utf8_iter
    │   │   │       lib-utf8_iter.json
    │   │   │
    │   │   ├───uuid-bc1120679cd55c97
    │   │   │       dep-lib-uuid
    │   │   │       invoked.timestamp
    │   │   │       lib-uuid
    │   │   │       lib-uuid.json
    │   │   │
    │   │   ├───version_check-ecee75b39792c45e
    │   │   │       dep-lib-version_check
    │   │   │       invoked.timestamp
    │   │   │       lib-version_check
    │   │   │       lib-version_check.json
    │   │   │
    │   │   ├───walkdir-72372ca8b4d326ed
    │   │   │       dep-lib-walkdir
    │   │   │       invoked.timestamp
    │   │   │       lib-walkdir
    │   │   │       lib-walkdir.json
    │   │   │
    │   │   ├───want-243dfde86f45217c
    │   │   │       dep-lib-want
    │   │   │       invoked.timestamp
    │   │   │       lib-want
    │   │   │       lib-want.json
    │   │   │
    │   │   ├───winapi-004b18de43d27af5
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───winapi-205ebb9e8d63ccbc
    │   │   │       dep-lib-winapi
    │   │   │       invoked.timestamp
    │   │   │       lib-winapi
    │   │   │       lib-winapi.json
    │   │   │
    │   │   ├───winapi-c36795ec7bcf4203
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───winapi-util-80c3cee3fd87fad0
    │   │   │       dep-lib-winapi_util
    │   │   │       invoked.timestamp
    │   │   │       lib-winapi_util
    │   │   │       lib-winapi_util.json
    │   │   │
    │   │   ├───windows-link-91a052856f9bb0ec
    │   │   │       dep-lib-windows_link
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_link
    │   │   │       lib-windows_link.json
    │   │   │
    │   │   ├───windows-registry-ada8148771a4d13d
    │   │   │       dep-lib-windows_registry
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_registry
    │   │   │       lib-windows_registry.json
    │   │   │
    │   │   ├───windows-result-d7e9129cc1a0bd63
    │   │   │       dep-lib-windows_result
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_result
    │   │   │       lib-windows_result.json
    │   │   │
    │   │   ├───windows-strings-35eaef3ad4595c32
    │   │   │       dep-lib-windows_strings
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_strings
    │   │   │       lib-windows_strings.json
    │   │   │
    │   │   ├───windows-sys-2543d0a0a5185d71
    │   │   │       dep-lib-windows_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_sys
    │   │   │       lib-windows_sys.json
    │   │   │
    │   │   ├───windows-sys-41da0970ac15e060
    │   │   │       dep-lib-windows_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_sys
    │   │   │       lib-windows_sys.json
    │   │   │
    │   │   ├───windows-sys-933e89c96c921385
    │   │   │       dep-lib-windows_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_sys
    │   │   │       lib-windows_sys.json
    │   │   │
    │   │   ├───windows-targets-542b06f200ae9feb
    │   │   │       dep-lib-windows_targets
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_targets
    │   │   │       lib-windows_targets.json
    │   │   │
    │   │   ├───windows-targets-7450d71b090de8af
    │   │   │       dep-lib-windows_targets
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_targets
    │   │   │       lib-windows_targets.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-1bfa4e7a31a5bfd2
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───windows_x86_64_msvc-2487e97a9af7faa7
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-36f95b1ffa8a9e3e
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───windows_x86_64_msvc-657020c39551da92
    │   │   │       dep-lib-windows_x86_64_msvc
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_x86_64_msvc
    │   │   │       lib-windows_x86_64_msvc.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-b91860b336b577ee
    │   │   │       dep-lib-windows_x86_64_msvc
    │   │   │       invoked.timestamp
    │   │   │       lib-windows_x86_64_msvc
    │   │   │       lib-windows_x86_64_msvc.json
    │   │   │
    │   │   ├───windows_x86_64_msvc-c5da1bf27149a5ce
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───winnow-f89320763f898ab0
    │   │   │       dep-lib-winnow
    │   │   │       invoked.timestamp
    │   │   │       lib-winnow
    │   │   │       lib-winnow.json
    │   │   │
    │   │   ├───writeable-8b72f77cdd141252
    │   │   │       dep-lib-writeable
    │   │   │       invoked.timestamp
    │   │   │       lib-writeable
    │   │   │       lib-writeable.json
    │   │   │
    │   │   ├───yara-1a2fb30b47c6b51c
    │   │   │       dep-lib-yara
    │   │   │       invoked.timestamp
    │   │   │       lib-yara
    │   │   │       lib-yara.json
    │   │   │
    │   │   ├───yara-sys-1cfc1fbe1feeb3bd
    │   │   │       dep-lib-yara_sys
    │   │   │       invoked.timestamp
    │   │   │       lib-yara_sys
    │   │   │       lib-yara_sys.json
    │   │   │
    │   │   ├───yara-sys-282d8af6be18de16
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───yara-sys-5074e7948e921fca
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───yoke-derive-3a898eed09391613
    │   │   │       dep-lib-yoke_derive
    │   │   │       invoked.timestamp
    │   │   │       lib-yoke_derive
    │   │   │       lib-yoke_derive.json
    │   │   │
    │   │   ├───yoke-f5163d48fee778ca
    │   │   │       dep-lib-yoke
    │   │   │       invoked.timestamp
    │   │   │       lib-yoke
    │   │   │       lib-yoke.json
    │   │   │
    │   │   ├───zerocopy-9a03daf9864ef42e
    │   │   │       dep-lib-zerocopy
    │   │   │       invoked.timestamp
    │   │   │       lib-zerocopy
    │   │   │       lib-zerocopy.json
    │   │   │
    │   │   ├───zerocopy-9e8652a158dd706d
    │   │   │       build-script-build-script-build
    │   │   │       build-script-build-script-build.json
    │   │   │       dep-build-script-build-script-build
    │   │   │       invoked.timestamp
    │   │   │
    │   │   ├───zerocopy-a002c5c5bf27c046
    │   │   │       run-build-script-build-script-build
    │   │   │       run-build-script-build-script-build.json
    │   │   │
    │   │   ├───zerofrom-d1e410de713137b5
    │   │   │       dep-lib-zerofrom
    │   │   │       invoked.timestamp
    │   │   │       lib-zerofrom
    │   │   │       lib-zerofrom.json
    │   │   │
    │   │   ├───zerofrom-derive-af4d786be22bee75
    │   │   │       dep-lib-zerofrom_derive
    │   │   │       invoked.timestamp
    │   │   │       lib-zerofrom_derive
    │   │   │       lib-zerofrom_derive.json
    │   │   │
    │   │   ├───zeroize-a98d846932453112
    │   │   │       dep-lib-zeroize
    │   │   │       invoked.timestamp
    │   │   │       lib-zeroize
    │   │   │       lib-zeroize.json
    │   │   │
    │   │   ├───zerotrie-321dd49dfd22b335
    │   │   │       dep-lib-zerotrie
    │   │   │       invoked.timestamp
    │   │   │       lib-zerotrie
    │   │   │       lib-zerotrie.json
    │   │   │
    │   │   ├───zerovec-5a822c4bb4ec249d
    │   │   │       dep-lib-zerovec
    │   │   │       invoked.timestamp
    │   │   │       lib-zerovec
    │   │   │       lib-zerovec.json
    │   │   │
    │   │   └───zerovec-derive-c6184e30a737d9f7
    │   │           dep-lib-zerovec_derive
    │   │           invoked.timestamp
    │   │           lib-zerovec_derive
    │   │           lib-zerovec_derive.json
    │   │
    │   ├───build
    │   │   ├───aiav_core-393064d9268f1f0f
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-393064d9268f1f0f.d
    │   │   │       build_script_build-393064d9268f1f0f.exe
    │   │   │       build_script_build-393064d9268f1f0f.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───aiav_core-dd977d7560372212
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───anyhow-425029e29b70ed17
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-425029e29b70ed17.d
    │   │   │       build_script_build-425029e29b70ed17.exe
    │   │   │       build_script_build-425029e29b70ed17.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───anyhow-ff38c781c72d4a9d
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───crossbeam-utils-a7e63e84aeb138f5
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-a7e63e84aeb138f5.d
    │   │   │       build_script_build-a7e63e84aeb138f5.exe
    │   │   │       build_script_build-a7e63e84aeb138f5.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───crossbeam-utils-b9bda329fd2a7fef
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───generic-array-994a7eb1c3915046
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───generic-array-da69c1e013da4c10
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-da69c1e013da4c10.d
    │   │   │       build_script_build-da69c1e013da4c10.exe
    │   │   │       build_script_build-da69c1e013da4c10.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───getrandom-0b655c80a5801821
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───getrandom-c6739071082e141a
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-c6739071082e141a.d
    │   │   │       build_script_build-c6739071082e141a.exe
    │   │   │       build_script_build-c6739071082e141a.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───httparse-602b32ea3cc20e33
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-602b32ea3cc20e33.d
    │   │   │       build_script_build-602b32ea3cc20e33.exe
    │   │   │       build_script_build-602b32ea3cc20e33.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───httparse-c60a42d330efc568
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───icu_normalizer_data-29374487902c9b82
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-29374487902c9b82.d
    │   │   │       build_script_build-29374487902c9b82.exe
    │   │   │       build_script_build-29374487902c9b82.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───icu_normalizer_data-383af800317a8a0d
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───icu_properties_data-a75c10c0b95ca3a5
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-a75c10c0b95ca3a5.d
    │   │   │       build_script_build-a75c10c0b95ca3a5.exe
    │   │   │       build_script_build-a75c10c0b95ca3a5.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───icu_properties_data-d90b8efa038250d7
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───libc-2ac05cc7209cd09d
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-2ac05cc7209cd09d.d
    │   │   │       build_script_build-2ac05cc7209cd09d.exe
    │   │   │       build_script_build-2ac05cc7209cd09d.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───libc-64067b9c004baca8
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───lock_api-189a950c53bb6605
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───lock_api-c9b872d74c70190e
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-c9b872d74c70190e.d
    │   │   │       build_script_build-c9b872d74c70190e.exe
    │   │   │       build_script_build-c9b872d74c70190e.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───native-tls-7baa5ea4b10c1b22
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───native-tls-923165eaa7dc6b05
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-923165eaa7dc6b05.d
    │   │   │       build_script_build-923165eaa7dc6b05.exe
    │   │   │       build_script_build-923165eaa7dc6b05.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───num-traits-722a33a5c81fd1bd
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───num-traits-9b4c43b0be0f13cf
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-9b4c43b0be0f13cf.d
    │   │   │       build_script_build-9b4c43b0be0f13cf.exe
    │   │   │       build_script_build-9b4c43b0be0f13cf.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───parking_lot_core-a019e7dabc9c2e47
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-a019e7dabc9c2e47.d
    │   │   │       build_script_build-a019e7dabc9c2e47.exe
    │   │   │       build_script_build-a019e7dabc9c2e47.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───parking_lot_core-ca05e6752816678a
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───proc-macro2-03e603e5e82ce587
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───proc-macro2-55d43a5f543f612d
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-55d43a5f543f612d.d
    │   │   │       build_script_build-55d43a5f543f612d.exe
    │   │   │       build_script_build-55d43a5f543f612d.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───rayon-core-55eec917a3498dfc
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───rayon-core-b15fe4b07b6be01e
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-b15fe4b07b6be01e.d
    │   │   │       build_script_build-b15fe4b07b6be01e.exe
    │   │   │       build_script_build-b15fe4b07b6be01e.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───rustversion-6ac19f131d307d13
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-6ac19f131d307d13.d
    │   │   │       build_script_build-6ac19f131d307d13.exe
    │   │   │       build_script_build-6ac19f131d307d13.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───rustversion-8c5f00cba4172933
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   │           version.expr
    │   │   │
    │   │   ├───serde-1f109dc638bcb436
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───serde-93df092846f03f95
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-93df092846f03f95.d
    │   │   │       build_script_build-93df092846f03f95.exe
    │   │   │       build_script_build-93df092846f03f95.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───serde_json-5364743ce22b6124
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-5364743ce22b6124.d
    │   │   │       build_script_build-5364743ce22b6124.exe
    │   │   │       build_script_build-5364743ce22b6124.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───serde_json-82589005b33858ea
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───thiserror-38c9fc2effd7942c
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───thiserror-3c2dd2db9da33329
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-3c2dd2db9da33329.d
    │   │   │       build_script_build-3c2dd2db9da33329.exe
    │   │   │       build_script_build-3c2dd2db9da33329.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───thiserror-45609ecdd1baf4ad
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───thiserror-9a3d3f408f026f99
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-9a3d3f408f026f99.d
    │   │   │       build_script_build-9a3d3f408f026f99.exe
    │   │   │       build_script_build-9a3d3f408f026f99.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───typenum-3f747e578d898652
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   │           tests.rs
    │   │   │
    │   │   ├───typenum-f51a0db1f8fb82c0
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-f51a0db1f8fb82c0.d
    │   │   │       build_script_build-f51a0db1f8fb82c0.exe
    │   │   │       build_script_build-f51a0db1f8fb82c0.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───winapi-004b18de43d27af5
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-004b18de43d27af5.d
    │   │   │       build_script_build-004b18de43d27af5.exe
    │   │   │       build_script_build-004b18de43d27af5.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───winapi-c36795ec7bcf4203
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───windows_x86_64_msvc-1bfa4e7a31a5bfd2
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-1bfa4e7a31a5bfd2.d
    │   │   │       build_script_build-1bfa4e7a31a5bfd2.exe
    │   │   │       build_script_build-1bfa4e7a31a5bfd2.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───windows_x86_64_msvc-2487e97a9af7faa7
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───windows_x86_64_msvc-36f95b1ffa8a9e3e
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-36f95b1ffa8a9e3e.d
    │   │   │       build_script_build-36f95b1ffa8a9e3e.exe
    │   │   │       build_script_build-36f95b1ffa8a9e3e.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───windows_x86_64_msvc-c5da1bf27149a5ce
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   ├───yara-sys-282d8af6be18de16
    │   │   │   │   invoked.timestamp
    │   │   │   │   output
    │   │   │   │   root-output
    │   │   │   │   stderr
    │   │   │   │
    │   │   │   └───out
    │   │   │       │   22d36f18b6be8ee9-ahocorasick.o
    │   │   │       │   22d36f18b6be8ee9-arena.o
    │   │   │       │   22d36f18b6be8ee9-atoms.o
    │   │   │       │   22d36f18b6be8ee9-base64.o
    │   │   │       │   22d36f18b6be8ee9-bitmask.o
    │   │   │       │   22d36f18b6be8ee9-compiler.o
    │   │   │       │   22d36f18b6be8ee9-endian.o
    │   │   │       │   22d36f18b6be8ee9-exec.o
    │   │   │       │   22d36f18b6be8ee9-exefiles.o
    │   │   │       │   22d36f18b6be8ee9-filemap.o
    │   │   │       │   22d36f18b6be8ee9-grammar.o
    │   │   │       │   22d36f18b6be8ee9-hash.o
    │   │   │       │   22d36f18b6be8ee9-hex_grammar.o
    │   │   │       │   22d36f18b6be8ee9-hex_lexer.o
    │   │   │       │   22d36f18b6be8ee9-lexer.o
    │   │   │       │   22d36f18b6be8ee9-libyara.o
    │   │   │       │   22d36f18b6be8ee9-mem.o
    │   │   │       │   22d36f18b6be8ee9-modules.o
    │   │   │       │   22d36f18b6be8ee9-notebook.o
    │   │   │       │   22d36f18b6be8ee9-object.o
    │   │   │       │   22d36f18b6be8ee9-parser.o
    │   │   │       │   22d36f18b6be8ee9-proc.o
    │   │   │       │   22d36f18b6be8ee9-re.o
    │   │   │       │   22d36f18b6be8ee9-re_grammar.o
    │   │   │       │   22d36f18b6be8ee9-re_lexer.o
    │   │   │       │   22d36f18b6be8ee9-rules.o
    │   │   │       │   22d36f18b6be8ee9-scan.o
    │   │   │       │   22d36f18b6be8ee9-scanner.o
    │   │   │       │   22d36f18b6be8ee9-simple_str.o
    │   │   │       │   22d36f18b6be8ee9-sizedstr.o
    │   │   │       │   22d36f18b6be8ee9-stack.o
    │   │   │       │   22d36f18b6be8ee9-stopwatch.o
    │   │   │       │   22d36f18b6be8ee9-stream.o
    │   │   │       │   22d36f18b6be8ee9-strutils.o
    │   │   │       │   22d36f18b6be8ee9-threading.o
    │   │   │       │   295d7888718ef7d0-console.o
    │   │   │       │   52ca377d95469586-string.o
    │   │   │       │   574351f11b774f17-tlsh.o
    │   │   │       │   574351f11b774f17-tlsh_impl.o
    │   │   │       │   574351f11b774f17-tlsh_util.o
    │   │   │       │   594ad001aa21621b-time.o
    │   │   │       │   89c1e75934c2c660-windows.o
    │   │   │       │   8bae5dcf030c4f2a-elf.o
    │   │   │       │   a2ca937764893f75-tests.o
    │   │   │       │   ade8b54a2f00ed02-math.o
    │   │   │       │   bindings.rs
    │   │   │       │   f2b158f18cf8d1ab-pe.o
    │   │   │       │   f2b158f18cf8d1ab-pe_utils.o
    │   │   │       │   flag_check.c
    │   │   │       │   libyara.a
    │   │   │       │   yara.lib
    │   │   │       │
    │   │   │       └───yara
    │   │   │           │   .bazelrc
    │   │   │           │   .clang-format
    │   │   │           │   .gitattributes
    │   │   │           │   .gitignore
    │   │   │           │   .readthedocs.yaml
    │   │   │           │   appveyor.yml
    │   │   │           │   AUTHORS
    │   │   │           │   bootstrap.sh
    │   │   │           │   BUILD.bazel
    │   │   │           │   build.sh
    │   │   │           │   configure.ac
    │   │   │           │   CONTRIBUTORS
    │   │   │           │   COPYING
    │   │   │           │   Makefile.am
    │   │   │           │   README.md
    │   │   │           │   sample.file
    │   │   │           │   sample.rules
    │   │   │           │   SECURITY.md
    │   │   │           │   WORKSPACE.bazel
    │   │   │           │   yara.man
    │   │   │           │   yara.pc.in
    │   │   │           │   yarac.man
    │   │   │           │
    │   │   │           ├───.github
    │   │   │           │   ├───ISSUE_TEMPLATE
    │   │   │           │   │       bug_report.md
    │   │   │           │   │       feature_request.md
    │   │   │           │   │
    │   │   │           │   └───workflows
    │   │   │           │           build.yml
    │   │   │           │           coverity.yml
    │   │   │           │           oss-fuzz.yml
    │   │   │           │
    │   │   │           ├───bazel
    │   │   │           │       jansson.BUILD
    │   │   │           │       jansson.bzl
    │   │   │           │       magic.BUILD
    │   │   │           │       openssl.BUILD
    │   │   │           │       yara.bzl
    │   │   │           │       yara_deps.bzl
    │   │   │           │
    │   │   │           ├───cli
    │   │   │           │       args.c
    │   │   │           │       args.h
    │   │   │           │       common.c
    │   │   │           │       common.h
    │   │   │           │       threading.c
    │   │   │           │       threading.h
    │   │   │           │       unicode.h
    │   │   │           │       yara.c
    │   │   │           │       yarac.c
    │   │   │           │
    │   │   │           ├───dist
    │   │   │           │       yara-python.spec
    │   │   │           │       yara.spec
    │   │   │           │
    │   │   │           ├───docs
    │   │   │           │   │   capi.rst
    │   │   │           │   │   commandline.rst
    │   │   │           │   │   conf.py
    │   │   │           │   │   docutils.conf
    │   │   │           │   │   gettingstarted.rst
    │   │   │           │   │   index.rst
    │   │   │           │   │   make.bat
    │   │   │           │   │   Makefile
    │   │   │           │   │   modules.rst
    │   │   │           │   │   requirements.txt
    │   │   │           │   │   writingmodules.rst
    │   │   │           │   │   writingrules.rst
    │   │   │           │   │   yarapython.rst
    │   │   │           │   │
    │   │   │           │   └───modules
    │   │   │           │           console.rst
    │   │   │           │           cuckoo.rst
    │   │   │           │           dotnet.rst
    │   │   │           │           elf.rst
    │   │   │           │           hash.rst
    │   │   │           │           magic.rst
    │   │   │           │           math.rst
    │   │   │           │           pe.rst
    │   │   │           │           string.rst
    │   │   │           │           time.rst
    │   │   │           │
    │   │   │           ├───extra
    │   │   │           │   │   logo.ai
    │   │   │           │   │   logo.svg
    │   │   │           │   │   old-logo.png
    │   │   │           │   │   old-logo.psd
    │   │   │           │   │   UltraEdit-wordfile.txt
    │   │   │           │   │
    │   │   │           │   └───codemirror
    │   │   │           │           index.html
    │   │   │           │           yara.js
    │   │   │           │
    │   │   │           ├───libyara
    │   │   │           │   │   ahocorasick.c
    │   │   │           │   │   arena.c
    │   │   │           │   │   atoms.c
    │   │   │           │   │   base64.c
    │   │   │           │   │   bitmask.c
    │   │   │           │   │   compiler.c
    │   │   │           │   │   crypto.h
    │   │   │           │   │   endian.c
    │   │   │           │   │   exception.h
    │   │   │           │   │   exec.c
    │   │   │           │   │   exefiles.c
    │   │   │           │   │   filemap.c
    │   │   │           │   │   grammar.c
    │   │   │           │   │   grammar.h
    │   │   │           │   │   grammar.y
    │   │   │           │   │   hash.c
    │   │   │           │   │   hex_grammar.c
    │   │   │           │   │   hex_grammar.h
    │   │   │           │   │   hex_grammar.y
    │   │   │           │   │   hex_lexer.c
    │   │   │           │   │   hex_lexer.l
    │   │   │           │   │   lexer.c
    │   │   │           │   │   lexer.l
    │   │   │           │   │   libyara.c
    │   │   │           │   │   mem.c
    │   │   │           │   │   modules.c
    │   │   │           │   │   notebook.c
    │   │   │           │   │   object.c
    │   │   │           │   │   parser.c
    │   │   │           │   │   proc.c
    │   │   │           │   │   re.c
    │   │   │           │   │   re_grammar.c
    │   │   │           │   │   re_grammar.h
    │   │   │           │   │   re_grammar.y
    │   │   │           │   │   re_lexer.c
    │   │   │           │   │   re_lexer.l
    │   │   │           │   │   rules.c
    │   │   │           │   │   scan.c
    │   │   │           │   │   scanner.c
    │   │   │           │   │   simple_str.c
    │   │   │           │   │   sizedstr.c
    │   │   │           │   │   stack.c
    │   │   │           │   │   stino.settings
    │   │   │           │   │   stopwatch.c
    │   │   │           │   │   stream.c
    │   │   │           │   │   strutils.c
    │   │   │           │   │   threading.c
    │   │   │           │   │
    │   │   │           │   ├───include
    │   │   │           │   │   │   yara.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───authenticode-parser
    │   │   │           │   │   │       authenticode.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───tlshc
    │   │   │           │   │   │       tlsh.h
    │   │   │           │   │   │
    │   │   │           │   │   └───yara
    │   │   │           │   │           ahocorasick.h
    │   │   │           │   │           arena.h
    │   │   │           │   │           atoms.h
    │   │   │           │   │           base64.h
    │   │   │           │   │           bitmask.h
    │   │   │           │   │           compiler.h
    │   │   │           │   │           dex.h
    │   │   │           │   │           dotnet.h
    │   │   │           │   │           elf.h
    │   │   │           │   │           elf_utils.h
    │   │   │           │   │           endian.h
    │   │   │           │   │           error.h
    │   │   │           │   │           exec.h
    │   │   │           │   │           exefiles.h
    │   │   │           │   │           filemap.h
    │   │   │           │   │           globals.h
    │   │   │           │   │           hash.h
    │   │   │           │   │           hex_lexer.h
    │   │   │           │   │           integers.h
    │   │   │           │   │           lexer.h
    │   │   │           │   │           libyara.h
    │   │   │           │   │           limits.h
    │   │   │           │   │           macho.h
    │   │   │           │   │           mem.h
    │   │   │           │   │           modules.h
    │   │   │           │   │           notebook.h
    │   │   │           │   │           object.h
    │   │   │           │   │           parser.h
    │   │   │           │   │           pe.h
    │   │   │           │   │           pe_utils.h
    │   │   │           │   │           proc.h
    │   │   │           │   │           re.h
    │   │   │           │   │           re_lexer.h
    │   │   │           │   │           rules.h
    │   │   │           │   │           scan.h
    │   │   │           │   │           scanner.h
    │   │   │           │   │           simple_str.h
    │   │   │           │   │           sizedstr.h
    │   │   │           │   │           stack.h
    │   │   │           │   │           stopwatch.h
    │   │   │           │   │           stream.h
    │   │   │           │   │           strutils.h
    │   │   │           │   │           threading.h
    │   │   │           │   │           types.h
    │   │   │           │   │           unaligned.h
    │   │   │           │   │           utils.h
    │   │   │           │   │
    │   │   │           │   ├───modules
    │   │   │           │   │   │   module_list
    │   │   │           │   │   │   pb_to_module.rst
    │   │   │           │   │   │
    │   │   │           │   │   ├───console
    │   │   │           │   │   │       console.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───cuckoo
    │   │   │           │   │   │       cuckoo.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───demo
    │   │   │           │   │   │       demo.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───dex
    │   │   │           │   │   │       dex.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───dotnet
    │   │   │           │   │   │       dotnet.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───elf
    │   │   │           │   │   │       elf.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───hash
    │   │   │           │   │   │       hash.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───macho
    │   │   │           │   │   │       macho.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───magic
    │   │   │           │   │   │       magic.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───math
    │   │   │           │   │   │       math.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───pb_tests
    │   │   │           │   │   │       pb_tests.c
    │   │   │           │   │   │       pb_tests.pb-c.c
    │   │   │           │   │   │       pb_tests.pb-c.h
    │   │   │           │   │   │       pb_tests.proto
    │   │   │           │   │   │       yara.pb-c.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───pe
    │   │   │           │   │   │   │   pe.c
    │   │   │           │   │   │   │   pe_utils.c
    │   │   │           │   │   │   │
    │   │   │           │   │   │   └───authenticode-parser
    │   │   │           │   │   │           authenticode.c
    │   │   │           │   │   │           certificate.c
    │   │   │           │   │   │           certificate.h
    │   │   │           │   │   │           countersignature.c
    │   │   │           │   │   │           countersignature.h
    │   │   │           │   │   │           helper.c
    │   │   │           │   │   │           helper.h
    │   │   │           │   │   │           structs.c
    │   │   │           │   │   │           structs.h
    │   │   │           │   │   │
    │   │   │           │   │   ├───string
    │   │   │           │   │   │       string.c
    │   │   │           │   │   │
    │   │   │           │   │   ├───tests
    │   │   │           │   │   │       tests.c
    │   │   │           │   │   │
    │   │   │           │   │   └───time
    │   │   │           │   │           time.c
    │   │   │           │   │
    │   │   │           │   ├───pb
    │   │   │           │   │       yara.proto
    │   │   │           │   │
    │   │   │           │   ├───proc
    │   │   │           │   │       freebsd.c
    │   │   │           │   │       linux.c
    │   │   │           │   │       mach.c
    │   │   │           │   │       none.c
    │   │   │           │   │       openbsd.c
    │   │   │           │   │       windows.c
    │   │   │           │   │
    │   │   │           │   └───tlshc
    │   │   │           │           tlsh.c
    │   │   │           │           tlsh_impl.c
    │   │   │           │           tlsh_impl.h
    │   │   │           │           tlsh_util.c
    │   │   │           │           tlsh_util.h
    │   │   │           │
    │   │   │           ├───m4
    │   │   │           │       acx_pthread.m4
    │   │   │           │
    │   │   │           ├───sandbox
    │   │   │           │       BUILD.bazel
    │   │   │           │       collect_matches.cc
    │   │   │           │       collect_matches.h
    │   │   │           │       sandboxed_yara.cc
    │   │   │           │       yara_entry_points.cc
    │   │   │           │       yara_matches.proto
    │   │   │           │       yara_transaction.cc
    │   │   │           │       yara_transaction.h
    │   │   │           │       yara_transaction_test.cc
    │   │   │           │
    │   │   │           ├───tests
    │   │   │           │   │   blob.h
    │   │   │           │   │   BUILD.bazel
    │   │   │           │   │   convention-portable-modifiers
    │   │   │           │   │   gcov-summary
    │   │   │           │   │   mapper.c
    │   │   │           │   │   test-alignment.c
    │   │   │           │   │   test-api.c
    │   │   │           │   │   test-arena.c
    │   │   │           │   │   test-async.c
    │   │   │           │   │   test-atoms.c
    │   │   │           │   │   test-bitmask.c
    │   │   │           │   │   test-dex.c
    │   │   │           │   │   test-dotnet.c
    │   │   │           │   │   test-elf.c
    │   │   │           │   │   test-exception.c
    │   │   │           │   │   test-macho.c
    │   │   │           │   │   test-magic.c
    │   │   │           │   │   test-math.c
    │   │   │           │   │   test-pb.c
    │   │   │           │   │   test-pe.c
    │   │   │           │   │   test-re-split.c
    │   │   │           │   │   test-rules.c
    │   │   │           │   │   test-stack.c
    │   │   │           │   │   test-string.c
    │   │   │           │   │   test-version.c
    │   │   │           │   │   util.c
    │   │   │           │   │   util.h
    │   │   │           │   │
    │   │   │           │   ├───data
    │   │   │           │   │   │   05cd06e6a202e12be22a02700ed6f1604e803ca8867277d852e8971efded0650
    │   │   │           │   │   │   079a472d22290a94ebb212aa8015cdc8dd28a968c6b4d3b88acdd58ce2d3b885
    │   │   │           │   │   │   079a472d22290a94ebb212aa8015cdc8dd28a968c6b4d3b88acdd58ce2d3b885.upx
    │   │   │           │   │   │   0ca09bde7602769120fadc4f7a4147347a7a97271370583586c9e587fd396171
    │   │   │           │   │   │   33fc70f99be6d2833ae48852d611c8048d0c053ed0b2c626db4dbe902832a08b
    │   │   │           │   │   │   3b8b90159fa9b6048cc5410c5d53f116943564e4d05b04a843f9b3d0540d0c1c
    │   │   │           │   │   │   6c2abf4b80a87e63eee2996e5cea8f004d49ec0c1806080fa72e960529cba14c
    │   │   │           │   │   │   756684f4017ba7e931a26724ae61606b16b5f8cc84ed38a260a34e50c5016f59
    │   │   │           │   │   │   bad_dotnet_pe
    │   │   │           │   │   │   base64
    │   │   │           │   │   │   baz.yar
    │   │   │           │   │   │   c6f9709feccf42f2d9e22057182fe185f177fb9daaa2649b4669a24f2ee7e3ba_0h_410h
    │   │   │           │   │   │   ca21e1c32065352d352be6cde97f89c141d7737ea92434831f998080783d5386
    │   │   │           │   │   │   ChipTune.efi
    │   │   │           │   │   │   e3d45a2865818756068757d7e319258fef40dad54532ee4355b86bc129f27345
    │   │   │           │   │   │   elf_with_imports
    │   │   │           │   │   │   foo.yar
    │   │   │           │   │   │   mtxex.dll
    │   │   │           │   │   │   mtxex_modified_rsrc_rva.dll
    │   │   │           │   │   │   pe_imports
    │   │   │           │   │   │   pe_mingw
    │   │   │           │   │   │   test-pb.data
    │   │   │           │   │   │   test-pb.data.bin
    │   │   │           │   │   │   tiny
    │   │   │           │   │   │   tiny-idata-51ff
    │   │   │           │   │   │   tiny-idata-5200
    │   │   │           │   │   │   tiny-macho
    │   │   │           │   │   │   tiny-overlay
    │   │   │           │   │   │   tiny-universal
    │   │   │           │   │   │   tiny.notes
    │   │   │           │   │   │   tiny_empty_import_name
    │   │   │           │   │   │   weird_rich
    │   │   │           │   │   │   x.txt
    │   │   │           │   │   │   xor.out
    │   │   │           │   │   │   xorwide.out
    │   │   │           │   │   │   xorwideandascii.out
    │   │   │           │   │   │
    │   │   │           │   │   └───include
    │   │   │           │   │           bar.yar
    │   │   │           │   │
    │   │   │           │   └───oss-fuzz
    │   │   │           │       │   dex_fuzzer.cc
    │   │   │           │       │   dotnet_fuzzer.cc
    │   │   │           │       │   elf_fuzzer.cc
    │   │   │           │       │   macho_fuzzer.cc
    │   │   │           │       │   pe_fuzzer.cc
    │   │   │           │       │   rules_fuzzer.cc
    │   │   │           │       │   rules_fuzzer.dict
    │   │   │           │       │   rules_fuzzer.options
    │   │   │           │       │
    │   │   │           │       ├───dex_fuzzer_corpus
    │   │   │           │       │       1cf540db2f048bb21bd89379a57279b9ff4c308558715a3baee666a47393d86e
    │   │   │           │       │       25ef27f9543444652f0c68fe412d3da627a1d2a590b0a2b30e47466c1e962136
    │   │   │           │       │       27fb31059503773723597edb875c937af971a6c15f91aac8c03c1fbdfa9e918c
    │   │   │           │       │       3ba9c082050f62e725c87ce4cf9f592fe9f177faf3a0c879f8fbe87312ca4b2c
    │   │   │           │       │       b1203d95c56f02e7e6dbea714275cc05b47ac2510958b85f436571b801af44e7
    │   │   │           │       │       b343d1058063e6e4b652ccf0589f93d0dbb6b092960e4aebc3c3c58894831359
    │   │   │           │       │       crash.poc
    │   │   │           │       │
    │   │   │           │       ├───dotnet_fuzzer_corpus
    │   │   │           │       │       buggy_stream_names
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5105966966636544
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5195285818507264
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5636481138556928
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5725060321509376
    │   │   │           │       │       clusterfuzz-testcase-minimized-dotnet_fuzzer-5880393521430528
    │   │   │           │       │       obfuscated
    │   │   │           │       │
    │   │   │           │       ├───elf_fuzzer_corpus
    │   │   │           │       │       crash-03bca75466ee42801a8bff280de04afc3d1a3637
    │   │   │           │       │       crash-086300bbce1c6537573057336a343a82d483e2c0
    │   │   │           │       │       crash-2cafe4de66d87a83d83aaf65d8e4cea48f2c1144
    │   │   │           │       │       crash-370485c5b087f780a2447a03d775f7188e323d31
    │   │   │           │       │       crash-49bb55d669fda0683f945b89396a6bd458caf2d8
    │   │   │           │       │       crash-49d00b6b033eaeb07cd39809dbc1d7ba2df196ec
    │   │   │           │       │       crash-723296cdc1c0dba83ea767d69286429e608c46c3
    │   │   │           │       │       crash-7dc27920ae1cb85333e7f2735a45014488134673
    │   │   │           │       │       crash-7e945ce5f43f515ea078c558a2e3205089d414e5
    │   │   │           │       │       crash-a809561e75b94bd5d4d8cf7488d9e2663fc1ccdc
    │   │   │           │       │       crash-a8715a38a94161c9509309f5dbb5a7936aba8376
    │   │   │           │       │       crash-aee928239444a7b039500d4499035e6d30cb89da
    │   │   │           │       │       crash-c4002396c52065d21fe1c1f05f8937aab8d59c18
    │   │   │           │       │       crash-c610b3036f195ad7fb05248a530278aad37b438d
    │   │   │           │       │       crash-c6569e6e28f0a18bb2f3bf49c982333a359bed67
    │   │   │           │       │       crash-cc6844f44825a785de1b079c88f728e1c0f779fb
    │   │   │           │       │       crash-f1fd008da535b110853885221ebfaac3f262a1c1e280f10929f7b353c44996c8
    │   │   │           │       │       poc-6bf54fca69bb5029676d747b12c74b597dd8c5939343ea8f2cbfea9e666dd6b1
    │   │   │           │       │       poc-789fc6da83de39c3ff394a950b0831f6fe5b63a85a46aaa236048b5c1dcf0e59
    │   │   │           │       │       poc-939e9cd87b0d80834210fbf54edc66341aebf416d7509f6633f1d49766978b22
    │   │   │           │       │       poc-93a9fd1909dd49fc2a9b654333504f249cdac58126d3cfc4728577e78cb3eb89
    │   │   │           │       │       poc-b5b03a1f305b2cc1c158e01fee6c08c65145325d4e073f04d969329577077862
    │   │   │           │       │       poc-fa8bbacb5a12f057a0ed3999c37d78b4991e6b201bda4dc9a75a7c7970c7690d
    │   │   │           │       │
    │   │   │           │       ├───macho_fuzzer_corpus
    │   │   │           │       │       1443c3cfb47c5eb41022a7063c24ab1bc9e45bfc31e98d5e6d3aa8377599b983
    │   │   │           │       │       589f7b0e30d885ed91229646e58ccc7615007d2fab06451fef8785c6126adba7
    │   │   │           │       │       5eefacbe52990526e4953802249447dd8c0a4b537459ca41e005a7173ca46138
    │   │   │           │       │       6164a837fd33574f37464a765ab461fff94b52e659b114fb6109f2635678c564
    │   │   │           │       │       66528aeb35dd705cc26a7daf4b8eda684f620efebfa0740fab84043e371ed566
    │   │   │           │       │       678d89b32eecd7d01390aaaf3507935b27854f4f3a7055e3f6b1b0ccf0ca5072
    │   │   │           │       │       6af5d157184d9144f86668f83e81760898df5db3c9e209596eb5fd9a91a7eeba
    │   │   │           │       │       797d1d450421b771482c0cc03f472e4eccbc9e4f544b6c12c1d4f070dec3c381
    │   │   │           │       │       85494d8cb5753f1ad09be39428135feb35eb4ef44f39d6e1e75e2ad30d93e158
    │   │   │           │       │       b225048e85b14f08a43dd4752b9bb4b20840f5a8726eac0ff765d45c9e619828
    │   │   │           │       │       fda81421d7403180923717a94e77aade8c9286d5b8de3ae0e2812343b666c6a7
    │   │   │           │       │
    │   │   │           │       ├───pe_fuzzer_corpus
    │   │   │           │       │       00388b550a2603a9e219bcb48acaf8cc115653cb1ea84cb4bccceb1aabe755b6
    │   │   │           │       │       12f50a7dbf0c42f61ae1c351b2a9f75e8edb3bb55e582619edc7ece4eb0a3094
    │   │   │           │       │       967af267b4124bada8f507cebf25f2192d146a4d63be71b45bfc03c5da7f21a7
    │   │   │           │       │       99e98cb7096dee974e28fea0f76f1c30bc44fd5762cb12b2702910a28b28f95f
    │   │   │           │       │       clusterfuzz-testcase-minimized-5211130361282560
    │   │   │           │       │       clusterfuzz-testcase-minimized-5839717883969536
    │   │   │           │       │       clusterfuzz-testcase-minimized-pe_fuzzer-5671228022718464
    │   │   │           │       │       clusterfuzz-testcase-minimized-pe_fuzzer-5741846293643264
    │   │   │           │       │       e5af0352010b1879ac1c63a69d3d9a02d577fa834165f855bd5ebee0f1105de1
    │   │   │           │       │
    │   │   │           │       └───rules_fuzzer_corpus
    │   │   │           │               1
    │   │   │           │               2
    │   │   │           │               3
    │   │   │           │               4
    │   │   │           │               5
    │   │   │           │               6
    │   │   │           │               7
    │   │   │           │               8
    │   │   │           │
    │   │   │           └───windows
    │   │   │               ├───vs2015
    │   │   │               │   │   NuGet.Config
    │   │   │               │   │   yara.sln
    │   │   │               │   │
    │   │   │               │   ├───libyara
    │   │   │               │   │       libyara.vcxproj
    │   │   │               │   │       packages.config
    │   │   │               │   │
    │   │   │               │   ├───test-alignment
    │   │   │               │   │       test-alignment.vcxproj
    │   │   │               │   │
    │   │   │               │   ├───yara
    │   │   │               │   │       yara.vcxproj
    │   │   │               │   │
    │   │   │               │   └───yarac
    │   │   │               │           yarac.vcxproj
    │   │   │               │
    │   │   │               ├───vs2017
    │   │   │               │   │   NuGet.Config
    │   │   │               │   │   yara.sln
    │   │   │               │   │
    │   │   │               │   ├───libyara
    │   │   │               │   │       libyara.vcxproj
    │   │   │               │   │       libyara.vcxproj.user
    │   │   │               │   │       packages.config
    │   │   │               │   │
    │   │   │               │   ├───yara
    │   │   │               │   │       yara.vcxproj
    │   │   │               │   │       yara.vcxproj.user
    │   │   │               │   │
    │   │   │               │   └───yarac
    │   │   │               │           yarac.vcxproj
    │   │   │               │           yarac.vcxproj.user
    │   │   │               │
    │   │   │               └───vs2019
    │   │   │                   │   NuGet.Config
    │   │   │                   │   yara.sln
    │   │   │                   │
    │   │   │                   ├───libyara
    │   │   │                   │       libyara.vcxproj
    │   │   │                   │       libyara.vcxproj.user
    │   │   │                   │       packages.config
    │   │   │                   │
    │   │   │                   ├───yara
    │   │   │                   │       yara.vcxproj
    │   │   │                   │       yara.vcxproj.user
    │   │   │                   │
    │   │   │                   └───yarac
    │   │   │                           yarac.vcxproj
    │   │   │                           yarac.vcxproj.user
    │   │   │
    │   │   ├───yara-sys-5074e7948e921fca
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-5074e7948e921fca.d
    │   │   │       build_script_build-5074e7948e921fca.exe
    │   │   │       build_script_build-5074e7948e921fca.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   ├───zerocopy-9e8652a158dd706d
    │   │   │       build-script-build.exe
    │   │   │       build_script_build-9e8652a158dd706d.d
    │   │   │       build_script_build-9e8652a158dd706d.exe
    │   │   │       build_script_build-9e8652a158dd706d.pdb
    │   │   │       build_script_build.pdb
    │   │   │
    │   │   └───zerocopy-a002c5c5bf27c046
    │   │       │   invoked.timestamp
    │   │       │   output
    │   │       │   root-output
    │   │       │   stderr
    │   │       │
    │   │       └───out
    │   ├───deps
    │   │       adler2-a91714e349360340.d
    │   │       aiav_core-a337b5898fdf3f72.d
    │   │       aiav_core.d
    │   │       aiav_core.exe
    │   │       aiav_core.pdb
    │   │       anyhow-ae8fdb3917046c53.d
    │   │       async_compression-29bb534d98ac5f73.d
    │   │       async_trait-6321abafdafd2053.d
    │   │       async_trait-6321abafdafd2053.dll
    │   │       async_trait-6321abafdafd2053.dll.exp
    │   │       async_trait-6321abafdafd2053.dll.lib
    │   │       async_trait-6321abafdafd2053.pdb
    │   │       atomic_waker-f5d65327110a6e9f.d
    │   │       autocfg-61ec581c137d3022.d
    │   │       axum-cd81b020da7cbb0a.d
    │   │       axum_core-8999a5c6ee71991b.d
    │   │       axum_macros-4ab30c4b25c5e146.d
    │   │       axum_macros-4ab30c4b25c5e146.dll
    │   │       axum_macros-4ab30c4b25c5e146.dll.exp
    │   │       axum_macros-4ab30c4b25c5e146.dll.lib
    │   │       axum_macros-4ab30c4b25c5e146.pdb
    │   │       base64-e24b524e93fa5c62.d
    │   │       bitflags-007785627f569cda.d
    │   │       block_buffer-1a46078d05cfc6f8.d
    │   │       bytes-0aa7efc2e977500e.d
    │   │       cc-7f94fdebce48ffa8.d
    │   │       cfg_if-c6d0dda5c7284520.d
    │   │       chrono-a7b24d5960f726dc.d
    │   │       cpufeatures-1ce9d5c3cb3cbca2.d
    │   │       crc32fast-05075484bef76adb.d
    │   │       crossbeam_channel-7bfbdbff6e091de2.d
    │   │       crossbeam_deque-455d67003937737b.d
    │   │       crossbeam_epoch-70309bde7fc2cb9c.d
    │   │       crossbeam_utils-55eac9e19b413f19.d
    │   │       crypto_common-2473bc00cf73bd40.d
    │   │       data_encoding-b81b8fac16eb9244.d
    │   │       deranged-6ad2bd433f15ff3c.d
    │   │       digest-163e3169032ff52a.d
    │   │       displaydoc-e1cd28d5b7ec5a5c.d
    │   │       displaydoc-e1cd28d5b7ec5a5c.dll
    │   │       displaydoc-e1cd28d5b7ec5a5c.dll.exp
    │   │       displaydoc-e1cd28d5b7ec5a5c.dll.lib
    │   │       displaydoc-e1cd28d5b7ec5a5c.pdb
    │   │       either-40e003cf640b5063.d
    │   │       encoding_rs-336d7acf3906a098.d
    │   │       equivalent-3ce44829f870ed83.d
    │   │       fastrand-07c470db7b609c4b.d
    │   │       filetime-2e964a7bd4098a99.d
    │   │       flate2-2bc9d9e2017fa912.d
    │   │       fnv-54f892b845cf2ec0.d
    │   │       form_urlencoded-11ea3024e3dea1f4.d
    │   │       fs_extra-1c00be5779abf281.d
    │   │       fs_extra-3d017a29d568cf1b.d
    │   │       futures-96962199174e0217.d
    │   │       futures_channel-798cbad065ecdf4e.d
    │   │       futures_core-71c49047742881a2.d
    │   │       futures_executor-879af228033e8e60.d
    │   │       futures_io-3435e85ac09014cd.d
    │   │       futures_macro-7359decbc5abd909.d
    │   │       futures_macro-7359decbc5abd909.dll
    │   │       futures_macro-7359decbc5abd909.dll.exp
    │   │       futures_macro-7359decbc5abd909.dll.lib
    │   │       futures_macro-7359decbc5abd909.pdb
    │   │       futures_sink-71cc1a12a67e1dc1.d
    │   │       futures_task-f082e4f446b7c4b1.d
    │   │       futures_util-5e4f3538b4efff3d.d
    │   │       generic_array-2d66856823516b2e.d
    │   │       getrandom-934b1dc09303d6c9.d
    │   │       glob-df36f6ca54e80cd6.d
    │   │       h2-1a2deab62510e8a5.d
    │   │       hashbrown-ce0d3f688599f137.d
    │   │       http-ba7a0be6ad9f96f7.d
    │   │       httparse-5218a787346b7095.d
    │   │       httpdate-a9b4396cf5708d3a.d
    │   │       http_body-75aaca9760ee5014.d
    │   │       http_body_util-e79668cfbcbc86c0.d
    │   │       hyper-14c41c7e95e1c689.d
    │   │       hyper_tls-baa8cb3ffd3a6f60.d
    │   │       hyper_util-19f64b530b5b88d2.d
    │   │       icu_collections-8a07ef4b5bc3e9be.d
    │   │       icu_locale_core-ed2c547bfbb51fb3.d
    │   │       icu_normalizer-3da369b65bdb0ee4.d
    │   │       icu_normalizer_data-6d51904dee78038d.d
    │   │       icu_properties-3a86f6a3a2b79904.d
    │   │       icu_properties_data-0ad32791a5f1c294.d
    │   │       icu_provider-3991b2007ebb8514.d
    │   │       idna-115608ccbac85c49.d
    │   │       idna_adapter-fde30a62a7cbd8c5.d
    │   │       indexmap-22087521c20af0ea.d
    │   │       ipnet-94ee01a5d2cc0de5.d
    │   │       iri_string-f8b07435f68655e5.d
    │   │       itoa-8b2ac52ae81e4d36.d
    │   │       lazy_static-750ea293523c32be.d
    │   │       libadler2-a91714e349360340.rlib
    │   │       libadler2-a91714e349360340.rmeta
    │   │       libaiav_core-a337b5898fdf3f72.rlib
    │   │       libaiav_core-a337b5898fdf3f72.rmeta
    │   │       libanyhow-ae8fdb3917046c53.rlib
    │   │       libanyhow-ae8fdb3917046c53.rmeta
    │   │       libasync_compression-29bb534d98ac5f73.rlib
    │   │       libasync_compression-29bb534d98ac5f73.rmeta
    │   │       libatomic_waker-f5d65327110a6e9f.rlib
    │   │       libatomic_waker-f5d65327110a6e9f.rmeta
    │   │       libautocfg-61ec581c137d3022.rlib
    │   │       libautocfg-61ec581c137d3022.rmeta
    │   │       libaxum-cd81b020da7cbb0a.rlib
    │   │       libaxum-cd81b020da7cbb0a.rmeta
    │   │       libaxum_core-8999a5c6ee71991b.rlib
    │   │       libaxum_core-8999a5c6ee71991b.rmeta
    │   │       libbase64-e24b524e93fa5c62.rlib
    │   │       libbase64-e24b524e93fa5c62.rmeta
    │   │       libbitflags-007785627f569cda.rlib
    │   │       libbitflags-007785627f569cda.rmeta
    │   │       libblock_buffer-1a46078d05cfc6f8.rlib
    │   │       libblock_buffer-1a46078d05cfc6f8.rmeta
    │   │       libbytes-0aa7efc2e977500e.rlib
    │   │       libbytes-0aa7efc2e977500e.rmeta
    │   │       libc-e32e016b50ea0291.d
    │   │       libcc-7f94fdebce48ffa8.rlib
    │   │       libcc-7f94fdebce48ffa8.rmeta
    │   │       libcfg_if-c6d0dda5c7284520.rlib
    │   │       libcfg_if-c6d0dda5c7284520.rmeta
    │   │       libchrono-a7b24d5960f726dc.rlib
    │   │       libchrono-a7b24d5960f726dc.rmeta
    │   │       libcpufeatures-1ce9d5c3cb3cbca2.rlib
    │   │       libcpufeatures-1ce9d5c3cb3cbca2.rmeta
    │   │       libcrc32fast-05075484bef76adb.rlib
    │   │       libcrc32fast-05075484bef76adb.rmeta
    │   │       libcrossbeam_channel-7bfbdbff6e091de2.rlib
    │   │       libcrossbeam_channel-7bfbdbff6e091de2.rmeta
    │   │       libcrossbeam_deque-455d67003937737b.rlib
    │   │       libcrossbeam_deque-455d67003937737b.rmeta
    │   │       libcrossbeam_epoch-70309bde7fc2cb9c.rlib
    │   │       libcrossbeam_epoch-70309bde7fc2cb9c.rmeta
    │   │       libcrossbeam_utils-55eac9e19b413f19.rlib
    │   │       libcrossbeam_utils-55eac9e19b413f19.rmeta
    │   │       libcrypto_common-2473bc00cf73bd40.rlib
    │   │       libcrypto_common-2473bc00cf73bd40.rmeta
    │   │       libdata_encoding-b81b8fac16eb9244.rlib
    │   │       libdata_encoding-b81b8fac16eb9244.rmeta
    │   │       libderanged-6ad2bd433f15ff3c.rlib
    │   │       libderanged-6ad2bd433f15ff3c.rmeta
    │   │       libdigest-163e3169032ff52a.rlib
    │   │       libdigest-163e3169032ff52a.rmeta
    │   │       libeither-40e003cf640b5063.rlib
    │   │       libeither-40e003cf640b5063.rmeta
    │   │       libencoding_rs-336d7acf3906a098.rlib
    │   │       libencoding_rs-336d7acf3906a098.rmeta
    │   │       libequivalent-3ce44829f870ed83.rlib
    │   │       libequivalent-3ce44829f870ed83.rmeta
    │   │       libfastrand-07c470db7b609c4b.rlib
    │   │       libfastrand-07c470db7b609c4b.rmeta
    │   │       libfiletime-2e964a7bd4098a99.rlib
    │   │       libfiletime-2e964a7bd4098a99.rmeta
    │   │       libflate2-2bc9d9e2017fa912.rlib
    │   │       libflate2-2bc9d9e2017fa912.rmeta
    │   │       libfnv-54f892b845cf2ec0.rlib
    │   │       libfnv-54f892b845cf2ec0.rmeta
    │   │       libform_urlencoded-11ea3024e3dea1f4.rlib
    │   │       libform_urlencoded-11ea3024e3dea1f4.rmeta
    │   │       libfs_extra-1c00be5779abf281.rlib
    │   │       libfs_extra-1c00be5779abf281.rmeta
    │   │       libfs_extra-3d017a29d568cf1b.rlib
    │   │       libfs_extra-3d017a29d568cf1b.rmeta
    │   │       libfutures-96962199174e0217.rlib
    │   │       libfutures-96962199174e0217.rmeta
    │   │       libfutures_channel-798cbad065ecdf4e.rlib
    │   │       libfutures_channel-798cbad065ecdf4e.rmeta
    │   │       libfutures_core-71c49047742881a2.rlib
    │   │       libfutures_core-71c49047742881a2.rmeta
    │   │       libfutures_executor-879af228033e8e60.rlib
    │   │       libfutures_executor-879af228033e8e60.rmeta
    │   │       libfutures_io-3435e85ac09014cd.rlib
    │   │       libfutures_io-3435e85ac09014cd.rmeta
    │   │       libfutures_sink-71cc1a12a67e1dc1.rlib
    │   │       libfutures_sink-71cc1a12a67e1dc1.rmeta
    │   │       libfutures_task-f082e4f446b7c4b1.rlib
    │   │       libfutures_task-f082e4f446b7c4b1.rmeta
    │   │       libfutures_util-5e4f3538b4efff3d.rlib
    │   │       libfutures_util-5e4f3538b4efff3d.rmeta
    │   │       libgeneric_array-2d66856823516b2e.rlib
    │   │       libgeneric_array-2d66856823516b2e.rmeta
    │   │       libgetrandom-934b1dc09303d6c9.rlib
    │   │       libgetrandom-934b1dc09303d6c9.rmeta
    │   │       libglob-df36f6ca54e80cd6.rlib
    │   │       libglob-df36f6ca54e80cd6.rmeta
    │   │       libh2-1a2deab62510e8a5.rlib
    │   │       libh2-1a2deab62510e8a5.rmeta
    │   │       libhashbrown-ce0d3f688599f137.rlib
    │   │       libhashbrown-ce0d3f688599f137.rmeta
    │   │       libhttp-ba7a0be6ad9f96f7.rlib
    │   │       libhttp-ba7a0be6ad9f96f7.rmeta
    │   │       libhttparse-5218a787346b7095.rlib
    │   │       libhttparse-5218a787346b7095.rmeta
    │   │       libhttpdate-a9b4396cf5708d3a.rlib
    │   │       libhttpdate-a9b4396cf5708d3a.rmeta
    │   │       libhttp_body-75aaca9760ee5014.rlib
    │   │       libhttp_body-75aaca9760ee5014.rmeta
    │   │       libhttp_body_util-e79668cfbcbc86c0.rlib
    │   │       libhttp_body_util-e79668cfbcbc86c0.rmeta
    │   │       libhyper-14c41c7e95e1c689.rlib
    │   │       libhyper-14c41c7e95e1c689.rmeta
    │   │       libhyper_tls-baa8cb3ffd3a6f60.rlib
    │   │       libhyper_tls-baa8cb3ffd3a6f60.rmeta
    │   │       libhyper_util-19f64b530b5b88d2.rlib
    │   │       libhyper_util-19f64b530b5b88d2.rmeta
    │   │       libicu_collections-8a07ef4b5bc3e9be.rlib
    │   │       libicu_collections-8a07ef4b5bc3e9be.rmeta
    │   │       libicu_locale_core-ed2c547bfbb51fb3.rlib
    │   │       libicu_locale_core-ed2c547bfbb51fb3.rmeta
    │   │       libicu_normalizer-3da369b65bdb0ee4.rlib
    │   │       libicu_normalizer-3da369b65bdb0ee4.rmeta
    │   │       libicu_normalizer_data-6d51904dee78038d.rlib
    │   │       libicu_normalizer_data-6d51904dee78038d.rmeta
    │   │       libicu_properties-3a86f6a3a2b79904.rlib
    │   │       libicu_properties-3a86f6a3a2b79904.rmeta
    │   │       libicu_properties_data-0ad32791a5f1c294.rlib
    │   │       libicu_properties_data-0ad32791a5f1c294.rmeta
    │   │       libicu_provider-3991b2007ebb8514.rlib
    │   │       libicu_provider-3991b2007ebb8514.rmeta
    │   │       libidna-115608ccbac85c49.rlib
    │   │       libidna-115608ccbac85c49.rmeta
    │   │       libidna_adapter-fde30a62a7cbd8c5.rlib
    │   │       libidna_adapter-fde30a62a7cbd8c5.rmeta
    │   │       libindexmap-22087521c20af0ea.rlib
    │   │       libindexmap-22087521c20af0ea.rmeta
    │   │       libipnet-94ee01a5d2cc0de5.rlib
    │   │       libipnet-94ee01a5d2cc0de5.rmeta
    │   │       libiri_string-f8b07435f68655e5.rlib
    │   │       libiri_string-f8b07435f68655e5.rmeta
    │   │       libitoa-8b2ac52ae81e4d36.rlib
    │   │       libitoa-8b2ac52ae81e4d36.rmeta
    │   │       liblazy_static-750ea293523c32be.rlib
    │   │       liblazy_static-750ea293523c32be.rmeta
    │   │       liblibc-e32e016b50ea0291.rlib
    │   │       liblibc-e32e016b50ea0291.rmeta
    │   │       liblitemap-79dcf64bb5ca3df6.rlib
    │   │       liblitemap-79dcf64bb5ca3df6.rmeta
    │   │       liblock_api-be4fb932db2d767a.rlib
    │   │       liblock_api-be4fb932db2d767a.rmeta
    │   │       liblog-2497ab1b7cc93e09.rlib
    │   │       liblog-2497ab1b7cc93e09.rmeta
    │   │       libmatchers-dc8f6c10d305fb26.rlib
    │   │       libmatchers-dc8f6c10d305fb26.rmeta
    │   │       libmatchit-c1c7c2af364eaf4e.rlib
    │   │       libmatchit-c1c7c2af364eaf4e.rmeta
    │   │       libmemchr-9fbfe290db6c3667.rlib
    │   │       libmemchr-9fbfe290db6c3667.rmeta
    │   │       libmime-f9f1f4fa98a806fe.rlib
    │   │       libmime-f9f1f4fa98a806fe.rmeta
    │   │       libminiz_oxide-08d57ffd8421ae71.rlib
    │   │       libminiz_oxide-08d57ffd8421ae71.rmeta
    │   │       libmio-c13e74388dda844e.rlib
    │   │       libmio-c13e74388dda844e.rmeta
    │   │       libnative_tls-f4a9bae321da6386.rlib
    │   │       libnative_tls-f4a9bae321da6386.rmeta
    │   │       libnotify-024f05088ffcc172.rlib
    │   │       libnotify-024f05088ffcc172.rmeta
    │   │       libnotify-d426d512ee2486fb.rlib
    │   │       libnotify-d426d512ee2486fb.rmeta
    │   │       libnotify_types-de4748f95dbaa80a.rlib
    │   │       libnotify_types-de4748f95dbaa80a.rmeta
    │   │       libnum_conv-2f62ad16c9a619c7.rlib
    │   │       libnum_conv-2f62ad16c9a619c7.rmeta
    │   │       libnum_traits-625f23918a0338c4.rlib
    │   │       libnum_traits-625f23918a0338c4.rmeta
    │   │       libnu_ansi_term-46c8b5a72b74a1d8.rlib
    │   │       libnu_ansi_term-46c8b5a72b74a1d8.rmeta
    │   │       libonce_cell-a95a06a88addb51d.rlib
    │   │       libonce_cell-a95a06a88addb51d.rmeta
    │   │       liboverload-80fa2aa9b2d958d6.rlib
    │   │       liboverload-80fa2aa9b2d958d6.rmeta
    │   │       libparking_lot-1eaa1adcecab2ade.rlib
    │   │       libparking_lot-1eaa1adcecab2ade.rmeta
    │   │       libparking_lot_core-7dab19c47cc17e84.rlib
    │   │       libparking_lot_core-7dab19c47cc17e84.rmeta
    │   │       libpercent_encoding-fdf869015c3cb7aa.rlib
    │   │       libpercent_encoding-fdf869015c3cb7aa.rmeta
    │   │       libpin_project_lite-e16f2974c4bc55f8.rlib
    │   │       libpin_project_lite-e16f2974c4bc55f8.rmeta
    │   │       libpin_utils-06448c9b18db7896.rlib
    │   │       libpin_utils-06448c9b18db7896.rmeta
    │   │       libpotential_utf-26fe591d16104503.rlib
    │   │       libpotential_utf-26fe591d16104503.rmeta
    │   │       libpowerfmt-97762f50161a5e65.rlib
    │   │       libpowerfmt-97762f50161a5e65.rmeta
    │   │       libppv_lite86-d5fce4fbbbbd65aa.rlib
    │   │       libppv_lite86-d5fce4fbbbbd65aa.rmeta
    │   │       libproc_macro2-42fca9d61906e7ae.rlib
    │   │       libproc_macro2-42fca9d61906e7ae.rmeta
    │   │       libquote-dcab23192f5e54b0.rlib
    │   │       libquote-dcab23192f5e54b0.rmeta
    │   │       librand-fbe3eaaeacb62fc3.rlib
    │   │       librand-fbe3eaaeacb62fc3.rmeta
    │   │       librand_chacha-d92d2f7340e3661b.rlib
    │   │       librand_chacha-d92d2f7340e3661b.rmeta
    │   │       librand_core-4cbe42c1a5d68020.rlib
    │   │       librand_core-4cbe42c1a5d68020.rmeta
    │   │       librayon-efb084663cd91c8d.rlib
    │   │       librayon-efb084663cd91c8d.rmeta
    │   │       librayon_core-a3b18bb1eeb6697d.rlib
    │   │       librayon_core-a3b18bb1eeb6697d.rmeta
    │   │       libregex-d693fea602dbe448.rlib
    │   │       libregex-d693fea602dbe448.rmeta
    │   │       libregex_automata-bd93915af68bcb9b.rlib
    │   │       libregex_automata-bd93915af68bcb9b.rmeta
    │   │       libregex_automata-c687f0b4254dbe7f.rlib
    │   │       libregex_automata-c687f0b4254dbe7f.rmeta
    │   │       libregex_syntax-15d13742d5118ba4.rlib
    │   │       libregex_syntax-15d13742d5118ba4.rmeta
    │   │       libregex_syntax-b4c487db393429b1.rlib
    │   │       libregex_syntax-b4c487db393429b1.rmeta
    │   │       libreqwest-66700b6e7c8d2798.rlib
    │   │       libreqwest-66700b6e7c8d2798.rmeta
    │   │       librustls_pki_types-7a8b2040bfae87f7.rlib
    │   │       librustls_pki_types-7a8b2040bfae87f7.rmeta
    │   │       libryu-82697acd6f72601a.rlib
    │   │       libryu-82697acd6f72601a.rmeta
    │   │       libsame_file-b632a472373cc769.rlib
    │   │       libsame_file-b632a472373cc769.rmeta
    │   │       libschannel-2105f33340163e46.rlib
    │   │       libschannel-2105f33340163e46.rmeta
    │   │       libscopeguard-921be4800d80aec2.rlib
    │   │       libscopeguard-921be4800d80aec2.rmeta
    │   │       libserde-f31d0f5c9df33671.rlib
    │   │       libserde-f31d0f5c9df33671.rmeta
    │   │       libserde_json-7f8da8b252fdf290.rlib
    │   │       libserde_json-7f8da8b252fdf290.rmeta
    │   │       libserde_path_to_error-643790b17c8c51f1.rlib
    │   │       libserde_path_to_error-643790b17c8c51f1.rmeta
    │   │       libserde_spanned-8dc007a025c1fbbf.rlib
    │   │       libserde_spanned-8dc007a025c1fbbf.rmeta
    │   │       libserde_urlencoded-caf52712984e37a4.rlib
    │   │       libserde_urlencoded-caf52712984e37a4.rmeta
    │   │       libsha1-fea51e311bf177da.rlib
    │   │       libsha1-fea51e311bf177da.rmeta
    │   │       libsha2-d6e0805e01c7925b.rlib
    │   │       libsha2-d6e0805e01c7925b.rmeta
    │   │       libsharded_slab-14c29f69614d1917.rlib
    │   │       libsharded_slab-14c29f69614d1917.rmeta
    │   │       libshlex-35ab51f09a2e0161.rlib
    │   │       libshlex-35ab51f09a2e0161.rmeta
    │   │       libslab-e89cced06ddf8236.rlib
    │   │       libslab-e89cced06ddf8236.rmeta
    │   │       libsmallvec-fb3eec18a0274313.rlib
    │   │       libsmallvec-fb3eec18a0274313.rmeta
    │   │       libsocket2-50a226a339bf00ae.rlib
    │   │       libsocket2-50a226a339bf00ae.rmeta
    │   │       libstable_deref_trait-0cddb107f804f61d.rlib
    │   │       libstable_deref_trait-0cddb107f804f61d.rmeta
    │   │       libsyn-5c069c3175d43129.rlib
    │   │       libsyn-5c069c3175d43129.rmeta
    │   │       libsync_wrapper-d46e05c922eb68bc.rlib
    │   │       libsync_wrapper-d46e05c922eb68bc.rmeta
    │   │       libsynstructure-31a6b0306b685eb8.rlib
    │   │       libsynstructure-31a6b0306b685eb8.rmeta
    │   │       libsystem_integration-aeb8c93e088097c2.rlib
    │   │       libsystem_integration-aeb8c93e088097c2.rmeta
    │   │       libtempfile-732618c10a6c61fe.rlib
    │   │       libtempfile-732618c10a6c61fe.rmeta
    │   │       libthiserror-675460888fa41240.rlib
    │   │       libthiserror-675460888fa41240.rmeta
    │   │       libthiserror-b93595d23c9cfbf2.rlib
    │   │       libthiserror-b93595d23c9cfbf2.rmeta
    │   │       libthread_local-61bc988c23e04bb4.rlib
    │   │       libthread_local-61bc988c23e04bb4.rmeta
    │   │       libtime-e3ce1df07e77717a.rlib
    │   │       libtime-e3ce1df07e77717a.rmeta
    │   │       libtime_core-db7f11a7c1b4ab87.rlib
    │   │       libtime_core-db7f11a7c1b4ab87.rmeta
    │   │       libtime_fmt-1aa1696dda24cbca.rlib
    │   │       libtime_fmt-1aa1696dda24cbca.rmeta
    │   │       libtinystr-3c1d36b832f0bd76.rlib
    │   │       libtinystr-3c1d36b832f0bd76.rmeta
    │   │       libtokio-a1fa662547faa898.rlib
    │   │       libtokio-a1fa662547faa898.rmeta
    │   │       libtokio_native_tls-a940531aa8771407.rlib
    │   │       libtokio_native_tls-a940531aa8771407.rmeta
    │   │       libtokio_tungstenite-70a72e8465b3b968.rlib
    │   │       libtokio_tungstenite-70a72e8465b3b968.rmeta
    │   │       libtokio_util-22c8505954a72cfa.rlib
    │   │       libtokio_util-22c8505954a72cfa.rmeta
    │   │       libtoml-e81de2c71e72c590.rlib
    │   │       libtoml-e81de2c71e72c590.rmeta
    │   │       libtoml_datetime-50934ea1be4f1f9b.rlib
    │   │       libtoml_datetime-50934ea1be4f1f9b.rmeta
    │   │       libtoml_edit-37c31486875479f2.rlib
    │   │       libtoml_edit-37c31486875479f2.rmeta
    │   │       libtoml_write-fe3d6e3726091bae.rlib
    │   │       libtoml_write-fe3d6e3726091bae.rmeta
    │   │       libtower-6eca708608896875.rlib
    │   │       libtower-6eca708608896875.rmeta
    │   │       libtower_http-d4c8e384623ec765.rlib
    │   │       libtower_http-d4c8e384623ec765.rmeta
    │   │       libtower_layer-a229a13a7dcf6942.rlib
    │   │       libtower_layer-a229a13a7dcf6942.rmeta
    │   │       libtower_service-f0756d2210fe710d.rlib
    │   │       libtower_service-f0756d2210fe710d.rmeta
    │   │       libtracing-0f675fd6e47c0c47.rlib
    │   │       libtracing-0f675fd6e47c0c47.rmeta
    │   │       libtracing_appender-aee8c81779e5994f.rlib
    │   │       libtracing_appender-aee8c81779e5994f.rmeta
    │   │       libtracing_core-14bc279be94be20d.rlib
    │   │       libtracing_core-14bc279be94be20d.rmeta
    │   │       libtracing_log-dd7b0e85d053cfdd.rlib
    │   │       libtracing_log-dd7b0e85d053cfdd.rmeta
    │   │       libtracing_serde-5b30069d68f97efc.rlib
    │   │       libtracing_serde-5b30069d68f97efc.rmeta
    │   │       libtracing_subscriber-186fa1093e61f8d1.rlib
    │   │       libtracing_subscriber-186fa1093e61f8d1.rmeta
    │   │       libtry_lock-c4119e0a5cc7a76e.rlib
    │   │       libtry_lock-c4119e0a5cc7a76e.rmeta
    │   │       libtungstenite-da8492dc2925ba7d.rlib
    │   │       libtungstenite-da8492dc2925ba7d.rmeta
    │   │       libtypenum-c814792aa62bf50e.rlib
    │   │       libtypenum-c814792aa62bf50e.rmeta
    │   │       libunicode_ident-f40376275c848f02.rlib
    │   │       libunicode_ident-f40376275c848f02.rmeta
    │   │       liburl-fce2138162876c2b.rlib
    │   │       liburl-fce2138162876c2b.rmeta
    │   │       libutf8-b7d75fb04535868b.rlib
    │   │       libutf8-b7d75fb04535868b.rmeta
    │   │       libutf8_iter-3ef5bd26c581d05f.rlib
    │   │       libutf8_iter-3ef5bd26c581d05f.rmeta
    │   │       libuuid-bc1120679cd55c97.rlib
    │   │       libuuid-bc1120679cd55c97.rmeta
    │   │       libversion_check-ecee75b39792c45e.rlib
    │   │       libversion_check-ecee75b39792c45e.rmeta
    │   │       libwalkdir-72372ca8b4d326ed.rlib
    │   │       libwalkdir-72372ca8b4d326ed.rmeta
    │   │       libwant-243dfde86f45217c.rlib
    │   │       libwant-243dfde86f45217c.rmeta
    │   │       libwinapi-205ebb9e8d63ccbc.rlib
    │   │       libwinapi-205ebb9e8d63ccbc.rmeta
    │   │       libwinapi_util-80c3cee3fd87fad0.rlib
    │   │       libwinapi_util-80c3cee3fd87fad0.rmeta
    │   │       libwindows_link-91a052856f9bb0ec.rlib
    │   │       libwindows_link-91a052856f9bb0ec.rmeta
    │   │       libwindows_registry-ada8148771a4d13d.rlib
    │   │       libwindows_registry-ada8148771a4d13d.rmeta
    │   │       libwindows_result-d7e9129cc1a0bd63.rlib
    │   │       libwindows_result-d7e9129cc1a0bd63.rmeta
    │   │       libwindows_strings-35eaef3ad4595c32.rlib
    │   │       libwindows_strings-35eaef3ad4595c32.rmeta
    │   │       libwindows_sys-2543d0a0a5185d71.rlib
    │   │       libwindows_sys-2543d0a0a5185d71.rmeta
    │   │       libwindows_sys-41da0970ac15e060.rlib
    │   │       libwindows_sys-41da0970ac15e060.rmeta
    │   │       libwindows_sys-933e89c96c921385.rlib
    │   │       libwindows_sys-933e89c96c921385.rmeta
    │   │       libwindows_targets-542b06f200ae9feb.rlib
    │   │       libwindows_targets-542b06f200ae9feb.rmeta
    │   │       libwindows_targets-7450d71b090de8af.rlib
    │   │       libwindows_targets-7450d71b090de8af.rmeta
    │   │       libwindows_x86_64_msvc-657020c39551da92.rlib
    │   │       libwindows_x86_64_msvc-657020c39551da92.rmeta
    │   │       libwindows_x86_64_msvc-b91860b336b577ee.rlib
    │   │       libwindows_x86_64_msvc-b91860b336b577ee.rmeta
    │   │       libwinnow-f89320763f898ab0.rlib
    │   │       libwinnow-f89320763f898ab0.rmeta
    │   │       libwriteable-8b72f77cdd141252.rlib
    │   │       libwriteable-8b72f77cdd141252.rmeta
    │   │       libyara-1a2fb30b47c6b51c.rlib
    │   │       libyara-1a2fb30b47c6b51c.rmeta
    │   │       libyara_sys-1cfc1fbe1feeb3bd.rlib
    │   │       libyara_sys-1cfc1fbe1feeb3bd.rmeta
    │   │       libyoke-f5163d48fee778ca.rlib
    │   │       libyoke-f5163d48fee778ca.rmeta
    │   │       libzerocopy-9a03daf9864ef42e.rlib
    │   │       libzerocopy-9a03daf9864ef42e.rmeta
    │   │       libzerofrom-d1e410de713137b5.rlib
    │   │       libzerofrom-d1e410de713137b5.rmeta
    │   │       libzeroize-a98d846932453112.rlib
    │   │       libzeroize-a98d846932453112.rmeta
    │   │       libzerotrie-321dd49dfd22b335.rlib
    │   │       libzerotrie-321dd49dfd22b335.rmeta
    │   │       libzerovec-5a822c4bb4ec249d.rlib
    │   │       libzerovec-5a822c4bb4ec249d.rmeta
    │   │       litemap-79dcf64bb5ca3df6.d
    │   │       lock_api-be4fb932db2d767a.d
    │   │       log-2497ab1b7cc93e09.d
    │   │       matchers-dc8f6c10d305fb26.d
    │   │       matchit-c1c7c2af364eaf4e.d
    │   │       memchr-9fbfe290db6c3667.d
    │   │       mime-f9f1f4fa98a806fe.d
    │   │       miniz_oxide-08d57ffd8421ae71.d
    │   │       mio-c13e74388dda844e.d
    │   │       native_tls-f4a9bae321da6386.d
    │   │       notify-024f05088ffcc172.d
    │   │       notify-d426d512ee2486fb.d
    │   │       notify_types-de4748f95dbaa80a.d
    │   │       num_conv-2f62ad16c9a619c7.d
    │   │       num_traits-625f23918a0338c4.d
    │   │       nu_ansi_term-46c8b5a72b74a1d8.d
    │   │       once_cell-a95a06a88addb51d.d
    │   │       overload-80fa2aa9b2d958d6.d
    │   │       parking_lot-1eaa1adcecab2ade.d
    │   │       parking_lot_core-7dab19c47cc17e84.d
    │   │       percent_encoding-fdf869015c3cb7aa.d
    │   │       pin_project_lite-e16f2974c4bc55f8.d
    │   │       pin_utils-06448c9b18db7896.d
    │   │       potential_utf-26fe591d16104503.d
    │   │       powerfmt-97762f50161a5e65.d
    │   │       ppv_lite86-d5fce4fbbbbd65aa.d
    │   │       proc_macro2-42fca9d61906e7ae.d
    │   │       quote-dcab23192f5e54b0.d
    │   │       rand-fbe3eaaeacb62fc3.d
    │   │       rand_chacha-d92d2f7340e3661b.d
    │   │       rand_core-4cbe42c1a5d68020.d
    │   │       rayon-efb084663cd91c8d.d
    │   │       rayon_core-a3b18bb1eeb6697d.d
    │   │       regex-d693fea602dbe448.d
    │   │       regex_automata-bd93915af68bcb9b.d
    │   │       regex_automata-c687f0b4254dbe7f.d
    │   │       regex_syntax-15d13742d5118ba4.d
    │   │       regex_syntax-b4c487db393429b1.d
    │   │       reqwest-66700b6e7c8d2798.d
    │   │       rustls_pki_types-7a8b2040bfae87f7.d
    │   │       rustversion-42883a32881fbff9.d
    │   │       rustversion-42883a32881fbff9.dll
    │   │       rustversion-42883a32881fbff9.dll.exp
    │   │       rustversion-42883a32881fbff9.dll.lib
    │   │       rustversion-42883a32881fbff9.pdb
    │   │       ryu-82697acd6f72601a.d
    │   │       same_file-b632a472373cc769.d
    │   │       schannel-2105f33340163e46.d
    │   │       scopeguard-921be4800d80aec2.d
    │   │       serde-f31d0f5c9df33671.d
    │   │       serde_derive-249edb18d4123132.d
    │   │       serde_derive-249edb18d4123132.dll
    │   │       serde_derive-249edb18d4123132.dll.exp
    │   │       serde_derive-249edb18d4123132.dll.lib
    │   │       serde_derive-249edb18d4123132.pdb
    │   │       serde_json-7f8da8b252fdf290.d
    │   │       serde_path_to_error-643790b17c8c51f1.d
    │   │       serde_spanned-8dc007a025c1fbbf.d
    │   │       serde_urlencoded-caf52712984e37a4.d
    │   │       sha1-fea51e311bf177da.d
    │   │       sha2-d6e0805e01c7925b.d
    │   │       sharded_slab-14c29f69614d1917.d
    │   │       shlex-35ab51f09a2e0161.d
    │   │       slab-e89cced06ddf8236.d
    │   │       smallvec-fb3eec18a0274313.d
    │   │       socket2-50a226a339bf00ae.d
    │   │       stable_deref_trait-0cddb107f804f61d.d
    │   │       syn-5c069c3175d43129.d
    │   │       sync_wrapper-d46e05c922eb68bc.d
    │   │       synstructure-31a6b0306b685eb8.d
    │   │       system_integration-aeb8c93e088097c2.d
    │   │       tempfile-732618c10a6c61fe.d
    │   │       thiserror-675460888fa41240.d
    │   │       thiserror-b93595d23c9cfbf2.d
    │   │       thiserror_impl-51959aa310fa4041.d
    │   │       thiserror_impl-51959aa310fa4041.dll
    │   │       thiserror_impl-51959aa310fa4041.dll.exp
    │   │       thiserror_impl-51959aa310fa4041.dll.lib
    │   │       thiserror_impl-51959aa310fa4041.pdb
    │   │       thiserror_impl-88b3cd32dd849905.d
    │   │       thiserror_impl-88b3cd32dd849905.dll
    │   │       thiserror_impl-88b3cd32dd849905.dll.exp
    │   │       thiserror_impl-88b3cd32dd849905.dll.lib
    │   │       thiserror_impl-88b3cd32dd849905.pdb
    │   │       thread_local-61bc988c23e04bb4.d
    │   │       time-e3ce1df07e77717a.d
    │   │       time_core-db7f11a7c1b4ab87.d
    │   │       time_fmt-1aa1696dda24cbca.d
    │   │       tinystr-3c1d36b832f0bd76.d
    │   │       tokio-a1fa662547faa898.d
    │   │       tokio_macros-2ae99dd1b20b3610.d
    │   │       tokio_macros-2ae99dd1b20b3610.dll
    │   │       tokio_macros-2ae99dd1b20b3610.dll.exp
    │   │       tokio_macros-2ae99dd1b20b3610.dll.lib
    │   │       tokio_macros-2ae99dd1b20b3610.pdb
    │   │       tokio_native_tls-a940531aa8771407.d
    │   │       tokio_tungstenite-70a72e8465b3b968.d
    │   │       tokio_util-22c8505954a72cfa.d
    │   │       toml-e81de2c71e72c590.d
    │   │       toml_datetime-50934ea1be4f1f9b.d
    │   │       toml_edit-37c31486875479f2.d
    │   │       toml_write-fe3d6e3726091bae.d
    │   │       tower-6eca708608896875.d
    │   │       tower_http-d4c8e384623ec765.d
    │   │       tower_layer-a229a13a7dcf6942.d
    │   │       tower_service-f0756d2210fe710d.d
    │   │       tracing-0f675fd6e47c0c47.d
    │   │       tracing_appender-aee8c81779e5994f.d
    │   │       tracing_attributes-91db32819f743128.d
    │   │       tracing_attributes-91db32819f743128.dll
    │   │       tracing_attributes-91db32819f743128.dll.exp
    │   │       tracing_attributes-91db32819f743128.dll.lib
    │   │       tracing_attributes-91db32819f743128.pdb
    │   │       tracing_core-14bc279be94be20d.d
    │   │       tracing_log-dd7b0e85d053cfdd.d
    │   │       tracing_serde-5b30069d68f97efc.d
    │   │       tracing_subscriber-186fa1093e61f8d1.d
    │   │       try_lock-c4119e0a5cc7a76e.d
    │   │       tungstenite-da8492dc2925ba7d.d
    │   │       typenum-c814792aa62bf50e.d
    │   │       unicode_ident-f40376275c848f02.d
    │   │       url-fce2138162876c2b.d
    │   │       utf8-b7d75fb04535868b.d
    │   │       utf8_iter-3ef5bd26c581d05f.d
    │   │       uuid-bc1120679cd55c97.d
    │   │       version_check-ecee75b39792c45e.d
    │   │       walkdir-72372ca8b4d326ed.d
    │   │       want-243dfde86f45217c.d
    │   │       winapi-205ebb9e8d63ccbc.d
    │   │       winapi_util-80c3cee3fd87fad0.d
    │   │       windows_link-91a052856f9bb0ec.d
    │   │       windows_registry-ada8148771a4d13d.d
    │   │       windows_result-d7e9129cc1a0bd63.d
    │   │       windows_strings-35eaef3ad4595c32.d
    │   │       windows_sys-2543d0a0a5185d71.d
    │   │       windows_sys-41da0970ac15e060.d
    │   │       windows_sys-933e89c96c921385.d
    │   │       windows_targets-542b06f200ae9feb.d
    │   │       windows_targets-7450d71b090de8af.d
    │   │       windows_x86_64_msvc-657020c39551da92.d
    │   │       windows_x86_64_msvc-b91860b336b577ee.d
    │   │       winnow-f89320763f898ab0.d
    │   │       writeable-8b72f77cdd141252.d
    │   │       yara-1a2fb30b47c6b51c.d
    │   │       yara_sys-1cfc1fbe1feeb3bd.d
    │   │       yoke-f5163d48fee778ca.d
    │   │       yoke_derive-3a898eed09391613.d
    │   │       yoke_derive-3a898eed09391613.dll
    │   │       yoke_derive-3a898eed09391613.dll.exp
    │   │       yoke_derive-3a898eed09391613.dll.lib
    │   │       yoke_derive-3a898eed09391613.pdb
    │   │       zerocopy-9a03daf9864ef42e.d
    │   │       zerofrom-d1e410de713137b5.d
    │   │       zerofrom_derive-af4d786be22bee75.d
    │   │       zerofrom_derive-af4d786be22bee75.dll
    │   │       zerofrom_derive-af4d786be22bee75.dll.exp
    │   │       zerofrom_derive-af4d786be22bee75.dll.lib
    │   │       zerofrom_derive-af4d786be22bee75.pdb
    │   │       zeroize-a98d846932453112.d
    │   │       zerotrie-321dd49dfd22b335.d
    │   │       zerovec-5a822c4bb4ec249d.d
    │   │       zerovec_derive-c6184e30a737d9f7.d
    │   │       zerovec_derive-c6184e30a737d9f7.dll
    │   │       zerovec_derive-c6184e30a737d9f7.dll.exp
    │   │       zerovec_derive-c6184e30a737d9f7.dll.lib
    │   │       zerovec_derive-c6184e30a737d9f7.pdb
    │   │
    │   ├───examples
    │   └───incremental
    └───rust-analyzer
        └───metadata
            ├───sysroot
            │       Cargo.lock
            │
            └───workspace
                    Cargo.lock