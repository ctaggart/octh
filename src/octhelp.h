#include "octave/oct.h"

typedef void* stdstring;

extern "C" stdstring stdstring_new(const char* a);

extern "C" void octave_value_list_new(octave_value_list* list, int n); 