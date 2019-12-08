use crate::ffi::*;

type BaseType_t = xtensa_int;
type TickType_t = xtensa_unsigned_long;

pub type QueueHandle_t = *mut xtensa_void;

pub type TaskHandle_t = *mut xtensa_void;
pub type TaskFunction_t = Option<unsafe extern "C" fn(arg1: *mut xtensa_void)>;


extern "C" {
    pub fn xPortInIsrContext() -> BaseType_t;
    pub fn xPortGetTickRateHz() -> TickType_t;

    pub fn vTaskDelay(xTicksToDelay: TickType_t);

    pub fn PendSV(request: xtensa_int);

    pub fn xTaskCreate(
        pxTaskCode: TaskFunction_t,
        pcName: *const u8,
        usStackDepth: u16,
        pvParameters: *mut xtensa_void,
        uxPriority: xtensa_unsigned_long,
        pxCreatedTask: *mut TaskHandle_t,
    ) -> xtensa_long;
}
