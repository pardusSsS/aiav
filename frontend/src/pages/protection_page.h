#ifndef PROTECTIONPAGE_H
#define PROTECTIONPAGE_H

#include <QWidget>
#include <QJsonDocument>
#include <QJsonObject>

// Gerekli sınıfların ön bildirimi (Forward declaration)
QT_BEGIN_NAMESPACE
class QPushButton;
class QLabel;
class QTextEdit;
class QCheckBox;
class QGroupBox;
QT_END_NAMESPACE

class ApiClient;
class ShieldWidget;

class ProtectionPage : public QWidget
{
    Q_OBJECT
public:
    explicit ProtectionPage(QWidget *parent = nullptr);

private slots:
    void onSettingsReceived(bool success, const QJsonDocument& jsonData);
    void onSettingsSaved(bool success, const QString& message);
    void loadCurrentSettings();
    
    // DÜZELTME: Tüm yeni slotlar burada doğru şekilde beyan ediliyor.
    void onMasterToggleChanged(bool checked);
    void onSaveSettingsClicked();
    void onWebStatusReceived(bool success, const QJsonDocument& jsonData);
    void onWebProtectionToggled(bool success, const QString& message);

private:
    void setupUI();
    void connectSignals(); 
    void populateUI(const QJsonObject& settings);
    QJsonObject gatherDataFromUI();
    void updateProtectionStatusVisuals(bool isActive);

    // Arayüz Bileşenleri
    ShieldWidget* m_shieldWidget;
    QLabel* m_statusTextLabel;
    QCheckBox* m_masterEnableToggle;
    QGroupBox* m_settingsGroup;
    QCheckBox* m_malwareBlockToggle;
    QCheckBox* m_trackerBlockToggle;
    QTextEdit* m_whitelistEdit;
    QTextEdit* m_blacklistEdit;
    QPushButton* m_saveButton;
    
    ApiClient* m_apiClient;
    QJsonObject m_currentSettings;
};

#endif // PROTECTIONPAGE_H