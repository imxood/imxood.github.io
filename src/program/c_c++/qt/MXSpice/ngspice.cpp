
#include <QCoreApplication>
#include <QLibrary>
#include <QtDebug>
#include <QDir>
#include <QMessageBox>

#include <boost/algorithm/string.hpp>
#include <string>

#include "customerevent.h"
#include "ngspice.h"

NGSpice* NGSpice::CreateInstance()
{
    static NGSpice* ngspiceInstance;
    if (!ngspiceInstance) {
        ngspiceInstance = new NGSpice();
    }
    return ngspiceInstance;
}

void NGSpice::Init()
{
    m_ngSpice_Command("reset");
}

bool NGSpice::LoadNetlist(stringstream &ss)
{
    vector<char*> lines;

    while (!ss.eof()) {
        char line[1024];
        ss.getline(line, sizeof(line));
        lines.push_back( strdup(line) );
        m_spice_obj->netlist += string( line ) + string( "\n" );
    }

    lines.push_back( nullptr );
    m_ngSpice_Circ( lines.data() );

    for (auto line : lines) {
        free(line);
    }
    qDebug() << "load netlist successed!";
    return true;
}

bool NGSpice::ParseNetlist()
{
    int idx;
    vector<string> lines;
    vector<string> items;
    boost::split(lines, m_spice_obj->netlist, boost::is_any_of("\n"));

    if (lines.size() == 0) {
        QMessageBox::information(NULL, "spice", "Netlist is NULL");
        return false;
    }

    auto&& first_line = lines[0];
    idx = first_line.find(".title");
    if (idx != -1) {
        auto&& title = first_line.substr(idx);
        boost::trim(title);
        m_spice_obj->title = title;
    }
    else {
        m_spice_obj->title = first_line;
    }

    for (int i=1; i<(int)lines.size(); i++) {

        auto&& line = lines[i];

        if (line[0] == '.') {
            if (line.starts_with(".tran")) {
                m_spice_obj->directives.push_back(line);
            }
        }
        else {
            boost::split(items, line, boost::is_any_of(" "));

            SpiceNode *node = new SpiceNode();
            node->name = items[0];
            for (int j=1; j<(int)items.size()-1; j++) {
                node->pins.push_back(move(items[j]));
            }
            m_spice_obj->nodes.push_back(node);
//            items.clear();
        }
    }
    return true;
}

bool NGSpice::Run()
{
    m_ngSpice_Command("bg_run");
    return true;
}

vector<string> NGSpice::GetAllSignals()
{
    vector<string> sigs;
    for(auto &node : m_spice_obj->nodes) {
        string& node_name = node->name;
        for(auto &type : GetCurrentTypes((SpicePrimitive)node_name[0])) {
            sigs.push_back(type + "(" + node_name + ")");
        }
    }
    return sigs;
}

const std::vector<string>& NGSpice::GetCurrentTypes( SpicePrimitive aPrimitive )
{
    static const vector<string> passive = { "I" };
    static const vector<string> diode = { "Id" };
    static const vector<string> bjt = { "Ib", "Ic", "Ie" };
    static const vector<string> mos = { "Ig", "Id", "Is" };
    static const vector<string> empty;

    switch( aPrimitive )
    {
        case SP_RESISTOR:
        case SP_CAPACITOR:
        case SP_INDUCTOR:
        case SP_VSOURCE:
            return passive;

        case SP_DIODE:
            return diode;

        case SP_BJT:
            return bjt;

        case SP_MOSFET:
            return mos;

        default:
            return empty;
    }
}

string NGSpice::GetXAxis(SimType aType) const
{
    switch( aType )
    {
        case ST_AC:
        case ST_NOISE:
            return string( "frequency" );
            break;

        case ST_DC:
            return string( "v-sweep" );
            break;

        case ST_TRANSIENT:
            return string( "time" );
            break;

        default:
            break;
    }

    return string( "" );
}

void NGSpice::SetReport(SpiceReporter* reporter)
{
    Q_ASSERT(reporter != nullptr);
    m_reporter = reporter;
}

string &NGSpice::GetNetlist()
{
    return m_spice_obj->netlist;
}

NGSpice::NGSpice() : m_reporter(nullptr), m_spice_obj(new SpiceObject)
{
    QLibrary ngspiceLib("/usr/local/lib/libngspice.so.0.0.0");
    if (!ngspiceLib.load()) {
        qDebug() << NGSPICE_LIBRARIES;
        qDebug() << "loading ngpice library failed, error: " << ngspiceLib.errorString();
        return;
    }
    m_ngSpice_Init = (ngSpice_Init)ngspiceLib.resolve("ngSpice_Init");
    m_ngSpice_Init_Sync = (ngSpice_Init_Sync)ngspiceLib.resolve("ngSpice_Init_Sync");
    m_ngSpice_Command = (ngSpice_Command)ngspiceLib.resolve("ngSpice_Command");
    m_ngGet_Vec_Info = (ngGet_Vec_Info)ngspiceLib.resolve("ngGet_Vec_Info");
    m_ngGet_Evt_NodeInfo = (ngGet_Evt_NodeInfo)ngspiceLib.resolve("ngGet_Evt_NodeInfo");
    m_ngSpice_AllEvtNodes = (ngSpice_AllEvtNodes)ngspiceLib.resolve("ngSpice_AllEvtNodes");
    m_ngSpice_Init_Evt = (ngSpice_Init_Evt)ngspiceLib.resolve("ngSpice_Init_Evt");
    m_ngSpice_Circ = (ngSpice_Circ)ngspiceLib.resolve("ngSpice_Circ");
    m_ngSpice_CurPlot = (ngSpice_CurPlot)ngspiceLib.resolve("ngSpice_CurPlot");
    m_ngSpice_AllPlots = (ngSpice_AllPlots)ngspiceLib.resolve("ngSpice_AllPlots");
    m_ngSpice_AllVecs = (ngSpice_AllVecs)ngspiceLib.resolve("ngSpice_AllVecs");
    m_ngSpice_running = (ngSpice_running)ngspiceLib.resolve("ngSpice_running");
    m_ngSpice_SetBkpt = (ngSpice_SetBkpt)ngspiceLib.resolve("ngSpice_SetBkpt");

    m_ngSpice_cp_vset = (ngSpice_cp_vset)ngspiceLib.resolve("cp_vset");

    m_ngSpice_Init( &printFunc, &statFunc, &ngexitFunc, NULL, NULL, &bgtrunFunc, this );
    qDebug() << "init ok";

    bool t = true;
    m_ngSpice_cp_vset("cpdebug", CP_BOOL, &t);

    string ngspice_cm_path = "/usr/lib/ngspice";

    //    string command = "set __CMPATH=";
    //    command = command + "\"" + ngspice_cm_path + "\"";
    //    m_ngSpice_Command( command.c_str() );
    LoadCodemodels(ngspice_cm_path);

    m_ngSpice_Command( "unset interactive" );
    m_ngSpice_Command( "set noaskquit" );
    m_ngSpice_Command( "set nomoremode" );
}

NGSpice::~NGSpice()
{
    delete m_spice_obj;
}

bool NGSpice::LoadCodemodels(const string &cmDir)
{
    QDir dir(QString::fromStdString(cmDir));
    QStringList fileList = dir.entryList(QStringList() << "*.cm", QDir::Files);
    foreach (QString file, fileList) {
        string command = "codemodel " + dir.absoluteFilePath(file).toStdString();
        qDebug() << file;
        m_ngSpice_Command( command.c_str() );
    }
    return fileList.size() != 0;
}

int NGSpice::printFunc(char *what, int id, void *user)
{
    int ret = 0;
    if( strncasecmp(what, "stdout ", 7) == 0 ) {
        what += 7;
        ret = 0;
    } else if( strncasecmp(what, "stderr ", 7) == 0 ) {
        what += 7;
        ret = -1;
    }
    qDebug() << what << endl;
    return ret;
}

int NGSpice::statFunc(char *what, int id, void *user)
{
//    qDebug() << "statFunc what: " << what << " id: " << id;
    return 0;
}

int NGSpice::ngexitFunc(int status, bool immediate, bool exit_upon_quit, int id, void *user)
{
    qDebug() << "ngexitFunc status: " << status;
    return 0;
}

int NGSpice::bgtrunFunc(bool is_exited, int id, void *user)
{
    NGSpice *ngSpice = (NGSpice *)user;

    if (is_exited) {
        qDebug() << "*********************************************";
        char** nodes = ngSpice->m_ngSpice_AllEvtNodes();
        while (nodes) {
            qDebug() << nodes++;
        }
        qDebug() << "*********************************************";
    }

    ngSpice->m_reporter->OnStateChange(is_exited);
    return 0;
}
