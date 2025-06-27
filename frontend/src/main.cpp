#include <QApplication>
#include <QFile>
#include <QDebug>
#include "mainwindow.h"

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);
    
    QFile styleFile(":/styles/dark_theme.qss");
    if (styleFile.open(QFile::ReadOnly | QFile::Text)) {
        app.setStyleSheet(QLatin1String(styleFile.readAll()));
    } else {
        qWarning() << "Stil dosyası yüklenemedi!";
    }

    MainWindow w;
    w.show();
    
    return app.exec();
}