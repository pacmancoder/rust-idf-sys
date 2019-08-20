#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type xtensa_int = isize;
pub type xtensa_void = ::core::ffi::c_void;

pub type esp_err_t = i32;
pub const esp_err_t_ESP_OK: esp_err_t = 0;

pub type wifi_mode_t = u32;
pub const wifi_mode_t_WIFI_MODE_NULL: wifi_mode_t = 0;
pub const wifi_mode_t_WIFI_MODE_STA: wifi_mode_t = 1;
pub const wifi_mode_t_WIFI_MODE_AP: wifi_mode_t = 2;
pub const wifi_mode_t_WIFI_MODE_APSTA: wifi_mode_t = 3;
pub const wifi_mode_t_WIFI_MODE_MAX: wifi_mode_t = 4;

pub type esp_interface_t = u32;
pub use self::esp_interface_t as wifi_interface_t;
pub const esp_interface_t_ESP_IF_WIFI_STA: esp_interface_t = 0;
pub const esp_interface_t_ESP_IF_WIFI_AP: esp_interface_t = 1;
pub const esp_interface_t_ESP_IF_MAX: esp_interface_t = 2;

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
pub struct ip4_addr_t {
    pub addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpip_adapter_ip_info_t {
    pub ip: ip4_addr_t,
    pub netmask: ip4_addr_t,
    pub gw: ip4_addr_t,
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

pub type tcpip_adapter_if_t = u32;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_STA: tcpip_adapter_if_t = 0;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_AP: tcpip_adapter_if_t = 1;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_ETH: tcpip_adapter_if_t = 2;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_MAX: tcpip_adapter_if_t = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpip_adapter_ip6_info_ip6_addr_t {
    pub addr: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpip_adapter_ip6_info_t {
    pub ip: tcpip_adapter_ip6_info_ip6_addr_t,
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
    ::core::option::Option<unsafe extern "C" fn(event: *mut system_event_t) -> esp_err_t>;

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
    pub fn esp_event_send(event: *mut system_event_t) -> esp_err_t;

    pub fn esp_wifi_init(config: *const wifi_init_config_t) -> esp_err_t;

    pub fn esp_wifi_set_mode(mode: wifi_mode_t) -> esp_err_t;
    pub fn esp_wifi_set_config(interface: wifi_interface_t, conf: *mut wifi_config_t) -> esp_err_t;
    pub fn esp_wifi_start() -> esp_err_t;
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
