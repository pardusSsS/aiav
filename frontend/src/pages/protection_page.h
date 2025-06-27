#ifndef PROTECTIONPAGE_H
#define PROTECTIONPAGE_H

#include <QWidget>
#include <QJsonDocument>
#include <QJsonObject>   // ➜ m_currentSettings için
#include <QJsonArray>

// Gerekli sınıfların ön bildirimi
QT_BEGIN_NAMESPACE
class QPushButton;
class QLabel;
class QTextEdit;
class QCheckBox;
QT_END_NAMESPACE

class ApiClient;
class ShieldWidget; // Kendi özel kalkan widget'ımızı kullanacağız

class ProtectionPage : public QWidget
{
    Q_OBJECT
public:
    explicit ProtectionPage(QWidget *parent = nullptr);

private slots:
    void onSaveClicked();
    void onSettingsReceived(bool success, const QJsonDocument& jsonData);
    void onSettingsSaved(bool success, const QString& message);
    void loadCurrentSettings();

private:
    void connectSignals(); 
    void setupUI();
    void populateUI(const QJsonObject& settings);
    QJsonObject gatherDataFromUI();
    // YENİ: Kalkanın ve metinlerin görünümünü güncelleyecek fonksiyon
    void updateProtectionStatusVisuals(const QJsonObject& webSettings);

    // Arayüz Bileşenleri
    ShieldWidget* m_shieldWidget; // YENİ: Durum kalkanı
    QLabel* m_statusTextLabel;    // YENİ: "Aktif" / "Pasif" metni
    QCheckBox* m_malwareBlockToggle;
    QCheckBox* m_trackerBlockToggle;
    QTextEdit* m_whitelistEdit;
    QTextEdit* m_blacklistEdit;
    QPushButton* m_saveButton;
    
    ApiClient* m_apiClient;
    QJsonObject m_currentSettings;
};

#endif // PROTECTIONPAGE_H