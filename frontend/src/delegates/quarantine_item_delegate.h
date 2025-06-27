#ifndef QUARANTINEITEMDELEGATE_H
#define QUARANTINEITEMDELEGATE_H

#include <QStyledItemDelegate>

class QuarantineItemDelegate : public QStyledItemDelegate
{
    Q_OBJECT
public:
    explicit QuarantineItemDelegate(QObject *parent = nullptr);

    // Bir hücrenin nasıl çizileceğini belirleyen ana fonksiyon
    void paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const override;

    // Hücrenin boyutunu belirleyen fonksiyon
    QSize sizeHint(const QStyleOptionViewItem &option, const QModelIndex &index) const override;
};

#endif // QUARANTINEITEMDELEGATE_H