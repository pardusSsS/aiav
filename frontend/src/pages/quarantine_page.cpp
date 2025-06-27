#include "quarantine_page.h"
#include "../delegates/quarantine_item_delegate.h"
#include "../api/api_client.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QTableWidget>
#include <QHeaderView>
#include <QJsonObject>
#include <QJsonArray>
#include <QMessageBox>
#include <QDebug>

QuarantinePage::QuarantinePage(QWidget *parent) : QWidget(parent)
{
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    
    mainLayout->addWidget(new QLabel("<h3>Karantinadaki Dosyalar</h3>", this));
    m_fetchQuarantineButton = new QPushButton("Karantina Listesini Yenile", this);
    mainLayout->addWidget(m_fetchQuarantineButton);

    m_quarantineTable = new QTableWidget(this);
    m_quarantineTable->setItemDelegate(new QuarantineItemDelegate(this));
    m_quarantineTable->setColumnCount(3);
    m_quarantineTable->verticalHeader()->setVisible(false);
    m_quarantineTable->horizontalHeader()->setVisible(false);
    m_quarantineTable->setSelectionBehavior(QAbstractItemView::SelectRows);
    m_quarantineTable->setSelectionMode(QAbstractItemView::SingleSelection);
    m_quarantineTable->setEditTriggers(QAbstractItemView::NoEditTriggers);
    mainLayout->addWidget(m_quarantineTable);

    QHBoxLayout *actionLayout = new QHBoxLayout();
    m_deleteButton = new QPushButton("Seçileni Sil", this);
    m_restoreButton = new QPushButton("Seçileni Geri Yükle", this);
    m_deleteButton->setEnabled(false);
    m_restoreButton->setEnabled(false);
    actionLayout->addStretch();
    actionLayout->addWidget(m_deleteButton);
    actionLayout->addWidget(m_restoreButton);
    mainLayout->addLayout(actionLayout);

    m_apiClient = new ApiClient(this);

    connect(m_apiClient, &ApiClient::quarantineListReceived, this, &QuarantinePage::onQuarantineListReceived);
    connect(m_fetchQuarantineButton, &QPushButton::clicked, this, &QuarantinePage::onFetchQuarantineClicked);
    connect(m_quarantineTable, &QTableWidget::itemSelectionChanged, this, &QuarantinePage::onQuarantineSelectionChanged);
    connect(m_deleteButton, &QPushButton::clicked, this, &QuarantinePage::onDeleteFileClicked);
    connect(m_restoreButton, &QPushButton::clicked, this, &QuarantinePage::onRestoreFileClicked);
    connect(m_apiClient, &ApiClient::fileActionFinished, this, &QuarantinePage::onFileActionFinished);

    onFetchQuarantineClicked();
}

void QuarantinePage::onFetchQuarantineClicked()
{
    m_quarantineTable->setRowCount(0);
    m_quarantineTable->setRowCount(1);
    m_quarantineTable->setItem(0, 0, new QTableWidgetItem("Karantina listesi alınıyor..."));
    m_apiClient->getQuarantineList();
}

void QuarantinePage::onQuarantineListReceived(bool success, const QJsonDocument& data)
{
    m_quarantineTable->setRowCount(0); // Tabloyu her zaman temizle

    if (!success || !data.isObject()) {
        m_quarantineTable->setRowCount(1);
        m_quarantineTable->setItem(0, 0, new QTableWidgetItem("Karantina listesi alınamadı veya format hatalı!"));
        return;
    }

    QJsonObject rootObj = data.object();
    if (!rootObj.contains("files") || !rootObj["files"].isArray()) {
        m_quarantineTable->setRowCount(1);
        m_quarantineTable->setItem(0, 0, new QTableWidgetItem("Alınan yanıtta 'files' alanı bulunamadı."));
        return;
    }

    QJsonArray filesArray = rootObj["files"].toArray();
    
    if (filesArray.isEmpty()) {
        m_quarantineTable->setRowCount(1);
        m_quarantineTable->setItem(0, 0, new QTableWidgetItem("Karantinada dosya bulunmuyor."));
        return;
    }

    m_quarantineTable->setRowCount(filesArray.count());
    for (int row = 0; row < filesArray.count(); ++row) {
        QJsonValue val = filesArray.at(row);
        QJsonObject fileObj = val.toObject();
        
        // DÜZELTME: Artık tüm sütunlar için verileri JSON'dan çekip ekliyoruz.
        QTableWidgetItem* nameItem = new QTableWidgetItem(fileObj["quarantined_name"].toString());
        QTableWidgetItem* pathItem = new QTableWidgetItem(fileObj["original_path"].toString());
        QTableWidgetItem* signatureItem = new QTableWidgetItem(fileObj["signature"].toString());

        // Alınan verileri doğru sütunlara yerleştir
        m_quarantineTable->setItem(row, 0, nameItem);      // 0. Sütun: Karantina Adı
        m_quarantineTable->setItem(row, 1, pathItem);      // 1. Sütun: Orijinal Konum
        m_quarantineTable->setItem(row, 2, signatureItem); // 2. Sütun: Tehdit Türü
    }
}

void QuarantinePage::onQuarantineSelectionChanged()
{
    bool hasSelection = !m_quarantineTable->selectedItems().isEmpty();
    m_deleteButton->setEnabled(hasSelection);
    m_restoreButton->setEnabled(hasSelection);
}

void QuarantinePage::onDeleteFileClicked()
{
    auto selectedItems = m_quarantineTable->selectedItems();
    if (selectedItems.isEmpty()) return;
    QString filename = selectedItems.first()->text();
    QMessageBox::StandardButton reply = QMessageBox::question(this, "Onay", QString("'%1' dosyasını kalıcı olarak silmek istediğinizden emin misiniz?").arg(filename), QMessageBox::Yes|QMessageBox::No);
    if (reply == QMessageBox::Yes) {
        m_apiClient->deleteQuarantineFile(filename);
    }
}

void QuarantinePage::onRestoreFileClicked()
{
    auto selectedItems = m_quarantineTable->selectedItems();
    if (selectedItems.isEmpty()) return;
    QString filename = selectedItems.first()->text();
    m_apiClient->restoreQuarantineFile(filename);
}

void QuarantinePage::onFileActionFinished(bool success, const QString& message)
{
    if (success) {
        QMessageBox::information(this, "Başarılı", message);
        onFetchQuarantineClicked();
    } else {
        QMessageBox::warning(this, "Hata", "Dosya işlemi başarısız oldu: " + message);
    }
}