#include "api_client.h"
#include <QNetworkReply>
#include <QJsonObject>
#include <QUrl>
#include <QDebug>

ApiClient::ApiClient(QObject *parent) : QObject(parent) {
    m_networkManager = new QNetworkAccessManager(this);
    connect(m_networkManager, &QNetworkAccessManager::finished, this, &ApiClient::onReplyFinished);
}

void ApiClient::getEngineStatus() { m_networkManager->get(QNetworkRequest(QUrl(m_baseUrl + "/engine-status"))); }
void ApiClient::getQuarantineList() { m_networkManager->get(QNetworkRequest(QUrl(m_baseUrl + "/quarantine"))); }
void ApiClient::getScanProgress() { m_networkManager->get(QNetworkRequest(QUrl(m_baseUrl + "/scan/progress"))); }
void ApiClient::startQuickScan() { m_networkManager->post(QNetworkRequest(QUrl(m_baseUrl + "/scan/quick")), QByteArray()); }
void ApiClient::deleteQuarantineFile(const QString &filename) { m_networkManager->deleteResource(QNetworkRequest(QUrl(m_baseUrl + "/quarantine/" + filename))); }
void ApiClient::restoreQuarantineFile(const QString &filename) { m_networkManager->post(QNetworkRequest(QUrl(m_baseUrl + "/quarantine/" + filename + "/restore")), QByteArray()); }
void ApiClient::getSettings() { m_networkManager->get(QNetworkRequest(QUrl(m_baseUrl + "/settings"))); }
void ApiClient::enableWebProtection() { m_networkManager->post(QNetworkRequest(QUrl(m_baseUrl + "/protection/web/enable")), QByteArray()); }
void ApiClient::disableWebProtection() { m_networkManager->post(QNetworkRequest(QUrl(m_baseUrl + "/protection/web/disable")), QByteArray()); }
void ApiClient::getWebProtectionStatus() { m_networkManager->get(QNetworkRequest(QUrl(m_baseUrl + "/protection/web"))); }

void ApiClient::startScan(const QString& path) {
    QNetworkRequest request(QUrl(m_baseUrl + "/scan"));
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    QJsonObject json;
    json["path"] = path;
    m_networkManager->post(request, QJsonDocument(json).toJson());
}

void ApiClient::saveSettings(const QJsonDocument& settingsDoc) {
    QNetworkRequest request(QUrl(m_baseUrl + "/settings"));
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");
    m_networkManager->post(request, settingsDoc.toJson());
}

void ApiClient::onReplyFinished(QNetworkReply* reply) {
    bool success = (reply->error() == QNetworkReply::NoError);
    QByteArray responseData = reply->readAll();
    QJsonDocument jsonDoc = QJsonDocument::fromJson(responseData);
    
    const QUrl url = reply->url();
    const QNetworkAccessManager::Operation operation = reply->operation();

    if (!success) {
        qWarning() << "[ApiClient] API Error:" << reply->errorString() << "on" << url.toString();
    }

    if (url.path().endsWith("/engine-status")) emit engineStatusReceived(success, jsonDoc);
    else if (url.path().endsWith("/quarantine") && operation == QNetworkAccessManager::GetOperation) emit quarantineListReceived(success, jsonDoc);
    else if (url.path().endsWith("/scan")) emit scanRequestFinished(success, QString::fromUtf8(responseData));
    else if (operation == QNetworkAccessManager::DeleteOperation) emit fileActionFinished(success, "Dosya başarıyla silindi.");
    else if (url.path().contains("/restore")) emit fileActionFinished(success, "Dosya başarıyla geri yüklendi.");
    else if (url.path().endsWith("/settings") && operation == QNetworkAccessManager::GetOperation) emit settingsReceived(success, jsonDoc);
    else if (url.path().endsWith("/settings") && operation == QNetworkAccessManager::PostOperation) emit settingsSaved(success, "Ayarlar başarıyla kaydedildi.");
    else if (url.path().endsWith("/scan/progress")) emit scanProgressReceived(success, jsonDoc);
    else if (url.path().endsWith("/protection/web") && operation == QNetworkAccessManager::GetOperation) emit webProtectionStatusReceived(success, jsonDoc);
    else if (url.path().contains("/protection/web/")) emit webProtectionToggled(success, "Web koruma isteği işlendi.");
    
    reply->deleteLater();
}