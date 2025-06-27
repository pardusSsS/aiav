#ifndef SCANPAGE_H
#define SCANPAGE_H

#include <QWidget>
#include <QList>
#include <QUrl>

QT_BEGIN_NAMESPACE
class QTextEdit;
class QPushButton;
class QLabel;
class QProgressBar;
class QFrame;
class QTimer;
class QWebSocket;
class QJsonDocument;
QT_END_NAMESPACE

class FuturisticDropZone;
class ApiClient;

class ScanPage : public QWidget
{
    Q_OBJECT

public:
    explicit ScanPage(QWidget *parent = nullptr);
    ~ScanPage();

private slots:
    void onFilesDropped(const QList<QUrl>& fileUrls);
    void selectFolderToScan();
    void performQuickScan();
    void onScanResult(bool success, const QString& message);
    void onLogSocketConnected();
    void onLogMessageReceived(const QString &message);
    void updateScanProgress();
    void onScanProgressReceived(bool success, const QJsonDocument& data);

private:
    void setupUI();
    void connectSignals();
    void startScanProcess(const QString& scanType);
    void stopScanProcess();

    FuturisticDropZone* m_dropZone;
    QPushButton* m_selectFolderButton;
    QPushButton* m_quickScanButton;
    QProgressBar* m_progressBar;
    QFrame* m_progressContainer;
    QLabel* m_statusLabel;
    QTextEdit* m_logOutput;
    
    ApiClient* m_apiClient;
    QTimer* m_progressTimer;
    QWebSocket* m_logSocket;
};

#endif // SCANPAGE_H