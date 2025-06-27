#include "navigationbar.h"
#include <QVBoxLayout>
#include <QPushButton>
#include <QIcon>
#include <QButtonGroup>
#include <QLabel>

NavigationBar::NavigationBar(QWidget *parent) : QWidget(parent)
{
    QVBoxLayout* layout = new QVBoxLayout(this);
    layout->setContentsMargins(10, 20, 10, 20);
    layout->setSpacing(15);
    this->setObjectName("navigationBar");
    this->setFixedWidth(240); // Geniş, metinli menü

    QLabel* title = new QLabel("AI-ANTIVIRUS", this);
    title->setObjectName("pageTitle");
    title->setStyleSheet("font-size: 20px; padding-left: 5px;");
    layout->addWidget(title);
    layout->addSpacing(30);

    QButtonGroup* buttonGroup = new QButtonGroup(this);
    buttonGroup->setExclusive(true);
    
    QStringList labels = {"Koruma", "Web Koruması", "Tarama", "Karantina", "Ayarlar"};
    QStringList icons = {":/icons/overview.svg",  ":/icons/wifi.svg", ":/icons/scan.svg", ":/icons/protection.svg", ":/icons/settings.svg"};

    for (int i = 0; i < labels.count(); ++i) {
        QPushButton* button = new QPushButton(QIcon(icons[i]), " " + labels[i], this);
        button->setObjectName("navButton");
        button->setIconSize(QSize(24, 24));
        button->setCheckable(true);
        buttonGroup->addButton(button);
        layout->addWidget(button);
        connect(button, &QPushButton::clicked, [this, i](){ emit pageChanged(i); });
    }
    
    qobject_cast<QPushButton*>(buttonGroup->buttons().first())->setChecked(true);
    layout->addStretch();
}