#ifndef SETTINGSPAGE_H
#define SETTINGSPAGE_H

#include <QWidget>
#include <QJsonDocument>
#include <QJsonObject>      
#include <QJsonArray> 

// Gerekli Qt sınıflarının ön bildirimi
QT_BEGIN_NAMESPACE
class QLineEdit;
class QSpinBox;
class QPushButton;
class QJsonObject; // Düzeltme: QJsonObject burada bildiriliyor
QT_END_NAMESPACE

class ApiClient;

class SettingsPage : public QWidget
{
    Q_OBJECT
public:
    explicit SettingsPage(QWidget *parent = nullptr);

private slots:
    void onSettingsReceived(bool success, const QJsonDocument& jsonData);
    void onSaveSettingsClicked();
    void onSettingsSaved(bool success, const QString& message);
    void loadCurrentSettings();

private:
    // Arayüzü oluşturan ve verileri işleyen özel fonksiyonlar
    void setupUI();
    void populateUI(const QJsonObject& settings);
    QJsonObject gatherDataFromUI();

    ApiClient* m_apiClient;

    // Sunucudan gelen ayarların tam bir kopyasını tutacak üye değişkeni
    QJsonObject m_currentSettings; 

    // Ayar alanları için tüm üye değişken bildirimleri
    QLineEdit* m_quarantineDirEdit;
    QLineEdit* m_watchDirEdit;
    QLineEdit* m_aiEndpointEdit;
    QLineEdit* m_excludedExtensionsEdit;
    QSpinBox* m_scanThreadsSpinBox;
    QPushButton* m_saveButton; // Kaydet butonu da bir üye olmalı
};

#endif // SETTINGSPAGE_H