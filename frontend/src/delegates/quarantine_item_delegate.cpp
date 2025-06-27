#include "quarantine_item_delegate.h"
#include <QPainter>
#include <QApplication>

QuarantineItemDelegate::QuarantineItemDelegate(QObject *parent)
    : QStyledItemDelegate(parent)
{
}

QSize QuarantineItemDelegate::sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const
{
    // Satırların yüksekliğini biraz artıralım
    QSize size = QStyledItemDelegate::sizeHint(option, index);
    size.setHeight(60); 
    return size;
}

void QuarantineItemDelegate::paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const
{
    painter->setRenderHint(QPainter::Antialiasing);

    // --- Arka Planı Çizme ---
    if (option.state & QStyle::State_Selected) {
        // Eğer satır seçiliyse, vurgu rengiyle bir çerçeve ve dolgu çiz
        painter->fillRect(option.rect, QColor(159, 80, 255, 50)); // Yarı-transparan mor dolgu
        painter->setPen(QPen(QColor("#9f50ff"), 2));
        painter->drawRect(option.rect.adjusted(0, 0, -1, -1));
    } else {
        // Normal satırlar arasında ince bir ayırıcı çizgi
        painter->setPen(QPen(QColor("#2a2b40")));
        painter->drawLine(option.rect.bottomLeft(), option.rect.bottomRight());
    }

    // --- Metinleri Çizme ---
    // Her sütundan veriyi al
    QString quarantinedName = index.sibling(index.row(), 0).data(Qt::DisplayRole).toString();
    QString originalPath = index.sibling(index.row(), 1).data(Qt::DisplayRole).toString();
    QString threatType = index.sibling(index.row(), 2).data(Qt::DisplayRole).toString();

    // Metinler için farklı font ve renkler kullanarak hiyerarşi oluştur
    // Sadece ilk sütun için çizim yap, çünkü delegate her hücre için ayrı ayrı çağrılır.
    // Bu, tüm satırı tek seferde çizmemizi sağlar.
    if (index.column() == 0) {
        // Ana Dosya Adı (Büyük ve Parlak)
        painter->setPen(Qt::white);
        painter->setFont(QFont("Segoe UI", 12, QFont::Bold));
        QRect nameRect = option.rect.adjusted(10, 5, -10, -25);
        painter->drawText(nameRect, Qt::AlignLeft | Qt::AlignVCenter, quarantinedName);

        // Orijinal Konum (Küçük ve Gri)
        painter->setPen(QColor("#a0a1b9"));
        painter->setFont(QFont("Segoe UI", 9));
        QRect pathRect = option.rect.adjusted(10, 25, -10, -5);
        painter->drawText(pathRect, Qt::AlignLeft | Qt::AlignVCenter, originalPath);

        // Tehdit Türü (Sağda ve Vurgu Renginde)
        painter->setPen(QColor("#9f50ff"));
        painter->setFont(QFont("Segoe UI", 10, QFont::Bold));
        QRect threatRect = option.rect.adjusted(-10, 0, -10, 0); // Sağa yaslamak için
        painter->drawText(threatRect, Qt::AlignRight | Qt::AlignVCenter, threatType);
    }
}