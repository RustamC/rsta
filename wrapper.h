#pragma once
#include <iostream>
#include <tcl/tcl.h>
#include <Sta.hh>
#include <VerilogReader.hh>

namespace sta_adapter {
    using namespace sta;

    class OpenSta {
        public:
            OpenSta();
            ~OpenSta() {};
            void read_liberty(const char *filename);
            bool read_verilog(const char *filename);
            bool link_design(const char *top_cell_name);
            int read_sdc(const char *filename);
        private:
            sta::Sta *sta;
    };

    OpenSta::OpenSta() {
        Tcl_Interp *interp = Tcl_CreateInterp();
        
        sta::initSta();
        this->sta = new sta::Sta;
        Sta::setSta(this->sta);
        this->sta->makeComponents();
        this->sta->setTclInterp(interp);
    }

    void OpenSta::read_liberty(const char *filename) {
        this->sta->readLiberty(filename, 
                          this->sta->cmdCorner(),
                          sta::MinMaxAll::all(),
                          true);
    }

    bool OpenSta::read_verilog(const char *filename) {
        NetworkReader *network = this->sta->networkReader();
        if (network) {
            this->sta->readNetlistBefore();
            return readVerilogFile(filename, network);
        }
        else
            return false;
    }

    bool OpenSta::link_design(const char *top_cell_name) {
        return Sta::sta()->linkDesign(top_cell_name);
    }

    int OpenSta::read_sdc(const char *filename) {
        Tcl_Interp *interp = Sta::sta()->tclInterp();

        return Tcl_EvalFile(interp, filename);
    }
}