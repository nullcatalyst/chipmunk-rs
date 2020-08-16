#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// suppress warnings about u128 (that we don't even use ourselves anyway)
#![allow(improper_ctypes)]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/chipmunk_bindings.rs"));
