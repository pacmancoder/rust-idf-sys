//! Module contains structures, enumerations and functions related to IDF system events

use crate::{
    ffi::*,
    error::*,
    network_adapter::*,
    wifi::*,
};

pub type system_event_id_t = u32;
pub const system_event_id_t_SYSTEM_EVENT_WIFI_READY: system_event_id_t = 0;
pub const system_event_id_t_SYSTEM_EVENT_SCAN_DONE: system_event_id_t = 1;
pub const system_event_id_t_SYSTEM_EVENT_STA_START: system_event_id_t = 2;
pub const system_event_id_t_SYSTEM_EVENT_STA_STOP: system_event_id_t = 3;
pub const system_event_id_t_SYSTEM_EVENT_STA_CONNECTED: system_event_id_t = 4;
pub const system_event_id_t_SYSTEM_EVENT_STA_DISCONNECTED: system_event_id_t = 5;
pub const system_event_id_t_SYSTEM_EVENT_STA_AUTHMODE_CHANGE: system_event_id_t = 6;
pub const system_event_id_t_SYSTEM_EVENT_STA_GOT_IP: system_event_id_t = 7;
pub const system_event_id_t_SYSTEM_EVENT_STA_LOST_IP: system_event_id_t = 8;
pub const system_event_id_t_SYSTEM_EVENT_STA_WPS_ER_SUCCESS: system_event_id_t = 9;
pub const system_event_id_t_SYSTEM_EVENT_STA_WPS_ER_FAILED: system_event_id_t = 10;
pub const system_event_id_t_SYSTEM_EVENT_STA_WPS_ER_TIMEOUT: system_event_id_t = 11;
pub const system_event_id_t_SYSTEM_EVENT_STA_WPS_ER_PIN: system_event_id_t = 12;
pub const system_event_id_t_SYSTEM_EVENT_AP_START: system_event_id_t = 13;
pub const system_event_id_t_SYSTEM_EVENT_AP_STOP: system_event_id_t = 14;
pub const system_event_id_t_SYSTEM_EVENT_AP_STACONNECTED: system_event_id_t = 15;
pub const system_event_id_t_SYSTEM_EVENT_AP_STADISCONNECTED: system_event_id_t = 16;
pub const system_event_id_t_SYSTEM_EVENT_AP_STAIPASSIGNED: system_event_id_t = 17;
pub const system_event_id_t_SYSTEM_EVENT_AP_PROBEREQRECVED: system_event_id_t = 18;
pub const system_event_id_t_SYSTEM_EVENT_GOT_IP6: system_event_id_t = 19;
pub const system_event_id_t_SYSTEM_EVENT_ETH_START: system_event_id_t = 20;
pub const system_event_id_t_SYSTEM_EVENT_ETH_STOP: system_event_id_t = 21;
pub const system_event_id_t_SYSTEM_EVENT_ETH_CONNECTED: system_event_id_t = 22;
pub const system_event_id_t_SYSTEM_EVENT_ETH_DISCONNECTED: system_event_id_t = 23;
pub const system_event_id_t_SYSTEM_EVENT_ETH_GOT_IP: system_event_id_t = 24;
pub const system_event_id_t_SYSTEM_EVENT_MAX: system_event_id_t = 25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_sta_connected_t {
    pub ssid: [u8; 32usize],
    pub ssid_len: u8,
    pub bssid: [u8; 6usize],
    pub channel: u8,
    pub authmode: wifi_auth_mode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_sta_disconnected_t {
    pub ssid: [u8; 32usize],
    pub ssid_len: u8,
    pub bssid: [u8; 6usize],
    pub reason: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_sta_scan_done_t {
    pub status: u32,
    pub number: u8,
    pub scan_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_sta_authmode_change_t {
    pub old_mode: wifi_auth_mode_t,
    pub new_mode: wifi_auth_mode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_sta_got_ip_t {
    pub ip_info: tcpip_adapter_ip_info_t,
    pub ip_changed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_sta_wps_er_pin_t {
    pub pin_code: [u8; 8usize],
}

pub type system_event_sta_wps_fail_reason_t = u32;
pub const system_event_sta_wps_fail_reason_t_WPS_FAIL_REASON_NORMAL:
system_event_sta_wps_fail_reason_t = 0;
pub const system_event_sta_wps_fail_reason_t_WPS_FAIL_REASON_RECV_M2D:
system_event_sta_wps_fail_reason_t = 1;
pub const system_event_sta_wps_fail_reason_t_WPS_FAIL_REASON_MAX:
system_event_sta_wps_fail_reason_t = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_ap_staconnected_t {
    pub mac: [u8; 6usize],
    pub aid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_ap_stadisconnected_t {
    pub mac: [u8; 6usize],
    pub aid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_ap_probe_req_rx_t {
    pub rssi: xtensa_int,
    pub mac: [u8; 6usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_got_ip6_t {
    pub if_index: tcpip_adapter_if_t,
    pub ip6_info: tcpip_adapter_ip6_info_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union system_event_info_t {
    pub connected: system_event_sta_connected_t,
    pub disconnected: system_event_sta_disconnected_t,
    pub scan_done: system_event_sta_scan_done_t,
    pub auth_change: system_event_sta_authmode_change_t,
    pub got_ip: system_event_sta_got_ip_t,
    pub sta_er_pin: system_event_sta_wps_er_pin_t,
    pub sta_er_fail_reason: system_event_sta_wps_fail_reason_t,
    pub sta_connected: system_event_ap_staconnected_t,
    pub sta_disconnected: system_event_ap_stadisconnected_t,
    pub ap_probereqrecved: system_event_ap_probe_req_rx_t,
    pub got_ip6: system_event_got_ip6_t,
    _bindgen_union_align: [u32; 11usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_event_t {
    pub event_id: system_event_id_t,
    pub event_info: system_event_info_t,
}

pub type system_event_handler_t =
    Option<unsafe extern "C" fn(event: *mut system_event_t) -> esp_err_t>;

pub type system_event_cb_t =
    Option<unsafe extern "C" fn(ctx: *mut xtensa_void, event: *mut system_event_t) -> esp_err_t>;

extern "C" {
    pub fn esp_event_send(event: *mut system_event_t) -> esp_err_t;
    pub fn esp_event_loop_init(cb: system_event_cb_t, ctx: *mut xtensa_void) -> esp_err_t;
}