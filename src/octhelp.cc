#include "octhelp.h"

// https://github.com/rust-lang/rust-bindgen/issues/714

extern "C" octave_value_list octave_value_list_create(int n) {
    return octave_value_list(n);
}