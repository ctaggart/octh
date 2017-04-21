# Rust bindings for GNU Octave oct.h

The goal is to be able to [extend](https://www.gnu.org/software/octave/doc/interpreter/External-Code-Interface.html) [GNU Octave](https://www.gnu.org/software/octave/) with functions built with Rust instead of C++. Octave functions can be built using the C++ oct.h. A great milestone will be to be able to build and run this [hello world](https://www.gnu.org/software/octave/doc/interpreter/Getting-Started-with-Oct_002dFiles.html#Getting-Started-with-Oct_002dFiles) in Rust instead.

``` cpp
#include <octave/oct.h>

DEFUN_DLD (helloworld, args, nargout,
           "Hello World Help String")
{
  octave_stdout << "Hello World has "
                << args.length () << " input arguments and "
                << nargout << " output arguments.\n";

  return octave_value_list ();
}
```
I could use some help with taming bindgen. See [the issues](https://github.com/ctaggart/octh/issues) for a current list.