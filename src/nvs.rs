use crate::error::*;

const ESP_ERR_NVS_BASE: esp_err_t = 0x1100;
const ESP_ERR_NVS_NOT_FOUND: esp_err_t = ESP_ERR_NVS_BASE + 0x02;
const ESP_ERR_NVS_NO_FREE_PAGES: esp_err_t = ESP_ERR_NVS_BASE + 0x0d;


extern "C" {
    pub fn nvs_flash_init() -> esp_err_t;
    pub fn nvs_flash_erase() -> esp_err_t;
}