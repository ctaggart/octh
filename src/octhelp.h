#include "octave/oct.h"

typedef void* stdstring;

extern "C" stdstring stdstring_new(const char* a);
// extern "C" void stdstring_new(stdstring* s, const char* a);

extern "C" void octave_value_list_new(octave_value_list* list, int n); 
extern "C" void octave_value_list_set_value(octave_value_list* list, int n, octave_value* value);
extern "C" void octave_value_list_get_value(octave_value* value, octave_value_list* list, int n);

// octave_value
// https://octave.org/doxygen/5.1/d3/d35/classoctave__value.html

extern "C" bool octave_value_is_scalar_type(octave_value* value);
extern "C" bool octave_value_is_uint16_type(octave_value* value);
extern "C" bool octave_value_is_uint32_type(octave_value* value);
extern "C" bool octave_value_is_uint64_type(octave_value* value);
extern "C" bool octave_value_is_uint8_type(octave_value* value);
extern "C" bool octave_value_isinteger(octave_value* value);
extern "C" bool octave_value_isnull(octave_value* value);
extern "C" bool octave_value_isnumeric(octave_value* value);
extern "C" bool octave_value_isreal(octave_value* value);