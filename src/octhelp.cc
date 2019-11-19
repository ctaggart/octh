#include "octhelp.h"


// std:string

extern "C" stdstring stdstring_new(const char* a)
{
    return reinterpret_cast<void*>(new std::string(a));
}


// octave_value_list

// https://github.com/rust-lang/rust-bindgen/issues/714
extern "C" void octave_value_list_new(octave_value_list* list, int n) {
    new (list) octave_value_list(n);
}

extern "C" int octave_value_list_length(const octave_value_list* list) {
    return (*list).length();
}

extern "C" void octave_value_list_set_value(octave_value_list* list, int n, octave_value* value) {
    (*list)(n) = *value;
}

extern "C" void octave_value_list_get_value(octave_value* value, const octave_value_list* list, int n){
    (*value) = (*list)(n);
}


// octave_value

extern "C" bool octave_value_is_scalar_type(const octave_value* value){
    return (*value).is_scalar_type();
}

extern "C" bool octave_value_is_string(const octave_value* value){
    return (*value).is_string();
}

extern "C" bool octave_value_is_true(const octave_value* value){
    return (*value).is_true();
}

extern "C" bool octave_value_is_uint16_type(const octave_value* value){
    return (*value).is_uint16_type();
}

extern "C" bool octave_value_is_uint32_type(const octave_value* value){
    return (*value).is_uint32_type();
}

extern "C" bool octave_value_is_uint64_type(const octave_value* value){
    return (*value).is_uint64_type();
}

extern "C" bool octave_value_is_uint8_type(const octave_value* value){
    return (*value).is_uint8_type();
}

extern "C" bool octave_value_isinteger(const octave_value* value){
    return (*value).isinteger();
}

extern "C" bool octave_value_isnull(const octave_value* value){
    return (*value).isnull();
}

extern "C" bool octave_value_isnumeric(const octave_value* value){
    return (*value).isnumeric();
}

extern "C" bool octave_value_isreal(const octave_value* value){
    return (*value).isreal();
}