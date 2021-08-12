#ifndef NGSPICE_H
#define NGSPICE_H

#include <sstream>
#include <memory>

//#define ngspice_BOOL_H

#include "spicereporter.h"
#include <ngspice/ngspice.h>
#include <ngspice/sharedspice.h>
#include <ngspice/cpextern.h>

using namespace std;

enum SimType {
    ST_UNKNOWN,
    ST_AC,
    ST_DC,
    ST_DISTORTION,
    ST_NOISE,
    ST_OP,
    ST_POLE_ZERO,
    ST_SENSITIVITY,
    ST_TRANS_FUNC,
    ST_TRANSIENT
};

enum SpicePrimitive {
    SP_UNKNOWN      = ' ',
    SP_RESISTOR     = 'R',
    SP_CAPACITOR    = 'C',
    SP_INDUCTOR     = 'L',
    SP_DIODE        = 'D',
    SP_BJT          = 'Q',
    SP_MOSFET       = 'M',
    SP_JFET         = 'J',
    SP_SUBCKT       = 'X',
    SP_VSOURCE      = 'V',
    SP_ISOURCE      = 'I'
};

class SpiceNode {
public:
    SpiceNode() : name("") {}

private:
    friend class SpiceObject;
    friend class NGSpice;
    string name;
    vector<string> pins;
};

class SpiceObject {
public:
    SpiceObject() : netlist(""), title("") {}
    ~SpiceObject() {
        for (auto node : nodes) {
            delete node;
        }
    };
private:
    friend class NGSpice;
    string netlist;
    string title;
    vector<string> directives;
    vector<SpiceNode*> nodes;
};

class NGSpice : public QObject
{
    Q_OBJECT
public:
    ~NGSpice();

    static NGSpice* CreateInstance();
    void Init();
    bool LoadNetlist(stringstream &ss);
    bool ParseNetlist();
    bool Run();

    vector<string> GetAllSignals();

    string GetXAxis(SimType aType) const;
    const std::vector<string>& GetCurrentTypes(SpicePrimitive aPrimitive);

    void SetReport(SpiceReporter* reporter);

    string& GetNetlist();

private:
    NGSpice();

    bool LoadCodemodels(const string &cmDir);

    /* communicate with form window */
    SpiceReporter* m_reporter;

    /* netlist information*/
    SpiceObject* m_spice_obj;


    typedef int  (*ngSpice_Init)(SendChar* printfcn, SendStat* statfcn, ControlledExit* ngexit,
                                 SendData* sdata, SendInitData* sinitdata, BGThreadRunning* bgtrun, void* userData);

    typedef int  (*ngSpice_Init_Sync)(GetVSRCData *vsrcdat, GetISRCData *isrcdat, GetSyncData *syncdat, int *ident, void *userData);

    typedef int  (*ngSpice_Command)(const char* command);

    typedef pvector_info (*ngGet_Vec_Info)(const char* vecname);

    typedef pevt_shared_data (*ngGet_Evt_NodeInfo)(const char* nodename);

    typedef char** (*ngSpice_AllEvtNodes)();

    typedef int  (*ngSpice_Init_Evt)(SendEvtData* sevtdata, SendInitEvtData* sinitevtdata, void* userData);

    typedef int (*ngSpice_Circ)(char** circarray);

    typedef char* (*ngSpice_CurPlot)(void);

    typedef char** (*ngSpice_AllPlots)(void);

    typedef char** (*ngSpice_AllVecs)(const char* plotname);

    typedef bool (*ngSpice_running)(void);

    typedef bool (*ngSpice_SetBkpt)(double time);

    typedef void (*ngSpice_cp_vset)(char *varname, enum cp_types type, void *value);

    ngSpice_Init m_ngSpice_Init = nullptr;
    ngSpice_Init_Sync m_ngSpice_Init_Sync = nullptr;
    ngSpice_Command m_ngSpice_Command = nullptr;
    ngGet_Vec_Info m_ngGet_Vec_Info = nullptr;
    ngGet_Evt_NodeInfo m_ngGet_Evt_NodeInfo = nullptr;
    ngSpice_AllEvtNodes m_ngSpice_AllEvtNodes = nullptr;
    ngSpice_Init_Evt m_ngSpice_Init_Evt = nullptr;
    ngSpice_Circ m_ngSpice_Circ = nullptr;
    ngSpice_CurPlot m_ngSpice_CurPlot = nullptr;
    ngSpice_AllPlots m_ngSpice_AllPlots = nullptr;
    ngSpice_AllVecs m_ngSpice_AllVecs = nullptr;
    ngSpice_running m_ngSpice_running = nullptr;
    ngSpice_SetBkpt m_ngSpice_SetBkpt = nullptr;
    ngSpice_cp_vset m_ngSpice_cp_vset = nullptr;

    // Callback functions
    static int printFunc( char* what, int id, void* user );
    static int statFunc( char* what, int id, void* user );
    static int ngexitFunc( int status, bool immediate, bool exit_upon_quit, int id, void* user );
    static int bgtrunFunc( bool is_exited, int id, void* user );

};

#endif // NGSPICE_H
