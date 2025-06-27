#include "settings_page.h"
#include "../api/api_client.h"
#include <QVBoxLayout>
#include <QFormLayout>
#include <QLabel>
#include <QLineEdit>
#include <QSpinBox>
#include <QPushButton>
#include <QJsonObject>
#include <QJsonArray>
#include <QStringList>
#include <QMessageBox>
#include <QTimer>
#include <QFrame>

SettingsPage::SettingsPage(QWidget *parent) : QWidget(parent)
{
    setupUI();
    
    m_apiClient = new ApiClient(this);
    connect(m_saveButton, &QPushButton::clicked, this, &SettingsPage::onSaveSettingsClicked);
    connect(m_apiClient, &ApiClient::settingsReceived, this, &SettingsPage::onSettingsReceived);
    connect(m_apiClient, &ApiClient::settingsSaved, this, &SettingsPage::onSettingsSaved);

    loadCurrentSettings();
}

void SettingsPage::setupUI()
{
    QVBoxLayout* mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(40, 30, 40, 30);
    
    QLabel* title = new QLabel("Uygulama Ayarları", this);
    title->setObjectName("pageTitle");
    mainLayout->addWidget(title, 0, Qt::AlignCenter);
    mainLayout->addSpacing(20);

    QFrame* settingsGroup = new QFrame(this);
    settingsGroup->setObjectName("settingsGroup");
    
    QFormLayout* formLayout = new QFormLayout(settingsGroup);
    formLayout->setSpacing(15);
    formLayout->setLabelAlignment(Qt::AlignRight);

    m_quarantineDirEdit = new QLineEdit(this);
    m_watchDirEdit = new QLineEdit(this);
    m_aiEndpointEdit = new QLineEdit(this);
    m_excludedExtensionsEdit = new QLineEdit(this);
    m_scanThreadsSpinBox = new QSpinBox(this);
    m_scanThreadsSpinBox->setRange(1, 16);

    formLayout->addRow("Karantina Klasörü:", m_quarantineDirEdit);
    formLayout->addRow("İzlenecek Klasör:", m_watchDirEdit);
    formLayout->addRow("AI Sunucu Adresi:", m_aiEndpointEdit);
    formLayout->addRow("Hariç Tutulan Uzantılar (virgülle ayırın):", m_excludedExtensionsEdit);
    formLayout->addRow("Tarama Thread Sayısı:", m_scanThreadsSpinBox);
    
    mainLayout->addWidget(settingsGroup);

    m_saveButton = new QPushButton("Ayarları Kaydet", this);
    m_saveButton->setObjectName("saveSettingsButton");
    QHBoxLayout* buttonLayout = new QHBoxLayout();
    buttonLayout->addStretch();
    buttonLayout->addWidget(m_saveButton);
    
    mainLayout->addLayout(buttonLayout);
    mainLayout->addStretch();
}

void SettingsPage::loadCurrentSettings()
{
    m_apiClient->getSettings();
}

void SettingsPage::onSettingsReceived(bool success, const QJsonDocument& jsonData)
{
    if (success && jsonData.isObject()) {
        m_currentSettings = jsonData.object();
        populateUI(m_currentSettings);
    } else {
        QMessageBox::warning(this, "Hata", "Mevcut ayarlar sunucudan yüklenemedi.");
    }
}

void SettingsPage::populateUI(const QJsonObject& settings)
{
    m_quarantineDirEdit->setText(settings["quarantine_dir"].toString());
    m_watchDirEdit->setText(settings["watch_dir"].toString());
    m_aiEndpointEdit->setText(settings["ai_endpoint"].toString());
    m_scanThreadsSpinBox->setValue(settings["scan_threads"].toInt());
    
    QJsonArray extensions = settings["excluded_extensions"].toArray();
    QStringList extList;
    for(const auto& val : extensions) {
        extList.append(val.toString());
    }
    m_excludedExtensionsEdit->setText(extList.join(", "));
}

void SettingsPage::onSaveSettingsClicked()
{
    m_saveButton->setEnabled(false);
    m_saveButton->setText("Kaydediliyor...");
    
    QJsonDocument settingsDoc(gatherDataFromUI());
    m_apiClient->saveSettings(settingsDoc);
}

QJsonObject SettingsPage::gatherDataFromUI()
{
    QJsonObject newSettings = m_currentSettings;

    newSettings["quarantine_dir"] = m_quarantineDirEdit->text();
    newSettings["watch_dir"] = m_watchDirEdit->text();
    newSettings["ai_endpoint"] = m_aiEndpointEdit->text();
    newSettings["scan_threads"] = m_scanThreadsSpinBox->value();

    QStringList extList = m_excludedExtensionsEdit->text().split(',', Qt::SkipEmptyParts);
    QJsonArray extensionsArray;
    for(const QString& ext : extList) {
        extensionsArray.append(ext.trimmed());
    }
    newSettings["excluded_extensions"] = extensionsArray;

    return newSettings;
}

void SettingsPage::onSettingsSaved(bool success, const QString& message)
{
    m_saveButton->setEnabled(true);
    m_saveButton->setText("Ayarları Kaydet");
    if (success) {
        QMessageBox::information(this, "Başarılı", "Ayarlar kaydedildi.\nDeğişikliklerin çoğunun etkin olması için arka plan servisinin yeniden başlatılması gerekebilir.");
    } else {
        QMessageBox::critical(this, "Hata", "Ayarlar kaydedilemedi.\n\n" + message);
    }
}