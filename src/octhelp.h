#include "octave/oct.h"


// std::string

typedef void* stdstring;
extern "C" stdstring stdstring_new(const char* a);


// octave_value_list

extern "C" void octave_value_list_new(octave_value_list* list, int n);
extern "C" int octave_value_list_length(const octave_value_list* list);
extern "C" void octave_value_list_set_value(octave_value_list* list, int n, octave_value* value);
extern "C" void octave_value_list_get_value(octave_value* value, const octave_value_list* list, int n);


// octave_value
// https://octave.org/doxygen/5.1/d3/d35/classoctave__value.html

extern "C" bool octave_value_is_scalar_type(const octave_value* value);
extern "C" bool octave_value_is_string(const octave_value* value);
extern "C" bool octave_value_is_true(const octave_value* value);
extern "C" bool octave_value_is_uint16_type(const octave_value* value);
extern "C" bool octave_value_is_uint32_type(const octave_value* value);
extern "C" bool octave_value_is_uint64_type(const octave_value* value);
extern "C" bool octave_value_is_uint8_type(const octave_value* value);
extern "C" bool octave_value_isinteger(const octave_value* value);
extern "C" bool octave_value_isnull(const octave_value* value);
extern "C" bool octave_value_isnumeric(const octave_value* value);
extern "C" bool octave_value_isreal(const octave_value* value);
