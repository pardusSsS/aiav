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
#include <QTimer>
#include <QGroupBox>
#include <QDesktopServices>
#include <QUrl>

ProtectionPage::ProtectionPage(QWidget *parent) : QWidget(parent)
{
    setupUI();
    m_apiClient = new ApiClient(this);
    connectSignals();
    QTimer::singleShot(50, this, &ProtectionPage::loadCurrentSettings);
}

void ProtectionPage::setupUI()
{
    QVBoxLayout* mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(40, 30, 40, 30);
    mainLayout->setSpacing(25);

    QHBoxLayout* topLayout = new QHBoxLayout();
    m_shieldWidget = new ShieldWidget(this);
    m_shieldWidget->setFixedSize(120, 120);

    QVBoxLayout* titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(5);
    QLabel* title = new QLabel("Akıllı Ağ Kalkanı", this);
    title->setObjectName("pageTitle");
    
    m_statusTextLabel = new QLabel("Durum kontrol ediliyor...", this);
    m_statusTextLabel->setObjectName("pageSubtitle");
    
    m_masterEnableToggle = new QCheckBox("Ağ Kalkanını Etkinleştir", this);
    m_masterEnableToggle->setStyleSheet("QCheckBox { font-weight: bold; font-size: 16px; }");

    titleLayout->addWidget(title);
    titleLayout->addWidget(m_statusTextLabel);
    titleLayout->addSpacing(10);
    titleLayout->addWidget(m_masterEnableToggle);
    
    topLayout->addWidget(m_shieldWidget);
    topLayout->addSpacing(20);
    topLayout->addLayout(titleLayout);
    topLayout->addStretch();
    mainLayout->addLayout(topLayout);

    m_settingsGroup = new QGroupBox("Koruma Ayarları", this);
    m_settingsGroup->setObjectName("settingsGroup");
    QVBoxLayout* settingsLayout = new QVBoxLayout(m_settingsGroup);
    settingsLayout->setSpacing(15);
    settingsLayout->setContentsMargins(20, 25, 20, 20);

    settingsLayout->addWidget(new QLabel("<b>Otomatik Engelleme Kategorileri</b>"));
    m_malwareBlockToggle = new QCheckBox("Bilinen Zararlı Yazılım ve Oltalama Sitelerini Engelle", this);
    m_trackerBlockToggle = new QCheckBox("Reklam ve Aktivite Takipçilerini Engelle (Gizliliği Artırır)", this);
    settingsLayout->addWidget(m_malwareBlockToggle);
    settingsLayout->addWidget(m_trackerBlockToggle);
    settingsLayout->addSpacing(20);

    QGridLayout* gridLayout = new QGridLayout();
    gridLayout->setSpacing(20);
    gridLayout->addWidget(new QLabel("<b>İzin Verilenler (Whitelist)</b><br><span style='color:#a0a0c0;'>Engellenmeyecek siteleri her satıra bir tane yazın.</span>"), 0, 0);
    m_whitelistEdit = new QTextEdit(this);
    gridLayout->addWidget(m_whitelistEdit, 1, 0);

    gridLayout->addWidget(new QLabel("<b>Özel Engellenenler (Blacklist)</b><br><span style='color:#a0a0c0;'>Engellemek istediğiniz özel siteleri yazın.</span>"), 0, 1);
    m_blacklistEdit = new QTextEdit(this);
    gridLayout->addWidget(m_blacklistEdit, 1, 1);
    settingsLayout->addLayout(gridLayout);
    settingsLayout->addSpacing(20);

    m_saveButton = new QPushButton("Ayarları Kaydet", this);
    m_saveButton->setObjectName("runQuickScanButton");
    m_saveButton->setFixedWidth(200);
    settingsLayout->addWidget(m_saveButton, 0, Qt::AlignRight);
    
    mainLayout->addWidget(m_settingsGroup);
    mainLayout->addStretch();
}

void ProtectionPage::connectSignals()
{
    connect(m_masterEnableToggle, &QCheckBox::toggled, this, &ProtectionPage::onMasterToggleChanged);
    connect(m_saveButton, &QPushButton::clicked, this, &ProtectionPage::onSaveSettingsClicked);
    
    connect(m_apiClient, &ApiClient::settingsReceived, this, &ProtectionPage::onSettingsReceived);
    connect(m_apiClient, &ApiClient::settingsSaved, this, &ProtectionPage::onSettingsSaved);
    connect(m_apiClient, &ApiClient::webProtectionStatusReceived, this, &ProtectionPage::onWebStatusReceived);
    connect(m_apiClient, &ApiClient::webProtectionToggled, this, &ProtectionPage::onWebProtectionToggled);
}

void ProtectionPage::loadCurrentSettings()
{
    setEnabled(false);
    m_statusTextLabel->setText("Mevcut ayarlar sunucudan yükleniyor...");
    m_apiClient->getSettings();
}

void ProtectionPage::onSettingsReceived(bool success, const QJsonDocument& jsonData)
{
    if (success && jsonData.isObject()) {
        m_currentSettings = jsonData.object();
        populateUI(m_currentSettings);
    } else {
        QMessageBox::warning(this, "Hata", "Mevcut ayarlar sunucudan yüklenemedi.");
    }
    m_apiClient->getWebProtectionStatus();
}

void ProtectionPage::onWebStatusReceived(bool success, const QJsonDocument& jsonData)
{
    setEnabled(true);
    if (!success) {
        QMessageBox::warning(this, "Hata", "Web koruma durumu öğrenilemedi.");
        updateProtectionStatusVisuals(false); // Hata durumunda kapalı göster
        return;
    }
    
    bool isActive = jsonData.object()["is_active"].toBool(false);

    QSignalBlocker blocker(m_masterEnableToggle); // Sinyalin tekrar tetiklenmesini engelle
    m_masterEnableToggle->setChecked(isActive);

    updateProtectionStatusVisuals(isActive);
}

void ProtectionPage::onWebProtectionToggled(bool success, const QString& message)
{
    if (!success) {
        QMessageBox::critical(this, "İşlem Başarısız", "Web kalkanı durumu değiştirilemedi:\n" + message);
    } else {
        if(m_masterEnableToggle->isChecked()){
            QMessageBox msgBox;
            msgBox.setWindowTitle("Kalkan Etkinleştirildi");
            msgBox.setIcon(QMessageBox::Information);
            msgBox.setText("Akıllı Ağ Kalkanı başarıyla etkinleştirildi.");
            msgBox.setInformativeText(
                "<b>Önemli Not:</b> Tarayıcınız, daha önce ziyaret ettiğiniz sitelerle olan "
                "<b>aktif bağlantılarını</b> bir süre açık tutabilir.\n\n"
                "Eğer engellediğiniz bir siteye hala erişebiliyorsanız, lütfen önce "
                "<b>tarayıcınızı tamamen kapatıp yeniden açmayı</b> deneyin.\n\n"
                "Sorun devam ederse, tarayıcınızın ayarlarından o siteye ait "
                "<b>'Çerezleri ve Site Verilerini' temizlemeniz</b> gerekebilir. "
                "Bu işlem, tüm aktif bağlantıları sonlandıracaktır."
            );
            msgBox.setStandardButtons(QMessageBox::Ok);
            msgBox.exec();
        }
    }
    
    m_apiClient->getWebProtectionStatus();
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
}

void ProtectionPage::updateProtectionStatusVisuals(bool isActive)
{
    m_masterEnableToggle->setEnabled(true);
    m_settingsGroup->setEnabled(isActive);
    if (isActive) {
        m_shieldWidget->setState(ShieldWidget::Secure);
        m_statusTextLabel->setText("Ağ Kalkanı AKTİF");
    } else {
        m_shieldWidget->setState(ShieldWidget::AtRisk);
        m_statusTextLabel->setText("Ağ Kalkanı PASİF");
    }
}

void ProtectionPage::onMasterToggleChanged(bool checked)
{
    m_masterEnableToggle->setEnabled(false); // Tekrar tıklanmasını engelle
    m_settingsGroup->setEnabled(false);
    
    if (checked) {
        m_statusTextLabel->setText("Ağ kalkanı etkinleştiriliyor...");
        m_apiClient->enableWebProtection();
    } else {
        m_statusTextLabel->setText("Ağ kalkanı devre dışı bırakılıyor...");
        m_apiClient->disableWebProtection();
    }
}

void ProtectionPage::onSaveSettingsClicked()
{
    m_saveButton->setEnabled(false);
    m_saveButton->setText("Kaydediliyor...");
    m_apiClient->saveSettings(QJsonDocument(gatherDataFromUI()));
}

void ProtectionPage::onSettingsSaved(bool success, const QString& message)
{
    m_saveButton->setEnabled(true);
    m_saveButton->setText("Ayarları Kaydet");
    if (success) {
        QMessageBox::information(this, "Başarılı", "Ayarlar kaydedildi.\nDeğişiklikler aktif korumaya uygulandı.");
        loadCurrentSettings(); 
    } else {
        QMessageBox::critical(this, "Hata", "Ayarlar kaydedilemedi:\n" + message);
    }
}

QJsonObject ProtectionPage::gatherDataFromUI()
{
    QJsonObject newSettings = m_currentSettings;
    QJsonObject webSettings = newSettings["web_security"].toObject();
    webSettings["enable_tracker_protection"] = m_trackerBlockToggle->isChecked();
    webSettings["enable_malware_protection"] = m_malwareBlockToggle->isChecked();
    QStringList whitelist = m_whitelistEdit->toPlainText().split('\n', Qt::SkipEmptyParts, Qt::CaseInsensitive);
    QStringList blacklist = m_blacklistEdit->toPlainText().split('\n', Qt::SkipEmptyParts, Qt::CaseInsensitive);
    webSettings["user_whitelist"] = QJsonArray::fromStringList(whitelist);
    webSettings["user_blacklist"] = QJsonArray::fromStringList(blacklist);
    newSettings["web_security"] = webSettings;
    return newSettings;
}