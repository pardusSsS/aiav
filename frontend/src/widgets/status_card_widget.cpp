#include "status_card_widget.h"
#include <QVBoxLayout>
#include <QLabel>

StatusCardWidget::StatusCardWidget(const QString& title, const QString& initialValue, QWidget *parent)
    : QFrame(parent)
{
    this->setObjectName("statusCard");
    this->setMinimumHeight(80);

    QVBoxLayout* layout = new QVBoxLayout(this);
    m_titleLabel = new QLabel(title.toUpper(), this);
    m_titleLabel->setObjectName("statusCard_Title");
    
    m_valueLabel = new QLabel(initialValue, this);
    m_valueLabel->setObjectName("statusCard_Value");

    layout->addWidget(m_titleLabel);
    layout->addWidget(m_valueLabel);
}

void StatusCardWidget::setValueText(const QString &text)
{
    m_valueLabel->setText(text);
}