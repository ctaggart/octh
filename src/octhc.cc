#include "octhc.h"

// https://github.com/servo/rust-bindgen/issues/714

extern "C" octave_value_list octave_value_list_create() {
    return octave_value_list();
}