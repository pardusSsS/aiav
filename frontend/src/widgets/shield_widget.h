#ifndef SHIELDWIDGET_H
#define SHIELDWIDGET_H

#include <QWidget>
#include <QColor>

class QGraphicsDropShadowEffect;

class ShieldWidget : public QWidget
{
    Q_OBJECT

public:
    // DÜZELTME: Koruma durumlarını tanımlayan public enum
    enum ProtectionState {
        Secure,
        Scanning,
        AtRisk
    };
    Q_ENUM(ProtectionState) // Qt'nin meta sisteminin bu enum'ı tanımasını sağlar

    explicit ShieldWidget(QWidget *parent = nullptr);

public slots:
    // DÜZELTME: Bu slot, DashboardPage'den çağrılarak kalkanın durumunu değiştirir.
    void setState(ProtectionState state);

protected:
    void paintEvent(QPaintEvent *event) override;

private:
    ProtectionState m_currentState; // Mevcut durumu tutan değişken
    QColor m_color;
    QGraphicsDropShadowEffect* m_glowEffect;
};

#endif // SHIELDWIDGET_H