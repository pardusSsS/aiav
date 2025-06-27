#include "shield_widget.h"
#include <QPainter>
#include <QPainterPath>
#include <QGraphicsDropShadowEffect>
#include <QSvgRenderer> // SVG ikonunu çizmek için

ShieldWidget::ShieldWidget(QWidget *parent)
    : QWidget(parent), m_currentState(Secure) // Başlangıç durumunu 'Secure' yap
{
    setMinimumSize(200, 200);

    m_glowEffect = new QGraphicsDropShadowEffect(this);
    m_glowEffect->setBlurRadius(60);
    m_glowEffect->setColor(QColor("#9f50ff")); // Başlangıç rengi
    m_glowEffect->setOffset(0, 0);
    setGraphicsEffect(m_glowEffect);
}

// DÜZELTME: Eksik olan fonksiyonun gövdesi eklendi.
void ShieldWidget::setState(ProtectionState state)
{
    if (m_currentState == state) return; // Durum zaten aynıysa bir şey yapma

    m_currentState = state;
    
    // Duruma göre rengi güncelle
    switch (m_currentState) {
        case Secure:
            m_color = QColor("#66fcf1"); // Güvenli: Neon Turkuaz
            break;
        case Scanning:
            m_color = QColor("#f0e68c"); // Taranıyor: Açık Sarı
            break;
        case AtRisk:
            m_color = QColor("#ff4675"); // Riskli: Canlı Kırmızı/Pembe
            break;
    }
    m_glowEffect->setColor(m_color);

    update(); // Durum değiştiğinde widget'ın yeniden çizilmesini sağla
}

void ShieldWidget::paintEvent(QPaintEvent *event)
{
    Q_UNUSED(event);
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);

    // Kalkan ikonunu SVG kaynağından çiziyoruz.
    // Bu, bize daha fazla esneklik ve kalite sağlar.
    QSvgRenderer renderer(QString(":/icons/shield_status.svg"));
    if (renderer.isValid()) {
        // SVG'yi çizmeden önce rengini değiştirmek doğrudan mümkün değil.
        // Ancak pen ve brush ile üzerine efektler verebiliriz.
        // Şimdilik sadece ikonu çiziyoruz, parlama efekti zaten dışarıdan geliyor.
        renderer.render(&painter, rect());
    }
    
    // Alternatif olarak, QPainterPath ile kalkanı kendimiz de çizebiliriz.
    // Ancak SVG kullanmak daha esnek.
}