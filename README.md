# An example for using Rust and C as a CakeML FFI

Today the [CakeML language](https://cakeml.org) provides a library of
approximately 370 operations on common datastructures like lists, hashmaps,
integer and floating point numbers, and a CakeML program receives and sends
input and output over `TextIO.stdIn`, `TextIO.stdOut`, `TextIO.stdErr` or by
reading and writing from file.

The foreign function interface of the CakeML language enables to extend
a CakeML program with functionality provided by external libraries, such as
networking or crypto.

This repository offers an example of extending CakeML with functionality
through Rust. In the program `even_numbers.cml` numbers are written to a buffer
`buf` that is filtered for only the even numbers.
From CakeML a foreign C function is called as:
```sml
#(even_numbers_upto) "" buf
```
The corresponding C-function `ffieven_numbers_upto` operates on the supplied
buffer is implemented in `basis_ffi.c` with the signature, where `c` points to
the string supplied in CakeML and `a` to the buffer:
```c
void ffieven_numbers_upto (unsigned char *c, long clen, unsigned char *a, long alen) {
```
This function wraps a call to a Rust function from `src/lib.rs` with the following signature, that returns a buffer back to C.
```rs
pub extern "C" fn even_numbers_upto (
    input_buf : *const libc::c_uchar,
    len: *mut usize,
) -> *const libc::c_uchar {
```

The trace of calls looks like:

```
cml: #(even_numbers_upto)
	c: ffieven_numbers_upto
		rs: even_numbers_upto
		rs: return char msg *
	c: free msg
	c: return void
cml: <next instruction>
```

## Usage

For compilation we require a C compiler, `cargo` for Rust and the
[CakeML compiler](https://github.com/CakeML/cakeml/releases) located at
`$CAKEDIR/cake`. Running `make` will build `even_numbers.cml` and run it.

Example run (with added boundaries `|`):

```
$ make
env LD_LIBRARY_PATH=target/debug ./even_numbers
cml even_numbers_upto: buffer of length 14 plus header
cml even_numbers_upto: 0 14 | 0 1 2 3 4 5 6 7 8 9 10 11 12 13
c ffieven_numbers_upto: available CakeML buffer length 16
c ffieven_numbers_upto: used buffer length 9
cml even_numbers_upto: written message length 7
cml even_numbers_upto: 0 7 | 0 2 4 6 8 10 12 | 7 8 9 10 11 12 13
[0; 2; 4; 6; 8]
```

