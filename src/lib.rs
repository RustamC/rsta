//! # sta
//!
//! `sta` is a wrapper for OpenSTA

mod bridge;

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
    /// * `filename` - A Path that holds the name of the Liberty file
    pub fn read_liberty(&mut self, filename: &Path) {
        let bytes = filename.to_str().unwrap().as_bytes();
        let filename = std::ffi::CString::new(bytes).unwrap();
        unsafe {
            self.sta.as_mut().read_liberty(filename.as_ptr());
        }
    }

    /// Reads verilog file
    ///
    /// # Arguments
    ///
    /// * `filename` - A Path that holds the name of the Verilog file
    pub fn read_verilog(&mut self, filename: &Path) -> bool {
        let bytes = filename.to_str().unwrap().as_bytes();
        let filename = std::ffi::CString::new(bytes).unwrap();
        unsafe {
            self.sta.as_mut().read_verilog(filename.as_ptr())
        }
    }

    pub fn link_design(&mut self, top_cell_name: &str) -> bool {
        self.top_cell_name = Some(top_cell_name.to_string());

        let bytes = top_cell_name.as_bytes();
        let top_cell_name = std::ffi::CString::new(bytes).unwrap();
        unsafe {
            self.sta.as_mut().link_design(top_cell_name.as_ptr())
        }
    }

    pub fn read_sdc(&mut self, filename: &Path) -> i32 {
        let bytes = filename.to_str().unwrap().as_bytes();
        let filename = std::ffi::CString::new(bytes).unwrap();
        unsafe {
            self.sta.as_mut().read_sdc(filename.as_ptr()).into()
        }
    }
}

impl Default for StaEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    //let mut sta = StaEngine::default();

    //let liberty_path = Path::new("");
    //dbg!(liberty_path);
    //sta.read_liberty(liberty_path);
}
