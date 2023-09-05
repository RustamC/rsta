//! # sta
//!
//! `sta` is a wrapper for OpenSTA

mod bridge;

pub type DelayCalcMode = bridge::DelayCalcMode;

use std::ffi::CString;
use std::path::Path;
use std::pin::Pin;

use autocxx::WithinBox;

/// This struct is an Adaptor used to hide
/// OpenSTA main class Sta
pub struct StaEngine {
    pub top_cell_name: Option<String>,

    /// sta object
    sta: Pin<Box<bridge::OpenSta>>,
}

impl StaEngine {
    /// Constructor
    pub fn new() -> Self {
        Self {
            top_cell_name: None,
            sta: bridge::OpenSta::new().within_box(),
        }
    }

    /// Reads liberty file
    ///
    /// # Arguments
    ///
    /// * `filename` - A path to the Liberty file to read.
    /// * `cornername` - A process corner used for delay calculation.
    /// * `min_max` - Delay calculation mode: min, max, all.
    /// * `infer_latches` - Flag to infer latches.
    pub fn read_liberty(
        &mut self,
        filename: &Path,
        cornername: Option<String>,
        min_max: DelayCalcMode,
        infer_latches: bool,
    ) -> bool {
        let filename = CString::new(filename.to_str().unwrap()).unwrap();

        unsafe {
            match cornername {
                Some(name) => {
                    let name = CString::new(name).unwrap();
                    self.sta.as_mut().read_liberty(
                        filename.as_ptr(),
                        name.as_ptr(),
                        min_max,
                        infer_latches,
                    )
                }
                None => self.sta.as_mut().read_liberty(
                    filename.as_ptr(),
                    std::ptr::null(),
                    min_max,
                    infer_latches,
                ),
            }
        }
    }

    /// Reads verilog file
    ///
    /// # Arguments
    ///
    /// * `filename` - A path to the Verilog file to read.
    pub fn read_verilog(&mut self, filename: &Path) -> bool {
        let filename = CString::new(filename.to_str().unwrap()).unwrap();
        unsafe { self.sta.as_mut().read_verilog(filename.as_ptr()) }
    }

    /// Link the design
    ///
    /// # Arguments
    ///
    /// * `top_cell_name` - The top level module/cell name of the design hierarchy to link.
    pub fn link_design(&mut self, top_cell_name: &str) -> bool {
        self.top_cell_name = Some(top_cell_name.to_string());

        let top_cell_name = CString::new(top_cell_name).unwrap();
        unsafe { self.sta.as_mut().link_design(top_cell_name.as_ptr()) }
    }

    /// Reads constraints file (.sdc)
    ///
    /// # Arguments
    ///
    /// * `filename` - A path to the SDC file to read.
    pub fn read_sdc(&mut self, filename: &Path) -> i32 {
        let filename = CString::new(filename.to_str().unwrap()).unwrap();
        unsafe { self.sta.as_mut().read_sdc(filename.as_ptr()).into() }
    }
}

impl Default for StaEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_library_reading() {
    let mut sta = StaEngine::default();

    let liberty_path = Path::new("./examples/Nangate45_fast.lib");
    dbg!(liberty_path);
    sta.read_liberty(
        liberty_path,
        Some("fast".to_string()),
        DelayCalcMode::All,
        true,
    );
}
