#include "./miniaudio/miniaudio.h"

size_t debug_ma_sizeof_format_converter_config() {
    return sizeof(ma_format_converter_config);
}

size_t debug_ma_sizeof_format_converter() {
    return sizeof(ma_format_converter);
}

size_t debug_ma_sizeof_channel_router_config() {
    return sizeof(ma_channel_router_config);
}

size_t debug_ma_sizeof_channel_router() {
    return sizeof(ma_channel_router);
}

size_t debug_ma_sizeof_src_config_sinc() {
    return sizeof(ma_src_config_sinc);
}

size_t debug_ma_sizeof_src_config() {
    return sizeof(ma_src_config);
}

size_t debug_ma_sizeof_src() {
    return sizeof(ma_src);
}

// generate bit flag checks using js or something :P
