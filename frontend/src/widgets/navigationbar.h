#ifndef NAVIGATIONBAR_H
#define NAVIGATIONBAR_H

#include <QWidget>
#include <QList>

class QPushButton;

class NavigationBar : public QWidget
{
    Q_OBJECT
public:
    explicit NavigationBar(QWidget *parent = nullptr);
signals:
    void pageChanged(int index);
};
#endif // NAVIGATIONBAR_H