#include "dashboard_page.h"
#include "../api/api_client.h"
// DÜZELTME: Kullandığımız özel widget'ların başlık dosyalarını dahil ediyoruz.
#include "../widgets/shield_widget.h"
#include "../widgets/status_card_widget.h" 
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QTimer>
#include <QJsonObject>
#include <QDebug>

DashboardPage::DashboardPage(QWidget *parent) : QWidget(parent)
{
    // Arayüz kurulumu
    QVBoxLayout* mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(50, 30, 50, 30);
    mainLayout->setSpacing(20);
    mainLayout->setAlignment(Qt::AlignTop);

    QLabel* titleLabel = new QLabel("ANTIVIRUS", this);
    titleLabel->setObjectName("pageTitle");
    titleLabel->setAlignment(Qt::AlignCenter);

    m_shieldWidget = new ShieldWidget(this);
    
    QLabel* statusText = new QLabel("PC PROTECTED", this);
    statusText->setStyleSheet("font-size: 20px; font-weight: bold; color: #e0e1e9; letter-spacing: 2px;");
    statusText->setAlignment(Qt::AlignCenter);

    QHBoxLayout* cardsLayout = new QHBoxLayout();
    cardsLayout->setSpacing(20);
    m_securityCard = new StatusCardWidget("SECURITY", "No threats", this);
    m_updatesCard = new StatusCardWidget("UPDATES", "Up to date", this);
    m_scanCard = new StatusCardWidget("SCAN", "Last scan: N/A", this);
    cardsLayout->addWidget(m_securityCard);
    cardsLayout->addWidget(m_updatesCard);
    cardsLayout->addWidget(m_scanCard);
    
    m_quickScanButton = new QPushButton("RUN QUICK SCAN", this);
    m_quickScanButton->setObjectName("runQuickScanButton");
    m_quickScanButton->setMinimumHeight(50);

    mainLayout->addWidget(titleLabel);
    mainLayout->addWidget(m_shieldWidget, 1, Qt::AlignCenter);
    mainLayout->addWidget(statusText, 0, Qt::AlignCenter);
    mainLayout->addSpacing(30);
    mainLayout->addLayout(cardsLayout);
    mainLayout->addStretch();
    mainLayout->addWidget(m_quickScanButton, 0, Qt::AlignHCenter);

    // API istemcisi ve Timer kurulumu
    m_apiClient = new ApiClient(this);
    m_statusTimer = new QTimer(this);
    m_statusTimer->setInterval(3000);

    // Sinyal ve slot bağlantıları
    connect(m_statusTimer, &QTimer::timeout, this, &DashboardPage::fetchEngineStatus);
    connect(m_apiClient, &ApiClient::engineStatusReceived, this, &DashboardPage::onEngineStatusReceived);
    connect(m_quickScanButton, &QPushButton::clicked, this, &DashboardPage::onQuickScanClicked);

    // Başlangıç işlemleri
    m_statusTimer->start();
    fetchEngineStatus();
}

void DashboardPage::fetchEngineStatus()
{
    m_apiClient->getEngineStatus();
}

void DashboardPage::onQuickScanClicked()
{
    // Bu satırlar artık hata vermeyecektir.
    m_shieldWidget->setState(ShieldWidget::Scanning); 
    m_securityCard->setValueText("Scanning...");
    m_apiClient->startQuickScan();
}

void DashboardPage::onEngineStatusReceived(bool success, const QJsonDocument& data)
{
    if (!success || !data.isObject()) {
        m_shieldWidget->setState(ShieldWidget::AtRisk);
        m_securityCard->setValueText("Connection Error");
        return;
    }

    QJsonObject statusObj = data.object();
    bool is_scanning = statusObj["is_scanning"].toBool(false);
    qint64 threats_found = static_cast<qint64>(statusObj["threats_found"].toDouble(0));
    qint64 quarantined_count = static_cast<qint64>(statusObj["quarantined_count"].toDouble(0));

    // Neon Kalkan'ın durumunu güncelle
    if (is_scanning) {
        m_shieldWidget->setState(ShieldWidget::Scanning);
    } else if (threats_found > 0 || quarantined_count > 0) {
        m_shieldWidget->setState(ShieldWidget::AtRisk);
    } else {
        m_shieldWidget->setState(ShieldWidget::Secure);
    }

    // Durum kartlarını güncelle
    m_securityCard->setValueText(QString("%1 Threats Found").arg(threats_found));
}