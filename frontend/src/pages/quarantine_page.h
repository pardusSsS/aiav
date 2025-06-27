#ifndef QUARANTINEPAGE_H
#define QUARANTINEPAGE_H

#include <QWidget>
// DÜZELTME: Doğru include yolu eklendi.
// 'pages' klasöründen bir üste ('src') çıkıp 'api' klasörüne giriyoruz.
#include "../api/api_client.h"

QT_BEGIN_NAMESPACE
class QTableWidget;
class QPushButton;
QT_END_NAMESPACE

class QuarantinePage : public QWidget
{
    Q_OBJECT

public:
    explicit QuarantinePage(QWidget *parent = nullptr);

private slots:
    void onQuarantineListReceived(bool success, const QJsonDocument& data);
    void onFetchQuarantineClicked();
    void onQuarantineSelectionChanged();
    void onDeleteFileClicked();
    void onRestoreFileClicked();
    void onFileActionFinished(bool success, const QString& message);

private:
    QTableWidget* m_quarantineTable;
    QPushButton* m_fetchQuarantineButton;
    QPushButton* m_deleteButton;
    QPushButton* m_restoreButton;
    ApiClient* m_apiClient;
};

#endif // QUARANTINEPAGE_H