#include "mainwindow.h"
#include "widgets/navigationbar.h"
#include "pages/dashboard_page.h"
#include "pages/protection_page.h"
#include "pages/scan_page.h"
#include "pages/quarantine_page.h"
#include "pages/settings_page.h"
#include <QHBoxLayout>
#include <QStackedWidget>
#include <QWidget>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    this->setWindowTitle("AI-Antivirus Kontrol Paneli");

    QWidget *centralWidget = new QWidget(this);
    QHBoxLayout *mainLayout = new QHBoxLayout(centralWidget);
    mainLayout->setSpacing(0);
    mainLayout->setContentsMargins(0, 0, 0, 0);

    m_navigationBar = new NavigationBar(this);
    m_stackedWidget = new QStackedWidget(this);

    m_dashboardPage = new DashboardPage(this);
    m_protectionPage = new ProtectionPage(this);
    m_scanPage = new ScanPage(this);
    m_quarantinePage = new QuarantinePage(this);
    m_settingsPage = new SettingsPage(this);

    m_stackedWidget->addWidget(m_dashboardPage);  // index 0
    m_stackedWidget->addWidget(m_protectionPage); // index 1
    m_stackedWidget->addWidget(m_scanPage);       // index 2
    m_stackedWidget->addWidget(m_quarantinePage); // index 3
    m_stackedWidget->addWidget(m_settingsPage);   // index 4

    mainLayout->addWidget(m_navigationBar);
    mainLayout->addWidget(m_stackedWidget);

    setCentralWidget(centralWidget);
    resize(1280, 800);

    connect(m_navigationBar, &NavigationBar::pageChanged, m_stackedWidget, &QStackedWidget::setCurrentIndex);
}

MainWindow::~MainWindow() {}