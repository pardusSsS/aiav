#include "commandcore_widget.h"
#include <QPainter>
#include <QPainterPath>
#include <QRadialGradient>
#include <QTimer>
#include <QGraphicsDropShadowEffect>

CommandCoreWidget::CommandCoreWidget(QWidget *parent)
    : QWidget(parent), m_currentState(Secure), m_pulseRadius(0.0)
{
    // Widget'ın bir minimum boyutu olmasını sağlıyoruz.
    setMinimumSize(300, 300);

    // "Nefes alma" animasyonunu oluşturuyoruz.
    // Bu animasyon, 'pulseRadius' özelliğini 2 saniye boyunca 0'dan 20'ye değiştirir.
    m_pulseAnimation = new QPropertyAnimation(this, "pulseRadius");
    m_pulseAnimation->setDuration(2000);
    m_pulseAnimation->setStartValue(0.0);
    m_pulseAnimation->setEndValue(20.0);
    m_pulseAnimation->setEasingCurve(QEasingCurve::InOutSine);
    m_pulseAnimation->setLoopCount(-1); // Sonsuz döngü
    m_pulseAnimation->start();

    // Parlama (Glow) efekti için bir gölge efekti oluşturuyoruz.
    m_glowEffect = new QGraphicsDropShadowEffect(this);
    m_glowEffect->setBlurRadius(60);   // Parlamanın ne kadar yayılacağı
    m_glowEffect->setOffset(0, 0);     // Gölgeyi tam arkasında tut
    setGraphicsEffect(m_glowEffect); // Efekti bu widget'a uygula
}

// Bu fonksiyon, Ana Panel'den gelen sinyalle Komuta Çekirdeği'nin durumunu değiştirir.
void CommandCoreWidget::setState(ProtectionState state)
{
    if (m_currentState == state) return; // Durum zaten aynıysa bir şey yapma

    m_currentState = state;
    update(); // Durum değiştiğinde widget'ın yeniden çizilmesini tetikle
}

// Aşağıdaki iki fonksiyon, QPropertyAnimation'ın 'pulseRadius' özelliğini
// kontrol edebilmesi için gereklidir.
qreal CommandCoreWidget::pulseRadius() const {
    return m_pulseRadius;
}

void CommandCoreWidget::setPulseRadius(qreal radius)
{
    m_pulseRadius = radius;
    update(); // Animasyonun her adımında widget'ı yeniden çiz
}

// Bu, widget'ın tüm çizim mantığının bulunduğu yerdir.
void CommandCoreWidget::paintEvent(QPaintEvent *event)
{
    Q_UNUSED(event); // Kullanılmayan parametre uyarısını önle
    
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing); // Çizimlerin kenarlarını yumuşat

    // Widget'ın boyutuna göre çizimi ortala ve ölçekle
    int side = qMin(width(), height());
    painter.translate(width() / 2.0, height() / 2.0);
    painter.scale(side / 300.0, side / 300.0); // 300x300 bir alana çizim yaptığımızı varsayıyoruz

    QColor coreColor;
    QString statusText;

    // Mevcut duruma göre renk ve metin seçimi
    switch (m_currentState) {
        case Secure:
            coreColor = QColor(102, 252, 241); // #66fcf1 (Canlı Cyan)
            statusText = "GÜVENDE";
            break;
        case Scanning:
            coreColor = QColor(255, 230, 100); // Sarı
            statusText = "TARANIYOR";
            break;
        case AtRisk:
            coreColor = QColor(255, 70, 120);  // Canlı Pembe/Kırmızı
            statusText = "RİSK ALTINDA";
            break;
    }

    // `m_glowEffect`'in rengini mevcut duruma göre dinamik olarak ayarla
    m_glowEffect->setColor(coreColor);

    // --- Kalkan Çizimi ---
    QPainterPath shieldPath;
    shieldPath.moveTo(0, -110);
    shieldPath.cubicTo(0, -80, -90, -70, -90, -40);
    shieldPath.quadTo(-90, 60, 0, 100);
    shieldPath.quadTo(90, 60, 90, -40);
    shieldPath.cubicTo(90, -70, 0, -80, 0, -110);
    
    // Kalkanın içini ve dışını çiz
    painter.setBrush(QColor(31, 40, 51, 220)); // Yarı-transparan iç dolgu
    painter.setPen(QPen(coreColor, 6));
    painter.drawPath(shieldPath);

    // --- Durum İkonu Çizimi (Onay veya Ünlem) ---
    QPainterPath iconPath;
    if (m_currentState == Secure) {
        // Onay işareti
        iconPath.moveTo(-45, 0);
        iconPath.lineTo(-15, 30);
        iconPath.lineTo(50, -35);
    } else {
        // Ünlem işareti
        iconPath.moveTo(0, -30);
        iconPath.lineTo(0, 10);
        iconPath.addEllipse(QPointF(0, 30), 5, 5);
    }
        
    QPen iconPen(coreColor.lighter(130), 15);
    iconPen.setCapStyle(Qt::RoundCap);
    iconPen.setJoinStyle(Qt::RoundJoin);
    painter.setPen(iconPen);
    painter.drawPath(iconPath);
    
    // --- Durum Metni Çizimi ---
    painter.setPen(Qt::white);
    painter.setFont(QFont("Segoe UI", 18, QFont::Bold));
    // Metni kalkanın altına yerleştir
    painter.drawText(QRectF(-120, 110, 240, 40), statusText, QTextOption(Qt::AlignCenter));
}