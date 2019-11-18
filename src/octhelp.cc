#include "octhelp.h"

extern "C" stdstring stdstring_new(const char* a)
{
    return reinterpret_cast<void*>(new std::string(a));
}

// https://github.com/rust-lang/rust-bindgen/issues/714
extern "C" void octave_value_list_new(octave_value_list* list, int n) {
    new (list) octave_value_list(n);
}