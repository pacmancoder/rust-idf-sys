//! Module contains structures, enumerations and functions related to network adapter interface

pub type esp_interface_t = u32;
pub const esp_interface_t_ESP_IF_WIFI_STA: esp_interface_t = 0;
pub const esp_interface_t_ESP_IF_WIFI_AP: esp_interface_t = 1;
pub const esp_interface_t_ESP_IF_MAX: esp_interface_t = 2;

pub type tcpip_adapter_if_t = u32;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_STA: tcpip_adapter_if_t = 0;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_AP: tcpip_adapter_if_t = 1;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_ETH: tcpip_adapter_if_t = 2;
pub const tcpip_adapter_if_t_TCPIP_ADAPTER_IF_MAX: tcpip_adapter_if_t = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpip_adapter_ip6_info_t_anonymous_type_0 {
    pub addr: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpip_adapter_ip6_info_t {
    pub ip: tcpip_adapter_ip6_info_t_anonymous_type_0,
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