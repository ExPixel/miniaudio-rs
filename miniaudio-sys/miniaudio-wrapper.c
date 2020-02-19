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

size_t debug_ma_sizeof_pcm_converter_config() {
    return sizeof(ma_pcm_converter_config);
}

size_t debug_ma_sizeof_pcm_converter() {
    return sizeof(ma_pcm_converter);
}

#ifndef MA_NO_DEVICE_IO
size_t debug_ma_sizeof_thread() {
    return sizeof(ma_thread);
}

size_t debug_ma_sizeof_mutex() {
    return sizeof(ma_mutex);
}

size_t debug_ma_sizeof_event() {
    return sizeof(ma_event);
}

size_t debug_ma_sizeof_semaphore() {
    return sizeof(ma_semaphore);
}
#endif

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

void debug_ma_init_channel_router_config(ma_channel_router_config* config) {
    config->channelsIn = 23;
    config->channelsOut = 483;
    config->mixingMode = ma_channel_mix_mode_custom_weights;
    config->noSSE2 = 0;
    config->noAVX2 = 0;
    config->noAVX512 = 1;
    config->noNEON = 1;
    config->onReadDeinterleaved = NULL;
    config->pUserData = NULL;

    // FIXME test the arrays too at some point:
    /* config->channelMapIn = {}; */
    /* config->channelMapOut = {}; */
    /* config->weights = {}; */
}

void debug_ma_init_channel_router(ma_channel_router* router) {
    debug_ma_init_channel_router_config(&router->config);
    router->isPassthrough = 1;
    router->isSimpleShuffle = 0;
    router->isSimpleMonoExpansion = 0;
    router->isStereoToMono = 1;
    router->useSSE2 = 1;
    router->useAVX2 = 1;
    router->useAVX512 = 0;
    router->useNEON = 0;

    // FIXME test the arrays too at some point:
    /* router.shuffleTable = {}; */
}

void debug_ma_init_src_config(ma_src_config* config) {
    config->sampleRateIn = 55;
    config->sampleRateOut = 8734;
    config->channels = 66;
    config->algorithm = ma_src_algorithm_sinc;
    config->neverConsumeEndOfInput = 1;
    config->noSSE2 = 0;
    config->noAVX2 = 0;
    config->noAVX512 = 1;
    config->noNEON = 1;
    config->onReadDeinterleaved = NULL;
    config->pUserData = NULL;
    config->sinc.windowFunction = ma_src_sinc_window_function_rectangular;
    config->sinc.windowWidth = 88;
}

void debug_ma_init_src(ma_src* src) {
    debug_ma_init_src_config(&src->config);

    src->sinc.timeIn = 45.0;
    src->sinc.inputFrameCount = 345;
    src->sinc.windowPosInSamples = 857;

    src->isEndOfInputLoaded = 0;
    src->useSSE2 = 1;
    src->useAVX2 = 1;
    src->useAVX512 = 0;
    src->useNEON = 0;
}
