//! Module contains wifi-related structures, enumerations and functions

use crate::{
    ffi::*,
    error::*,
    system_event::*,
    network_adapter,
};

pub type wifi_err_reason_t = u32;
pub const wifi_err_reason_t_WIFI_REASON_UNSPECIFIED: wifi_err_reason_t = 1;
pub const wifi_err_reason_t_WIFI_REASON_AUTH_EXPIRE: wifi_err_reason_t = 2;
pub const wifi_err_reason_t_WIFI_REASON_AUTH_LEAVE: wifi_err_reason_t = 3;
pub const wifi_err_reason_t_WIFI_REASON_ASSOC_EXPIRE: wifi_err_reason_t = 4;
pub const wifi_err_reason_t_WIFI_REASON_ASSOC_TOOMANY: wifi_err_reason_t = 5;
pub const wifi_err_reason_t_WIFI_REASON_NOT_AUTHED: wifi_err_reason_t = 6;
pub const wifi_err_reason_t_WIFI_REASON_NOT_ASSOCED: wifi_err_reason_t = 7;
pub const wifi_err_reason_t_WIFI_REASON_ASSOC_LEAVE: wifi_err_reason_t = 8;
pub const wifi_err_reason_t_WIFI_REASON_ASSOC_NOT_AUTHED: wifi_err_reason_t = 9;
pub const wifi_err_reason_t_WIFI_REASON_DISASSOC_PWRCAP_BAD: wifi_err_reason_t = 10;
pub const wifi_err_reason_t_WIFI_REASON_DISASSOC_SUPCHAN_BAD: wifi_err_reason_t = 11;
pub const wifi_err_reason_t_WIFI_REASON_IE_INVALID: wifi_err_reason_t = 13;
pub const wifi_err_reason_t_WIFI_REASON_MIC_FAILURE: wifi_err_reason_t = 14;
pub const wifi_err_reason_t_WIFI_REASON_4WAY_HANDSHAKE_TIMEOUT: wifi_err_reason_t = 15;
pub const wifi_err_reason_t_WIFI_REASON_GROUP_KEY_UPDATE_TIMEOUT: wifi_err_reason_t = 16;
pub const wifi_err_reason_t_WIFI_REASON_IE_IN_4WAY_DIFFERS: wifi_err_reason_t = 17;
pub const wifi_err_reason_t_WIFI_REASON_GROUP_CIPHER_INVALID: wifi_err_reason_t = 18;
pub const wifi_err_reason_t_WIFI_REASON_PAIRWISE_CIPHER_INVALID: wifi_err_reason_t = 19;
pub const wifi_err_reason_t_WIFI_REASON_AKMP_INVALID: wifi_err_reason_t = 20;
pub const wifi_err_reason_t_WIFI_REASON_UNSUPP_RSN_IE_VERSION: wifi_err_reason_t = 21;
pub const wifi_err_reason_t_WIFI_REASON_INVALID_RSN_IE_CAP: wifi_err_reason_t = 22;
pub const wifi_err_reason_t_WIFI_REASON_802_1X_AUTH_FAILED: wifi_err_reason_t = 23;
pub const wifi_err_reason_t_WIFI_REASON_CIPHER_SUITE_REJECTED: wifi_err_reason_t = 24;
pub const wifi_err_reason_t_WIFI_REASON_BEACON_TIMEOUT: wifi_err_reason_t = 200;
pub const wifi_err_reason_t_WIFI_REASON_NO_AP_FOUND: wifi_err_reason_t = 201;
pub const wifi_err_reason_t_WIFI_REASON_AUTH_FAIL: wifi_err_reason_t = 202;
pub const wifi_err_reason_t_WIFI_REASON_ASSOC_FAIL: wifi_err_reason_t = 203;
pub const wifi_err_reason_t_WIFI_REASON_HANDSHAKE_TIMEOUT: wifi_err_reason_t = 204;
pub const wifi_err_reason_t_WIFI_REASON_BASIC_RATE_NOT_SUPPORT: wifi_err_reason_t = 205;

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

pub const WIFI_PROTOCOL_11B: u32 = 1;
pub const WIFI_PROTOCOL_11G: u32 = 2;
pub const WIFI_PROTOCOL_11N: u32 = 4;

pub use network_adapter::esp_interface_t as wifi_interface_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wifi_init_config_t {
    pub event_handler: system_event_handler_t,
    pub osi_funcs: *mut xtensa_void,
    pub qos_enable: u8,
    pub ampdu_rx_enable: u8,
    pub rx_ba_win: u8,
    pub rx_ampdu_buf_num: u8,
    pub rx_ampdu_buf_len: u32,
    pub rx_max_single_pkt_len: u32,
    pub rx_buf_len: u32,
    pub amsdu_rx_enable: u8,
    pub rx_buf_num: u8,
    pub rx_pkt_num: u8,
    pub left_continuous_rx_buf_num: u8,
    pub tx_buf_num: u8,
    pub nvs_enable: u8,
    pub nano_enable: u8,
    pub magic: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wifi_scan_threshold_t {
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
    pub threshold: wifi_scan_threshold_t,
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

    pub fn esp_wifi_set_protocol(ifx: wifi_interface_t, protocol_bitmap: u8) -> esp_err_t;

    pub fn esp_wifi_connect() -> esp_err_t;
}

pub unsafe fn WIFI_INIT_CONFIG_DEFAULT() ->  wifi_init_config_t {
    wifi_init_config_t {
        event_handler: Some(esp_event_send),
        osi_funcs: ::core::ptr::null_mut(),
        qos_enable: 0,
        ampdu_rx_enable: 0,
        rx_ampdu_buf_len: 256,
        rx_ampdu_buf_num: 5,
        amsdu_rx_enable: 0,
        rx_ba_win: 0,
        rx_max_single_pkt_len: 1600 - 524,
        rx_buf_len: 524,
        rx_buf_num: 16,
        left_continuous_rx_buf_num: 4,
        rx_pkt_num: 7,
        tx_buf_num: 6,
        nvs_enable: 1,
        nano_enable: 0,
        magic: 0x1F2F3F4F,
    }
}