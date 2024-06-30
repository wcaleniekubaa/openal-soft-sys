#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

pub mod ffi {
    #[path = "../al.rs"]
    pub mod al;

    #[path = "../alc.rs"]
    pub mod alc;
}
