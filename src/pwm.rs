use crate::error::*;

extern "C" {
    pub fn pwm_init(
        period: u32,
        duties: *mut u32,
        channel_num: u8,
        pin_num: *const u32,
    ) -> esp_err_t;
    pub fn pwm_deinit() -> esp_err_t;
    pub fn pwm_set_duty(channel_num: u8, duty: u32) -> esp_err_t;
    pub fn pwm_set_period(period: u32) -> esp_err_t;
    pub fn pwm_start() -> esp_err_t;
    pub fn pwm_stop(stop_level_mask: u32) -> esp_err_t;
    pub fn pwm_set_phase(channel_num: u8, phase: i16) -> esp_err_t;
    pub fn pwm_set_channel_invert(channel_mask: u16) -> esp_err_t;
    pub fn pwm_clear_channel_invert(channel_mask: u16) -> esp_err_t;
}