#include "scan_page.h"
#include "../widgets/futuristic_dropzone.h"
#include "../api/api_client.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QTextEdit>
#include <QPushButton>
#include <QLabel>
#include <QFileDialog>
#include <QProgressBar>
#include <QFrame>
#include <QDir>
#include <QTimer>
#include <QDateTime>
#include <QFileInfo>
#include <QtWebSockets/QWebSocket>
#include <QJsonObject>
#include <QJsonDocument>

ScanPage::ScanPage(QWidget *parent) : QWidget(parent)
{
    setupUI();
    
    m_apiClient = new ApiClient(this);
    m_progressTimer = new QTimer(this);
    m_progressTimer->setInterval(500);
    m_logSocket = new QWebSocket();
    
    connectSignals();

    m_logSocket->open(QUrl("ws://127.0.0.1:3000/api/ws/logs"));
}

ScanPage::~ScanPage() {
    m_logSocket->close();
}

void ScanPage::setupUI() {
    QVBoxLayout* mainLayout = new QVBoxLayout(this);
    mainLayout->setSpacing(20);
    mainLayout->setContentsMargins(40, 30, 40, 30);
    
    QLabel* titleLabel = new QLabel("Manuel Analiz Portalı", this);
    titleLabel->setObjectName("pageTitle");
    QLabel* subtitleLabel = new QLabel("Dosyaları veya klasörleri analiz etmek için aşağıdaki portalı kullanın.", this);
    subtitleLabel->setObjectName("pageSubtitle");
    
    m_dropZone = new FuturisticDropZone(this);
    
    QHBoxLayout* buttonLayout = new QHBoxLayout();
    m_selectFolderButton = new QPushButton("Klasör Seç", this);
    m_quickScanButton = new QPushButton("Hızlı Tarama", this);
    m_quickScanButton->setObjectName("quickScanButton");
    buttonLayout->addWidget(m_selectFolderButton);
    buttonLayout->addWidget(m_quickScanButton);
    buttonLayout->addStretch();
    
    m_progressContainer = new QFrame(this);
    m_progressContainer->setObjectName("progressContainer");
    QVBoxLayout* progressLayout = new QVBoxLayout(m_progressContainer);
    m_statusLabel = new QLabel("Tarama durumu bekleniyor...", this);
    m_progressBar = new QProgressBar(this);
    m_progressBar->setRange(0, 100);
    m_progressBar->setValue(0);
    m_progressBar->setTextVisible(true);
    m_progressBar->setFormat("%p%");
    progressLayout->addWidget(m_statusLabel);
    progressLayout->addWidget(m_progressBar);
    m_progressContainer->hide();
    
    m_logOutput = new QTextEdit(this);
    m_logOutput->setReadOnly(true);
    m_logOutput->setObjectName("scanLog");
    m_logOutput->setMinimumHeight(150);

    mainLayout->addWidget(titleLabel, 0, Qt::AlignCenter);
    mainLayout->addWidget(subtitleLabel, 0, Qt::AlignCenter);
    mainLayout->addWidget(m_dropZone, 1);
    mainLayout->addLayout(buttonLayout);
    mainLayout->addWidget(m_progressContainer);
    mainLayout->addWidget(new QLabel("<h3>Canlı Aktivite ve Sonuçlar:</h3>"));
    mainLayout->addWidget(m_logOutput, 2);
}

void ScanPage::connectSignals() {
    connect(m_dropZone, &FuturisticDropZone::filesDropped, this, &ScanPage::onFilesDropped);
    connect(m_selectFolderButton, &QPushButton::clicked, this, &ScanPage::selectFolderToScan);
    connect(m_quickScanButton, &QPushButton::clicked, this, &ScanPage::performQuickScan);
    connect(m_apiClient, &ApiClient::scanRequestFinished, this, &ScanPage::onScanResult);
    connect(m_progressTimer, &QTimer::timeout, this, &ScanPage::updateScanProgress);
    connect(m_apiClient, &ApiClient::scanProgressReceived, this, &ScanPage::onScanProgressReceived);
    connect(m_logSocket, &QWebSocket::connected, this, &ScanPage::onLogSocketConnected);
    connect(m_logSocket, &QWebSocket::textMessageReceived, this, &ScanPage::onLogMessageReceived);
}

void ScanPage::startScanProcess(const QString& scanType) {
    m_logOutput->clear();
    m_logOutput->append(QString("[%1] %2 başlatılıyor...").arg(QDateTime::currentDateTime().toString("hh:mm:ss"), scanType));
    m_progressBar->setValue(0);
    m_statusLabel->setText("Tarama hazırlanıyor...");
    m_progressContainer->show();
    m_dropZone->setEnabled(false);
    m_selectFolderButton->setEnabled(false);
    m_quickScanButton->setEnabled(false);
    m_progressTimer->start();
}

void ScanPage::stopScanProcess() {
    m_progressTimer->stop();
    m_progressBar->setValue(100);
    m_statusLabel->setText("Tarama tamamlandı!");
    m_dropZone->setEnabled(true);
    m_selectFolderButton->setEnabled(true);
    m_quickScanButton->setEnabled(true);
    QTimer::singleShot(2000, [this]() { m_progressContainer->hide(); });
}

void ScanPage::onFilesDropped(const QList<QUrl>& fileUrls) {
    startScanProcess("Sürükle-Bırak Taraması");
    for (const QUrl &url : fileUrls) {
        if (!url.toLocalFile().isEmpty()) m_apiClient->startScan(url.toLocalFile());
    }
}

void ScanPage::selectFolderToScan() {
    QString path = QFileDialog::getExistingDirectory(this, "Taranacak Klasörü Seçin", QDir::homePath());
    if (!path.isEmpty()) {
        startScanProcess("Klasör Taraması");
        m_apiClient->startScan(path);
    }
}

void ScanPage::performQuickScan() {
    startScanProcess("Hızlı Sistem Taraması");
    m_apiClient->startQuickScan();
}

void ScanPage::onScanResult(bool success, const QString& message) {
    if(success){
        m_logOutput->append("-> ℹ️ Sunucu tarama isteğini kabul etti.");
    } else {
        m_logOutput->append(QString("-> ❌ API isteği gönderilirken hata oluştu: %1").arg(message));
    }
}

void ScanPage::updateScanProgress() {
    m_apiClient->getScanProgress();
}

void ScanPage::onScanProgressReceived(bool success, const QJsonDocument& data) {
    if (!success || !data.isObject()) {
        stopScanProcess();
        return;
    }
    QJsonObject progressObj = data.object();
    if (progressObj["is_running"].toBool()) {
        qint64 total = static_cast<qint64>(progressObj["total_files"].toDouble());
        qint64 scanned = static_cast<qint64>(progressObj["scanned_files"].toDouble());
        QString current_file = progressObj["current_file"].toString();
        m_statusLabel->setText(QString("Taranıyor: %1").arg(QFileInfo(current_file).fileName()));
        if (total > 0) {
            m_progressBar->setValue(static_cast<int>((scanned * 100) / total));
        }
    } else {
        stopScanProcess();
    }
}

void ScanPage::onLogSocketConnected() {
    m_logOutput->append("⚡ Canlı log akışına başarıyla bağlanıldı.");
}

void ScanPage::onLogMessageReceived(const QString &message) {
    m_logOutput->append(message.trimmed());
}