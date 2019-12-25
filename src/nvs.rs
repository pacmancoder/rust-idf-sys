use crate::error::*;

extern "C" {
    pub fn nvs_flash_init() -> esp_err_t;
    pub fn nvs_flash_erase() -> esp_err_t;
}