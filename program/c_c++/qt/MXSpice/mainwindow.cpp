#include <QKeyEvent>
#include <QDebug>
#include <QFileDialog>

#include <fstream>
#include <sstream>

#include "mainwindow.h"
#include "./ui_mainwindow.h"
#include "./customerevent.h"


void MainWindow::keyPressEvent(QKeyEvent *e)
{
    if ((e->modifiers() == Qt::ControlModifier) && (e->key() == Qt::Key_Q)) {
        this->close();
    }
}

bool MainWindow::event(QEvent *event)
{
    if (event->type() == CustomerEvent::eventType()) {
        CustomerEvent *customerEvent = dynamic_cast<CustomerEvent*>(event);
        qDebug() << "MainWindow event: " << customerEvent->getValueString();
        return true;
    }
    return QWidget::event(event);
}

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    m_report = new SpiceReporter(this);
    m_ngspice = NGSpice::CreateInstance();

    m_ngspice->SetReport(m_report);
}

MainWindow::~MainWindow()
{
    delete ui;
    delete m_report;
    delete m_ngspice;
}

void MainWindow::RunSpice(stringstream &ss)
{
//    string netlist = ".title KiCad schematic\nR1 Net-_C1-Pad2_ GND 100k\nV1 Net-_C1-Pad2_ GND sin(10 5 100k)\nC1 GND Net-_C1-Pad2_ 100n\n.save @r1[i]\n.save @v1[i]\n.save @c1[i]\n.save V(Net-_C1-Pad2_)\n.tran 0 30u 0 1n \n.end\n";
//    stringstream ss(netlist);
    m_ngspice->LoadNetlist(ss);
    m_ngspice->ParseNetlist();
    m_ngspice->Init();
    m_ngspice->Run();
    m_ngspice->GetAllSignals();
}

void MainWindow::UpdatePlot()
{
    /*  */
}

void MainWindow::on_btn_loadNetlist_clicked()
{
//    QString fileName = QFileDialog::getOpenFileName(this, tr("文件对话框！"));
    QString fileName = "/home/maxu/develop/kicad_projects/speci_test/speci_test.cir";

    if (fileName.size()) {
        stringstream ss;
        fstream fs;
        fs.open(fileName.toStdString(), ios::in);
        ss << fs.rdbuf();
        fs.close();
        RunSpice(ss);
    }

}
