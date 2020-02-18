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

/**
 * The following functions initialize miniaudio C structs with some
 * "random" values so that we can check for correct alignment in Rust.
 */

void debug_ma_init_format_converter_config(ma_format_converter_config* config) {
    config->formatIn = ma_format_s16;
    config->formatOut = ma_format_s24;
    config->channels = 45;
    config->streamFormatIn = ma_stream_format_pcm;
    config->streamFormatOut = ma_stream_format_pcm;
    config->ditherMode = ma_dither_mode_rectangle;
    config->noSSE2 = 0;
    config->noAVX2 = 0;
    config->noAVX512 = 1;
    config->noNEON = 1;
    config->onRead = NULL;
    config->onReadDeinterleaved = NULL;
    config->pUserData = NULL;
}

void debug_ma_init_format_converter(ma_format_converter* converter) {
    debug_ma_init_format_converter_config(&converter->config);
    converter->useSSE2 = 1;
    converter->useAVX2 = 1;
    converter->useAVX512 = 0;
    converter->useNEON = 0;
    converter->onConvertPCM = NULL;
    converter->onInterleavePCM = NULL;
    converter->onDeinterleavePCM = NULL;
}
