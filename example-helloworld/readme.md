The is a port of the `helloworld.cc` from [A.1.1 Getting Started with Oct-File](https://octave.org/doc/interpreter/Getting-Started-with-Oct_002dFiles.html#Getting-Started-with-Oct_002dFiles). The original ported C++ was:

``` cc
#include <octave/oct.h>

DEFUN_DLD (helloworld, args, nargout,
           "Hello World Help String")
{
  octave_stdout << "Hello World has "
                << args.length () << " input arguments and "
                << nargout << " output arguments.\n";

  // Return empty matrices for any outputs
  octave_value_list retval (nargout);
  for (int i = 0; i < nargout; i++)
    retval(i) = octave_value (Matrix ());

  return retval;
}
```

The `mkoctfile` function compiles it to a `helloworld.oct` file. You can then run it within Octave with:
``` m
helloworld (1, 2, 3)
```
Which prints:
```
Hello World has 3 input arguments and 0 output arguments.
```

You can build this example by doing:
``` sh
cargo build
cp target/debug/libhelloworld.so helloworld.oct
```

Start Octave with `octave`. Then run `helloworld (1, 2, 3)` within Octave.