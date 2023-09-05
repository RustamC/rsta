#pragma once
#include <iostream>
#include <tcl/tcl.h>
#include <Sta.hh>
#include <VerilogReader.hh>

namespace sta_adapter {
    using namespace sta;

    enum DelayCalcMode {
        Min,
        Max,
        All
    };

    class OpenSta {
        public:
            OpenSta();
            ~OpenSta() {};
            bool read_liberty(const char *filename, const char *corner_name, DelayCalcMode min_max, bool infer_latches);
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
        this->sta->makeComponents();
        this->sta->setTclInterp(interp);
    }

    bool OpenSta::read_liberty(const char *filename, const char *corner_name, DelayCalcMode min_max, bool infer_latches) {
        sta::MinMaxAll *mm = nullptr;
        switch (min_max)
        {
        case DelayCalcMode::Min:
            mm = sta::MinMaxAll::min();
            break;
        case DelayCalcMode::Max:
            mm = sta::MinMaxAll::max();
            break;
        case DelayCalcMode::All:
            mm = sta::MinMaxAll::all();
            break;
        };

        if (corner_name == nullptr) {
            this->sta->readLiberty(filename, 
                            this->sta->cmdCorner(),
                            mm,
                            infer_latches);
        } else {
            Corner *corner = this->sta->findCorner(corner_name);

            if (corner == nullptr) {
                return false;
            } else {
                this->sta->readLiberty(filename, 
                            corner,
                            mm,
                            infer_latches);
            }
        }

        return true;
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