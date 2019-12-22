use crate::{
    ffi::xtensa_int,
    error::esp_err_t,
};
use crate::ffi::{xtensa_unsigned_long, xtensa_void};

pub type uart_port_t = u32;
pub const uart_port_t_UART_NUM_0: uart_port_t = 0;
pub const uart_port_t_UART_NUM_1: uart_port_t = 1;

pub type uart_word_length_t = u32;
pub const uart_word_length_t_UART_DATA_5_BITS: uart_word_length_t = 0;
pub const uart_word_length_t_UART_DATA_6_BITS: uart_word_length_t = 1;
pub const uart_word_length_t_UART_DATA_7_BITS: uart_word_length_t = 2;
pub const uart_word_length_t_UART_DATA_8_BITS: uart_word_length_t = 3;

pub type uart_parity_t = u32;
pub const uart_parity_t_UART_PARITY_DISABLE: uart_parity_t = 0;
pub const uart_parity_t_UART_PARITY_EVEN: uart_parity_t = 2;
pub const uart_parity_t_UART_PARITY_ODD: uart_parity_t = 3;

pub type uart_stop_bits_t = u32;
pub const uart_stop_bits_t_UART_STOP_BITS_1: uart_stop_bits_t = 1;
pub const uart_stop_bits_t_UART_STOP_BITS_1_5: uart_stop_bits_t = 2;
pub const uart_stop_bits_t_UART_STOP_BITS_2: uart_stop_bits_t = 3;

pub type uart_hw_flowcontrol_t = u32;
pub const uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_DISABLE: uart_hw_flowcontrol_t = 0;
pub const uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_RTS: uart_hw_flowcontrol_t = 1;
pub const uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_CTS: uart_hw_flowcontrol_t = 2;
pub const uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_CTS_RTS: uart_hw_flowcontrol_t = 3;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uart_config_t {
    pub baud_rate: xtensa_int,
    pub data_bits: uart_word_length_t,
    pub parity: uart_parity_t,
    pub stop_bits: uart_stop_bits_t,
    pub flow_ctrl: uart_hw_flowcontrol_t,
    pub rx_flow_ctrl_thresh: u8,
}

extern "C" {
    pub fn uart_param_config(uart_num: uart_port_t, uart_conf: *mut uart_config_t) -> esp_err_t;

    pub fn uart_set_baudrate(uart_num: uart_port_t, baudrate: xtensa_int) -> esp_err_t;
    pub fn uart_set_word_length(uart_num: uart_port_t, data_bit: uart_word_length_t) -> esp_err_t;
    pub fn uart_set_parity(uart_num: uart_port_t, parity_mode: uart_parity_t) -> esp_err_t;
    pub fn uart_set_stop_bits(uart_num: uart_port_t, stop_bits: uart_stop_bits_t) -> esp_err_t;
    pub fn uart_set_hw_flow_ctrl(
        uart_num: uart_port_t,
        flow_ctrl: uart_hw_flowcontrol_t,
        rx_thresh: u8,
    ) -> esp_err_t;

    pub fn uart_enable_swap() -> esp_err_t;
    pub fn uart_disable_swap() -> esp_err_t;

    pub fn uart_write_bytes(
        uart_num: uart_port_t,
        src: *const u8,
        size: usize,
    ) -> xtensa_int;

    pub fn uart_read_bytes(
        uart_num: uart_port_t,
        buf: *mut u8,
        length: u32,
        ticks_to_wait: xtensa_unsigned_long,
    ) -> xtensa_int;

    pub fn uart_wait_tx_done(
        uart_num: uart_port_t,
        ticks_to_wait: xtensa_unsigned_long,
    ) -> esp_err_t;

    pub fn uart_flush_input(uart_num: uart_port_t) -> esp_err_t;

    pub fn uart_driver_install(
        uart_num: uart_port_t,
        rx_buffer_size: xtensa_int,
        tx_buffer_size: xtensa_int,
        queue_size: xtensa_int,
        uart_queue: *mut xtensa_void,
    ) -> esp_err_t;

    pub fn uart_driver_delete(uart_num: uart_port_t) -> esp_err_t;
}