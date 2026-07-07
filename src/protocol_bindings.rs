mod protocol_bindings {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]   // 추가
    #![allow(clippy::nursery)]
    #![allow(clippy::cargo)]

    include!(concat!(env!("OUT_DIR"), "/protocol.rs"));
}

pub use protocol_bindings::*;
