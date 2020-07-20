#!/bin/env sh
set -e
set -x

bg_header1="miniaudio-sys/bindings.h"
bg_header2="miniaudio-sys/bindings-with-vorbis.h"

bg_rust_target="1.36" 
bg_whitelisted_types="ma_.*"
bg_whitelisted_functions="ma_.*"
bg_whitelisted_variables="(ma|MA)_.*"

bg_flags="--no-layout-tests --ctypes-prefix libc --use-core --size_t-is-usize --impl-debug --no-prepend-enum-name"

bg_options="--rust-target ${bg_rust_target}"

for whitelisted_type in $(echo "$bg_whitelisted_types" | tr ' ' '\n');
do
    bg_options="${bg_options} --whitelist-type ${whitelisted_type}"
done

for whitelisted_function in $(echo "$bg_whitelisted_functions" | tr ' ' '\n');
do
    bg_options="${bg_options} --whitelist-function ${whitelisted_function}"
done

for whitelisted_variable in $(echo "$bg_whitelisted_variables" | tr ' ' '\n');
do
    bg_options="${bg_options} --whitelist-var ${whitelisted_variable}"
done

# echo "$bg_whitelisted_types" | tr ' ' '\n' | while read whitelisted_type
# do
#     bg_options="${bg_options} --whitelist-type ${whitelisted_type}"
# done

mkdir -p "miniaudio-sys/generated-bindings/"

bindgen ${bg_flags} ${bg_options} -o "miniaudio-sys/generated-bindings/bindings.rs" ${bg_header1}
bindgen ${bg_flags} ${bg_options} -o "miniaudio-sys/generated-bindings/bindings-with-vorbis.rs" ${bg_header2}
