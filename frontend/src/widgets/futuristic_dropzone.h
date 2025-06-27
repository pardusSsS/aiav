#ifndef FUTURISTICDROPZONE_H
#define FUTURISTICDROPZONE_H

#include <QWidget>
#include <QPropertyAnimation>

class FuturisticDropZone : public QWidget
{
    Q_OBJECT
    // Animasyon için 'glowFactor' adında bir özellik tanımlıyoruz
    Q_PROPERTY(qreal glowFactor READ glowFactor WRITE setGlowFactor)

public:
    explicit FuturisticDropZone(QWidget *parent = nullptr);

signals:
    void filesDropped(const QList<QUrl>& fileUrls);

protected:
    void paintEvent(QPaintEvent *event) override;
    void dragEnterEvent(QDragEnterEvent *event) override;
    void dragLeaveEvent(QDragLeaveEvent *event) override;
    void dropEvent(QDropEvent *event) override;

private:
    qreal m_glowFactor; // 0.0 ile 1.0 arası bir değer
    bool m_isDragOver;

    void setGlowFactor(qreal factor);
    qreal glowFactor() const;

    QPropertyAnimation* m_glowAnimation;
};

#endif // FUTURISTICDROPZONE_H