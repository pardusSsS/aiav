#ifndef STATUSCARDWIDGET_H
#define STATUSCARDWIDGET_H

#include <QFrame>

class QLabel;

class StatusCardWidget : public QFrame
{
    Q_OBJECT
public:
    explicit StatusCardWidget(const QString& title, const QString& initialValue, QWidget *parent = nullptr);
    void setValueText(const QString& text);

private:
    QLabel* m_titleLabel;
    QLabel* m_valueLabel;
};
#endif // STATUSCARDWIDGET_H