//! Module contains esp_err_t enumeration constants

pub type esp_err_t = i32;
pub const esp_err_t_ESP_OK: esp_err_t = 0;
pub const esp_err_t_ESP_FAIL: esp_err_t = -1;

pub const esp_err_t_ESP_ERR_COMMON_BASE: esp_err_t = 0x100;
pub const esp_err_t_ESP_ERR_NO_MEM: esp_err_t = esp_err_t_ESP_ERR_COMMON_BASE + 1;
pub const esp_err_t_ESP_ERR_INVALID_ARG: esp_err_t = esp_err_t_ESP_ERR_COMMON_BASE + 2;

pub const esp_err_t_ESP_ERR_WIFI_BASE: esp_err_t = 0x3000;
pub const esp_err_t_ESP_ERR_WIFI_CONN: esp_err_t = esp_err_t_ESP_ERR_WIFI_BASE + 7;
pub const esp_err_t_ESP_ERR_WIFI_NVS: esp_err_t = esp_err_t_ESP_ERR_WIFI_BASE + 8;
pub const esp_err_t_ESP_ERR_WIFI_PASSWORD: esp_err_t = esp_err_t_ESP_ERR_WIFI_BASE + 11;

pub const esp_err_t_ESP_ERR_NVS_BASE: esp_err_t = 0x1100;
pub const esp_err_t_ESP_ERR_NVS_NOT_FOUND: esp_err_t = esp_err_t_ESP_ERR_NVS_BASE + 2;
pub const esp_err_t_ESP_ERR_NVS_NO_FREE_PAGES: esp_err_t = esp_err_t_ESP_ERR_NVS_BASE + 13;