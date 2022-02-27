#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QEvent>

#include "ngspice.h"

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

protected:
    void keyPressEvent(QKeyEvent *e) override;
    bool event(QEvent *event) override;

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

    void RunSpice(stringstream &ss);
    void UpdatePlot();

private slots:
    void on_btn_loadNetlist_clicked();

private:
    Ui::MainWindow *ui;

    NGSpice* m_ngspice = nullptr;
    SpiceReporter* m_report = nullptr;
};
#endif // MAINWINDOW_H
