use libc::{c_char, c_int, c_uint};

#[link(name = "delta", kind = "static")]
extern "C" {
    pub fn delta_create(
        zSrc: *const c_char,
        lenSrc: c_uint,
        zOut: *const c_char,
        lenOut: c_uint,
        zDelta: *mut c_char,
    ) -> c_int;

    pub fn delta_output_size(zDelta: *const c_char, lenDelta: c_int) -> c_int;

    pub fn delta_apply(
        zSrc: *const c_char,
        lenSrc: c_int,
        zDelta: *const c_char,
        lenDelta: c_int,
        zOut: *mut c_char,
    ) -> c_int;

    pub fn delta_analyze(
        zDelta: *const c_char,
        lenDelta: c_int,
        pnCopy: *mut c_int,
        pnInsert: *mut c_int,
    ) -> c_int;
}
