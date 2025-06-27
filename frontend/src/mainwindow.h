#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

QT_BEGIN_NAMESPACE
class QStackedWidget;
QT_END_NAMESPACE

class NavigationBar;
class DashboardPage;
class ProtectionPage;
class ScanPage;
class QuarantinePage;
class SettingsPage;

class MainWindow : public QMainWindow
{
    Q_OBJECT
public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private:
    NavigationBar* m_navigationBar;
    QStackedWidget* m_stackedWidget;
    
    DashboardPage* m_dashboardPage;
    ProtectionPage* m_protectionPage;
    ScanPage* m_scanPage;
    QuarantinePage* m_quarantinePage;
    SettingsPage* m_settingsPage;
};
#endif // MAINWINDOW_H