#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_int, c_uint};

const MAX_CHUNK: usize = 2953;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Encode `data` to QRCode.
/// Returns Vec<bool> where true indicates white, and false indicates black
/// Will panic if data is empty or bigger than 2953
/// Will segfault if you're being stupid 
pub fn encode_data(data: &[u8], version: i32, level: ECLevel) -> QRCodeResult {
    if data.is_empty() || data.len() > MAX_CHUNK {
        panic!("Empty or too big buffer");
    }

    unsafe {
        let qr = QRcode_encodeData(
            data.len() as c_int,
            data.as_ptr(),
            version as c_int,
            level as c_uint
        );
        // Convert into human useable form lol
        let width = (*qr).width as usize; 
        let version = (*qr).version as i32;
        let mut data = vec![false; width*width];
        for i in 0..width*width {
            data[i] = *(*qr).data.add(i) & 0b00000001 == 0
        }
        // Free
        QRcode_free(qr);
        QRCodeResult {
            data,
            width,
            version
        }
    }
}

/// Safe error correction level
#[derive(Debug, Clone, Copy)]
pub enum ECLevel {
    /// lowest
    L = 0,
    M = 1,
    Q = 2,
    /// highest
    H = 3
}

/// Safe QRCode
#[derive(Debug, Clone)]
pub struct QRCodeResult {
    pub data: Vec<bool>,
    pub width: usize,
    pub version: i32
}