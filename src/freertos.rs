use crate::ffi::*;

type BaseType_t = xtensa_int;

extern "C" {
    pub fn xPortInIsrContext() -> BaseType_t;
}
