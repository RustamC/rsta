#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use autocxx::prelude::*;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe)
    generate!("sta_adapter::OpenSta")
    generate!("sta_adapter::DelayCalcMode")
}

pub type OpenSta = ffi::sta_adapter::OpenSta;
pub type DelayCalcMode = ffi::sta_adapter::DelayCalcMode;
