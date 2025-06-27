#ifndef API_CLIENT_H
#define API_CLIENT_H

#include <QObject>
#include <QString>
#include <QNetworkAccessManager>
#include <QJsonDocument>
#include <QList>
#include <QUrl>

class ApiClient : public QObject
{
    Q_OBJECT

public:
    explicit ApiClient(QObject *parent = nullptr);

    // Tüm public metotlar burada bildiriliyor
    void getEngineStatus();
    void getQuarantineList();
    void getScanProgress();
    void startScan(const QString& path);
    void startQuickScan();
    void deleteQuarantineFile(const QString& filename);
    void restoreQuarantineFile(const QString& filename);
    void saveSettings(const QJsonDocument& settingsDoc);
    void getSettings();
    void enableWebProtection();
    void disableWebProtection();
    void getWebProtectionStatus();

signals:
    void engineStatusReceived(bool success, const QJsonDocument& data);
    void quarantineListReceived(bool success, const QJsonDocument& data);
    void scanRequestFinished(bool success, const QString& message);
    void fileActionFinished(bool success, const QString& message);
    void settingsReceived(bool success, const QJsonDocument& data);
    void settingsSaved(bool success, const QString& message);
    void scanProgressReceived(bool success, const QJsonDocument& data);
    void webProtectionToggled(bool success, const QString& message);
    void webProtectionStatusReceived(bool success, const QJsonDocument& data);

private slots:
    void onReplyFinished(class QNetworkReply* reply);

private:
    QNetworkAccessManager* m_networkManager;
    const QString m_baseUrl = "http://127.0.0.1:3000/api";
};

#endif // API_CLIENT_H