//! Module contains esp_err_t enumeration constants

pub type esp_err_t = i32;
pub const esp_err_t_ESP_OK: esp_err_t = 0;
pub const esp_err_t_ESP_ERR_NO_MEM: esp_err_t = 257;
pub const esp_err_t_ESP_ERR_INVALID_ARG: esp_err_t = 258;
pub const esp_err_t_ESP_ERR_WIFI_CONN: esp_err_t = 12295;
pub const esp_err_t_ESP_ERR_WIFI_NVS: esp_err_t = 12296;
pub const esp_err_t_ESP_ERR_WIFI_PASSWORD: esp_err_t = 12299;