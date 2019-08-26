//! Module contains wifi-related structures, enumerations and functions

use crate::{
    ffi::*,
    error::*,
    system_event::*,
    network_adapter,
};

pub type wifi_mode_t = u32;
pub const wifi_mode_t_WIFI_MODE_NULL: wifi_mode_t = 0;
pub const wifi_mode_t_WIFI_MODE_STA: wifi_mode_t = 1;
pub const wifi_mode_t_WIFI_MODE_AP: wifi_mode_t = 2;
pub const wifi_mode_t_WIFI_MODE_APSTA: wifi_mode_t = 3;
pub const wifi_mode_t_WIFI_MODE_MAX: wifi_mode_t = 4;

pub type wifi_auth_mode_t = u32;
pub const wifi_auth_mode_t_WIFI_AUTH_OPEN: wifi_auth_mode_t = 0;
pub const wifi_auth_mode_t_WIFI_AUTH_WEP: wifi_auth_mode_t = 1;
pub const wifi_auth_mode_t_WIFI_AUTH_WPA_PSK: wifi_auth_mode_t = 2;
pub const wifi_auth_mode_t_WIFI_AUTH_WPA2_PSK: wifi_auth_mode_t = 3;
pub const wifi_auth_mode_t_WIFI_AUTH_WPA_WPA2_PSK: wifi_auth_mode_t = 4;
pub const wifi_auth_mode_t_WIFI_AUTH_WPA2_ENTERPRISE: wifi_auth_mode_t = 5;
pub const wifi_auth_mode_t_WIFI_AUTH_MAX: wifi_auth_mode_t = 6;

pub type wifi_scan_method_t = u32;
pub const wifi_scan_method_t_WIFI_FAST_SCAN: wifi_scan_method_t = 0;
pub const wifi_scan_method_t_WIFI_ALL_CHANNEL_SCAN: wifi_scan_method_t = 1;

pub type wifi_sort_method_t = u32;
pub const wifi_sort_method_t_WIFI_CONNECT_AP_BY_SIGNAL: wifi_sort_method_t = 0;
pub const wifi_sort_method_t_WIFI_CONNECT_AP_BY_SECURITY: wifi_sort_method_t = 1;

pub use network_adapter::esp_interface_t as wifi_interface_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_init_config_t {
    pub event_handler: system_event_handler_t,
    pub osi_funcs: *mut xtensa_void,
    pub static_rx_buf_num: xtensa_int,
    pub dynamic_rx_buf_num: xtensa_int,
    pub tx_buf_type: xtensa_int,
    pub static_tx_buf_num: xtensa_int,
    pub dynamic_tx_buf_num: xtensa_int,
    pub csi_enable: xtensa_int,
    pub ampdu_rx_enable: xtensa_int,
    pub ampdu_tx_enable: xtensa_int,
    pub nvs_enable: xtensa_int,
    pub nano_enable: xtensa_int,
    pub tx_ba_win: xtensa_int,
    pub rx_ba_win: xtensa_int,
    pub magic: xtensa_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_fast_scan_threshold_t {
    pub rssi: i8,
    pub authmode: wifi_auth_mode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_sta_config_t {
    pub ssid: [u8; 32usize],
    pub password: [u8; 64usize],
    pub scan_method: wifi_scan_method_t,
    pub bssid_set: bool,
    pub bssid: [u8; 6usize],
    pub channel: u8,
    pub listen_interval: u16,
    pub sort_method: wifi_sort_method_t,
    pub threshold: wifi_fast_scan_threshold_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_ap_config_t {
    pub ssid: [u8; 32usize],
    pub password: [u8; 64usize],
    pub ssid_len: u8,
    pub channel: u8,
    pub authmode: wifi_auth_mode_t,
    pub ssid_hidden: u8,
    pub max_connection: u8,
    pub beacon_interval: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union wifi_config_t {
    pub ap: wifi_ap_config_t,
    pub sta: wifi_sta_config_t,
    _bindgen_union_align: [u32; 31usize],
}

extern "C" {
    pub fn esp_wifi_init(config: *const wifi_init_config_t) -> esp_err_t;
    pub fn esp_wifi_deinit() -> esp_err_t;

    pub fn esp_wifi_set_mode(mode: wifi_mode_t) -> esp_err_t;
    pub fn esp_wifi_set_config(interface: wifi_interface_t, conf: *mut wifi_config_t) -> esp_err_t;

    pub fn esp_wifi_start() -> esp_err_t;
    pub fn esp_wifi_stop() -> esp_err_t;
}

pub unsafe fn WIFI_INIT_CONFIG_DEFAULT() ->  wifi_init_config_t {
    wifi_init_config_t {
        event_handler: Some(esp_event_send),
        osi_funcs: ::core::ptr::null_mut(),
        static_rx_buf_num: 5,
        dynamic_rx_buf_num: 0,
        tx_buf_type: 0,
        static_tx_buf_num: 6,
        dynamic_tx_buf_num: 0,
        csi_enable: 0,
        ampdu_rx_enable: 0,
        ampdu_tx_enable: 0,
        nvs_enable: 1,
        nano_enable: 0,
        tx_ba_win: 0,
        rx_ba_win: 0,
        magic: 0x1F2F3F4F,
    }
}