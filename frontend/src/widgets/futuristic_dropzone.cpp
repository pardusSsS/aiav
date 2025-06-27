#include "futuristic_dropzone.h"
#include <QPainter>
#include <QPainterPath>
#include <QDragEnterEvent>
#include <QMimeData>
#include <QGraphicsDropShadowEffect>

FuturisticDropZone::FuturisticDropZone(QWidget *parent)
    : QWidget(parent), m_glowFactor(0.5), m_isDragOver(false)
{
    setAcceptDrops(true);
    setMinimumSize(300, 260);

    // Parlama animasyonu
    m_glowAnimation = new QPropertyAnimation(this, "glowFactor");
    m_glowAnimation->setDuration(1500);
    m_glowAnimation->setStartValue(0.5);
    m_glowAnimation->setEndValue(1.0);
    m_glowAnimation->setEasingCurve(QEasingCurve::InOutSine);
    m_glowAnimation->setLoopCount(-1); // Sonsuz döngü
    m_glowAnimation->start();

    // Dış ışıma için gölge efekti
    QGraphicsDropShadowEffect* glow = new QGraphicsDropShadowEffect(this);
    glow->setBlurRadius(30);
    glow->setColor(QColor(159, 80, 255, 150)); // Morumsu bir parlama
    glow->setOffset(0,0);
    setGraphicsEffect(glow);
}

// QPropertyAnimation tarafından kullanılacak fonksiyonlar
void FuturisticDropZone::setGlowFactor(qreal factor) {
    m_glowFactor = factor;
    update(); // Her animasyon adımında yeniden çiz
}
qreal FuturisticDropZone::glowFactor() const {
    return m_glowFactor;
}

// Sürükleme olayları
void FuturisticDropZone::dragEnterEvent(QDragEnterEvent *event) {
    if (event->mimeData()->hasUrls()) {
        event->acceptProposedAction();
        m_isDragOver = true;
        m_glowAnimation->pause(); // Animasyonu durdur ve tam parlaklığa geç
        setGlowFactor(1.0);
    }
}

void FuturisticDropZone::dragLeaveEvent(QDragLeaveEvent *event) {
    Q_UNUSED(event);
    m_isDragOver = false;
    m_glowAnimation->resume(); // Animasyona devam et
}

void FuturisticDropZone::dropEvent(QDropEvent *event) {
    m_isDragOver = false;
    m_glowAnimation->resume();
    if (event->mimeData()->hasUrls()) {
        emit filesDropped(event->mimeData()->urls());
    }
}

// Çizim fonksiyonu
void FuturisticDropZone::paintEvent(QPaintEvent *event) {
    Q_UNUSED(event);
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);

    // Altıgenin köşe noktalarını hesapla
    QPainterPath hexagonPath;
    float width = this->width() - 20;
    float height = this->height() - 20;
    float x = 10, y = 10;
    hexagonPath.moveTo(x + width / 2, y);
    hexagonPath.lineTo(x + width, y + height / 4);
    hexagonPath.lineTo(x + width, y + height * 3 / 4);
    hexagonPath.lineTo(x + width / 2, y + height);
    hexagonPath.lineTo(x, y + height * 3 / 4);
    hexagonPath.lineTo(x, y + height / 4);
    hexagonPath.closeSubpath();

    // Duruma göre renkleri ayarla
    QColor borderColor = m_isDragOver ? QColor(220, 180, 255) : QColor(159, 80, 255);
    int alpha = static_cast<int>(m_glowFactor * 150 + 50); // 50 ile 200 arası alpha değeri
    borderColor.setAlpha(alpha);

    // Altıgeni çiz
    painter.setPen(QPen(borderColor, 3));
    painter.setBrush(QColor(31, 40, 51, 150)); // Yarı transparan iç dolgu
    painter.drawPath(hexagonPath);

    // İçine metin yaz
    painter.setPen(Qt::white);
    painter.setFont(QFont("Segoe UI", 14));
    painter.drawText(this->rect(), Qt::AlignCenter, "DOSYALARI ANALİZ İÇİN\nBU PORTALA SÜRÜKLEYİN");
}