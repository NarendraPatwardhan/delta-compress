mod ffi;

use libc::{c_char, c_int};

pub struct Delta;

impl Delta {
    pub fn create(src: &[u8], out: &[u8]) -> Result<Vec<u8>, &'static str> {
        unsafe {
            let mut delta = vec![0u8; out.len() + 60];
            let delta_len = ffi::delta_create(
                src.as_ptr() as *const c_char,
                src.len() as u32,
                out.as_ptr() as *const c_char,
                out.len() as u32,
                delta.as_mut_ptr() as *mut c_char,
            );
            if delta_len <= 0 {
                Err("Failed to create delta")
            } else {
                delta.truncate(delta_len as usize);
                Ok(delta)
            }
        }
    }

    pub fn output_size(delta: &[u8]) -> Result<usize, &'static str> {
        unsafe {
            let size =
                ffi::delta_output_size(delta.as_ptr() as *const c_char, delta.len() as c_int);
            if size < 0 {
                Err("Failed to get output size")
            } else {
                Ok(size as usize)
            }
        }
    }

    pub fn apply(src: &[u8], delta: &[u8]) -> Result<Vec<u8>, &'static str> {
        unsafe {
            let output_size = Self::output_size(delta)?;
            let mut output = vec![0u8; output_size];
            let result = ffi::delta_apply(
                src.as_ptr() as *const c_char,
                src.len() as c_int,
                delta.as_ptr() as *const c_char,
                delta.len() as c_int,
                output.as_mut_ptr() as *mut c_char,
            );
            if result < 0 {
                Err("Failed to apply delta")
            } else {
                output.truncate(result as usize);
                Ok(output)
            }
        }
    }

    pub fn analyze(delta: &[u8]) -> Result<(usize, usize), &'static str> {
        unsafe {
            let mut n_copy = 0;
            let mut n_insert = 0;
            let result = ffi::delta_analyze(
                delta.as_ptr() as *const c_char,
                delta.len() as c_int,
                &mut n_copy,
                &mut n_insert,
            );
            if result != 0 {
                Err("Failed to analyze delta")
            } else {
                Ok((n_copy as usize, n_insert as usize))
            }
        }
    }
}
