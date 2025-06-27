#ifndef DASHBOARDPAGE_H
#define DASHBOARDPAGE_H

#include <QWidget>

// Kullandığımız Qt sınıflarının ön bildirimi
QT_BEGIN_NAMESPACE
class QLabel;
class QPushButton;
class QTimer;
class QJsonDocument;
QT_END_NAMESPACE

// Kullandığımız özel sınıfların ön bildirimi
class ShieldWidget;
class StatusCardWidget;
class ApiClient;

class DashboardPage : public QWidget
{
    Q_OBJECT
public:
    explicit DashboardPage(QWidget *parent = nullptr);

private slots:
    void fetchEngineStatus();
    void onEngineStatusReceived(bool success, const QJsonDocument& data);
    void onQuickScanClicked();

private:
    ShieldWidget* m_shieldWidget;
    StatusCardWidget* m_securityCard;
    StatusCardWidget* m_updatesCard;
    StatusCardWidget* m_scanCard;
    QPushButton* m_quickScanButton;
    ApiClient* m_apiClient;
    QTimer* m_statusTimer;
};

#endif // DASHBOARDPAGE_H