#include "octhelp.h"

// https://github.com/rust-lang/rust-bindgen/issues/714

extern "C" int octave_value_list_create(octave_value_list* list, int n) {
    new (list) octave_value_list(n);
    return 0;
}