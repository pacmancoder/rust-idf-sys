use crate::{
    error::*,
    ffi::*,
};

pub type gpio_num_t = u32;

pub type gpio_int_type_t = u32;
pub const gpio_int_type_t_GPIO_INTR_DISABLE: gpio_int_type_t = 0;
pub const gpio_int_type_t_GPIO_INTR_POSEDGE: gpio_int_type_t = 1;
pub const gpio_int_type_t_GPIO_INTR_NEGEDGE: gpio_int_type_t = 2;
pub const gpio_int_type_t_GPIO_INTR_ANYEDGE: gpio_int_type_t = 3;
pub const gpio_int_type_t_GPIO_INTR_LOW_LEVEL: gpio_int_type_t = 4;
pub const gpio_int_type_t_GPIO_INTR_HIGH_LEVEL: gpio_int_type_t = 5;

pub type gpio_mode_t = u32;
pub const gpio_mode_t_GPIO_MODE_DISABLE: gpio_mode_t = 0;
pub const gpio_mode_t_GPIO_MODE_INPUT: gpio_mode_t = 1;
pub const gpio_mode_t_GPIO_MODE_OUTPUT: gpio_mode_t = 2;
pub const gpio_mode_t_GPIO_MODE_OUTPUT_OD: gpio_mode_t = 6;

pub type gpio_pull_mode_t = u32;
pub const gpio_pull_mode_t_GPIO_PULLUP_ONLY: gpio_pull_mode_t = 0;
pub const gpio_pull_mode_t_GPIO_PULLDOWN_ONLY: gpio_pull_mode_t = 1;
pub const gpio_pull_mode_t_GPIO_FLOATING: gpio_pull_mode_t = 2;

pub type gpio_pullup_t = u32;
pub const gpio_pullup_t_GPIO_PULLUP_DISABLE: gpio_pullup_t = 0;
pub const gpio_pullup_t_GPIO_PULLUP_ENABLE: gpio_pullup_t = 1;

pub type gpio_pulldown_t = u32;
pub const gpio_pulldown_t_GPIO_PULLDOWN_DISABLE: gpio_pulldown_t = 0;
pub const gpio_pulldown_t_GPIO_PULLDOWN_ENABLE: gpio_pulldown_t = 1;

pub type gpio_isr_t =
    ::core::option::Option<unsafe extern "C" fn(arg1: *mut xtensa_void)>;
pub type gpio_isr_handle_t = *mut xtensa_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpio_config_t {
    pub pin_bit_mask: u32,
    pub mode: gpio_mode_t,
    pub pull_up_en: gpio_pullup_t,
    pub pull_down_en: gpio_pulldown_t,
    pub intr_type: gpio_int_type_t,
}

extern "C" {
    pub fn gpio_config(gpio_cfg: *const gpio_config_t) -> esp_err_t;
    pub fn gpio_set_intr_type(gpio_num: gpio_num_t, intr_type: gpio_int_type_t) -> esp_err_t;
    pub fn gpio_set_level(gpio_num: gpio_num_t, level: u32) -> esp_err_t;
    pub fn gpio_get_level(gpio_num: gpio_num_t) -> xtensa_int;
    pub fn gpio_set_direction(gpio_num: gpio_num_t, mode: gpio_mode_t) -> esp_err_t;
    pub fn gpio_set_pull_mode(gpio_num: gpio_num_t, pull: gpio_pull_mode_t) -> esp_err_t;
    pub fn gpio_wakeup_enable(gpio_num: gpio_num_t, intr_type: gpio_int_type_t) -> esp_err_t;
    pub fn gpio_wakeup_disable(gpio_num: gpio_num_t) -> esp_err_t;
    pub fn gpio_isr_register(
        fn_: ::core::option::Option<unsafe extern "C" fn(arg1: *mut xtensa_void)>,
        arg: *mut xtensa_void,
        no_use: xtensa_int,
        handle_no_use: *mut gpio_isr_handle_t,
    ) -> esp_err_t;
    pub fn gpio_pullup_en(gpio_num: gpio_num_t) -> esp_err_t;
    pub fn gpio_pullup_dis(gpio_num: gpio_num_t) -> esp_err_t;
    pub fn gpio_pulldown_en(gpio_num: gpio_num_t) -> esp_err_t;
    pub fn gpio_pulldown_dis(gpio_num: gpio_num_t) -> esp_err_t;
    pub fn gpio_install_isr_service(no_use: xtensa_int) -> esp_err_t;
    pub fn gpio_uninstall_isr_service();
    pub fn gpio_isr_handler_add(
        gpio_num: gpio_num_t,
        isr_handler: gpio_isr_t,
        args: *mut xtensa_void,
    ) -> esp_err_t;

    pub fn gpio_isr_handler_remove(gpio_num: gpio_num_t) -> esp_err_t;
}
