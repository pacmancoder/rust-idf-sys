use crate::ffi::*;

type BaseType_t = xtensa_int;
type TickType_t = xtensa_unsigned_long;

extern "C" {
    pub fn xPortInIsrContext() -> BaseType_t;
    pub fn xPortGetTickRateHz() -> TickType_t;

    pub fn vTaskDelay(xTicksToDelay: TickType_t);

    pub fn PendSV(request: xtensa_int);
}
