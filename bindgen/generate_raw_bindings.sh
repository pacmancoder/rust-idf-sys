#!/bin/bash

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo $CURRENT_DIR

# $IDF_PATH/components/esp8266/include/esp_event_loop.h \

bindgen \
    --no-layout-tests \
    --use-core \
    --no-doc-comments \
    $IDF_PATH/components/esp8266/include/esp_wifi.h \
    -- \
    -I $IDF_PATH/components/freertos/port/esp8266/include \
    -I $IDF_PATH/components/freertos/port/esp8266/include/freertos \
    -I $IDF_PATH/components/freertos/include \
    -I $IDF_PATH/components/freertos/include/freertos/private \
    -I $IDF_PATH/components/esp8266/include \
    -I $IDF_PATH/components/heap/include \
    -I $IDF_PATH/components/heap/port/esp8266/include \
    -I $IDF_PATH/components/lwip/lwip/src/include \
    -I $IDF_PATH/components/lwip/port/esp8266/include \
    -I $IDF_PATH/components/lwip/include/lwip/apps \
    -I $IDF_PATH/components/tcpip_adapter/include \
    -I $CURRENT_DIR \
    > esp8266.rs
