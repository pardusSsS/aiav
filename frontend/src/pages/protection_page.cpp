#include "protection_page.h"
#include "../api/api_client.h"
#include "../widgets/shield_widget.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QGridLayout>
#include <QLabel>
#include <QPushButton>
#include <QTextEdit>
#include <QCheckBox>
#include <QMessageBox>
#include <QJsonObject>
#include <QJsonArray>
#include <QStringList>
#include <QFrame>
#include <QTimer>

ProtectionPage::ProtectionPage(QWidget *parent) : QWidget(parent)
{
    setupUI();
    m_apiClient = new ApiClient(this);
    connectSignals();
    QTimer::singleShot(100, this, &ProtectionPage::loadCurrentSettings);
}

void ProtectionPage::setupUI()
{
    QVBoxLayout* mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(40, 30, 40, 30);
    mainLayout->setSpacing(15);

    // --- Başlık ve Kalkan ---
    QHBoxLayout* topLayout = new QHBoxLayout();
    m_shieldWidget = new ShieldWidget(this);
    m_shieldWidget->setFixedSize(120, 120);

    QVBoxLayout* titleLayout = new QVBoxLayout();
    QLabel* title = new QLabel("Akıllı Ağ Kalkanı", this);
    title->setObjectName("pageTitle");
    m_statusTextLabel = new QLabel("Ayarlar yükleniyor...", this);
    m_statusTextLabel->setObjectName("pageSubtitle");
    m_statusTextLabel->setStyleSheet("font-size: 18px; font-weight: bold;");
    titleLayout->addWidget(title);
    titleLayout->addWidget(m_statusTextLabel);
    
    topLayout->addWidget(m_shieldWidget);
    topLayout->addSpacing(20);
    topLayout->addLayout(titleLayout);
    topLayout->addStretch();
    mainLayout->addLayout(topLayout);
    mainLayout->addSpacing(20);

    // --- Otomatik Koruma Kategorileri ---
    QFrame* protectionGroup = new QFrame(this);
    protectionGroup->setObjectName("settingsGroup");
    QVBoxLayout* protectionLayout = new QVBoxLayout(protectionGroup);
    protectionLayout->setSpacing(15);
    protectionLayout->addWidget(new QLabel("<b>Otomatik Engelleme Kategorileri</b>"));
    m_malwareBlockToggle = new QCheckBox("Bilinen Zararlı Yazılım ve Oltalama Sitelerini Engelle", this);
    m_trackerBlockToggle = new QCheckBox("Reklam ve Aktivite Takipçilerini Engelle (Gizliliği Artırır)", this);
    protectionLayout->addWidget(m_malwareBlockToggle);
    protectionLayout->addWidget(m_trackerBlockToggle);
    mainLayout->addWidget(protectionGroup);

    // --- Manuel Listeler ---
    QFrame* manualGroup = new QFrame(this);
    manualGroup->setObjectName("settingsGroup");
    QGridLayout* gridLayout = new QGridLayout(manualGroup);
    gridLayout->setSpacing(20);
    
    gridLayout->addWidget(new QLabel("<b>İzin Verilenler (Whitelist)</b><br><span style='color:#a0a0c0;'>Engellenmeyecek siteleri her satıra bir tane yazın.</span>"), 0, 0);
    m_whitelistEdit = new QTextEdit(this);
    gridLayout->addWidget(m_whitelistEdit, 1, 0);

    gridLayout->addWidget(new QLabel("<b>Özel Engellenenler (Blacklist)</b><br><span style='color:#a0a0c0;'>Engellemek istediğiniz özel siteleri yazın.</span>"), 0, 1);
    m_blacklistEdit = new QTextEdit(this);
    gridLayout->addWidget(m_blacklistEdit, 1, 1);
    mainLayout->addWidget(manualGroup);

    mainLayout->addStretch();
    
    m_saveButton = new QPushButton("Ayarları Kaydet ve Korumayı Uygula", this);
    m_saveButton->setObjectName("runQuickScanButton");
    mainLayout->addWidget(m_saveButton, 0, Qt::AlignRight);
}

void ProtectionPage::connectSignals()
{
    connect(m_saveButton, &QPushButton::clicked, this, &ProtectionPage::onSaveClicked);
    connect(m_apiClient, &ApiClient::settingsReceived, this, &ProtectionPage::onSettingsReceived);
    connect(m_apiClient, &ApiClient::settingsSaved, this, &ProtectionPage::onSettingsSaved);
}


void ProtectionPage::loadCurrentSettings()
{
    this->setEnabled(false);
    m_statusTextLabel->setText("Mevcut ayarlar sunucudan yükleniyor...");
    m_apiClient->getSettings();
}

void ProtectionPage::onSettingsReceived(bool success, const QJsonDocument& jsonData)
{
    this->setEnabled(true);
    if (success && jsonData.isObject()) {
        m_currentSettings = jsonData.object();
        populateUI(m_currentSettings);
    } else {
        QMessageBox::warning(this, "Hata", "Mevcut ayarlar sunucudan yüklenemedi.");
        updateProtectionStatusVisuals(QJsonObject()); // Hata durumunda pasif göster
    }
}

void ProtectionPage::populateUI(const QJsonObject& settings)
{
    if (!settings.contains("web_security")) return;
    
    QJsonObject webSettings = settings["web_security"].toObject();
    
    m_trackerBlockToggle->setChecked(webSettings["enable_tracker_protection"].toBool(true));
    m_malwareBlockToggle->setChecked(webSettings["enable_malware_protection"].toBool(true));

    QStringList whitelist, blacklist;
    for (const auto& item : webSettings["user_whitelist"].toArray()) { whitelist.append(item.toString()); }
    for (const auto& item : webSettings["user_blacklist"].toArray()) { blacklist.append(item.toString()); }
    m_whitelistEdit->setPlainText(whitelist.join("\n"));
    m_blacklistEdit->setPlainText(blacklist.join("\n"));

    updateProtectionStatusVisuals(webSettings);
}

// YENİ: Kalkanın ve metnin görünümünü güncelleyen merkezi fonksiyon
void ProtectionPage::updateProtectionStatusVisuals(const QJsonObject& webSettings)
{
    bool malware_enabled = webSettings["enable_malware_protection"].toBool(false);
    bool tracker_enabled = webSettings["enable_tracker_protection"].toBool(false);
    bool isActive = malware_enabled || tracker_enabled; // Korumalardan en az biri aktifse, kalkan aktiftir
    
    if (isActive) {
        m_shieldWidget->setState(ShieldWidget::Secure);
        m_statusTextLabel->setText("Ağ Kalkanı AKTİF");
    } else {
        m_shieldWidget->setState(ShieldWidget::AtRisk);
        m_statusTextLabel->setText("Ağ Kalkanı PASİF");
    }
}

void ProtectionPage::onSaveClicked()
{
    m_saveButton->setEnabled(false);
    m_saveButton->setText("Kaydediliyor...");
    
    m_apiClient->saveSettings(QJsonDocument(gatherDataFromUI()));
}

QJsonObject ProtectionPage::gatherDataFromUI()
{
    QJsonObject newSettings = m_currentSettings;
    
    QJsonObject webSettings = newSettings["web_security"].toObject();
    webSettings["enable_tracker_protection"] = m_trackerBlockToggle->isChecked();
    webSettings["enable_malware_protection"] = m_malwareBlockToggle->isChecked();

    QStringList whitelist = m_whitelistEdit->toPlainText().split('\n', Qt::SkipEmptyParts);
    QStringList blacklist = m_blacklistEdit->toPlainText().split('\n', Qt::SkipEmptyParts);
    webSettings["user_whitelist"] = QJsonArray::fromStringList(whitelist);
    webSettings["user_blacklist"] = QJsonArray::fromStringList(blacklist);
    
    newSettings["web_security"] = webSettings;
    return newSettings;
}

void ProtectionPage::onSettingsSaved(bool success, const QString& message)
{
    m_saveButton->setEnabled(true);
    m_saveButton->setText("Ayarları Kaydet ve Korumayı Uygula");

    if (success) {
        QMessageBox::information(this, "Başarılı", "Ayarlar kaydedildi ve ağ koruması güncellendi.");
        loadCurrentSettings(); // Başarılı kayıttan sonra UI'ı yeniden yükle ve durumu doğrula
    } else {
        // --- GEÇİCİ DEBUG DEĞİŞİKLİĞİ ---
        // Backend'den gelen 'message' değişkenini doğrudan gösterelim.
        QString detailed_error_message = "Ayarlar kaydedilemedi.\n\nArka Plandan Gelen Gerçek Hata:\n------------------------------------\n" + message;
        QMessageBox::critical(this, "Hata Detayı", detailed_error_message);
        // ------------------------------------
    }
}