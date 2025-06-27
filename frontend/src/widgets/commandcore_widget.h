#ifndef COMMANDCOREWIDGET_H
#define COMMANDCOREWIDGET_H

#include <QWidget>
#include <QPropertyAnimation> // Animasyon için
#include <QGraphicsDropShadowEffect> // Gölgelendirme efekti için

class CommandCoreWidget : public QWidget
{
    Q_OBJECT
    // Animasyon için Q_PROPERTY tanımı
    Q_PROPERTY(qreal pulseRadius READ pulseRadius WRITE setPulseRadius)

public:
    // Koruma durumlarını tanımlayan enum
    enum ProtectionState {
        Secure,
        Scanning,
        AtRisk
    };
    Q_ENUM(ProtectionState)

    explicit CommandCoreWidget(QWidget *parent = nullptr);

    void setState(ProtectionState state);
    qreal pulseRadius() const;
    void setPulseRadius(qreal radius);

protected:
    // Widget'ı çizmek için override edilen fonksiyon
    void paintEvent(QPaintEvent *event) override;

private:
    ProtectionState m_currentState;
    qreal m_pulseRadius;
    QPropertyAnimation* m_pulseAnimation;
    QGraphicsDropShadowEffect* m_glowEffect; 
};

#endif // COMMANDCOREWIDGET_H