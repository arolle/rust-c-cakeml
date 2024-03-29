#![allow(unused_unsafe)]
#![allow(unused_variables)]
extern crate libc;

/// returns an array of at most size len
/// adjusts len accordingly
/// Memory needs to be freed from rust with the function `free_char_array`
#[no_mangle]
pub extern "C"
fn even_numbers_upto (
    input_buf : *const libc::c_uchar,
    len: *mut usize,
) -> *const libc::c_uchar {
    assert!(!len.is_null());
    // turns input_buf into a slice
    let input_buf : &[u8] = unsafe{
        assert!(!input_buf.is_null());
        core::slice::from_raw_parts(input_buf, *len)
    };

    // begin calculation using input_buf
    let mut output_msg : Vec<u8> = input_buf.iter().skip(2).cloned().filter(|x| *x % 2 == 0).collect();
    let mut output : Vec<u8> = vec![];
    // adds message length header
    let b = output_msg.len().to_be_bytes();
    output.extend(&b[6..=7]);
    output.append(&mut output_msg);
    // end calculation
    unsafe {
        assert!(output.len() <= *len);
    }

    let output_buf: Box<[u8]> = output.into_boxed_slice();
    unsafe {
        // adjust len to used buffer length
        *len = output_buf.len();
        Box::into_raw(output_buf) as *const _
    }
}

#[no_mangle]
pub unsafe extern "C"
fn free_char_array (
    buf : *mut libc::c_uchar,
    len : libc::size_t,
) {
    let s = std::slice::from_raw_parts_mut(buf, len);
    Box::from_raw(s as *mut [libc::c_uchar]);
}

/// returns an array of at most size len
/// adjusts len accordingly
/// needs to be freed from c
#[no_mangle]
pub extern "C"
fn even_numbers_upto_malloc_buf (
    input_buf : *const libc::c_uchar,
    len: *mut usize,
) -> *mut libc::c_void {
    assert!(!len.is_null());
    // turns input_buf into a slice
    let input_buf : &[u8] = unsafe{
        assert!(!input_buf.is_null());
        core::slice::from_raw_parts(input_buf, *len)
    };

    // begin calculation using input_buf
    let mut output_msg : Vec<u8> = input_buf.iter().skip(2).cloned().filter(|x| *x % 2 == 0).collect();
    let mut output : Vec<u8> = vec![];
    // adds message length header
    let b = output_msg.len().to_be_bytes();
    output.extend(&b[6..=7]);
    output.append(&mut output_msg);
    // end calculation
    unsafe {
        assert!(output.len() <= *len);
        // adjust len to used buffer length
        *len = output.len();
    }

    slice_to_malloc_buf(&output)
}

// Supposedly by Daniel Henry-Mantilla in May 2021
// on the rust user forums
// https://users.rust-lang.org/t/how-to-return-byte-array-from-rust-function-to-ffi-c/18136/16
fn slice_to_malloc_buf (xs: &'_ [u8]) -> *mut libc::c_void
{
    use ::core::mem::MaybeUninit as MU;

    // allocation with libc allows to free the memory from c
    let ptr = unsafe { ::libc::malloc(xs.len()) };
    if ptr.is_null() { return ptr; }
    let dst = unsafe {
        ::core::slice::from_raw_parts_mut(
            ptr.cast::<MU<u8>>(),
            xs.len(),
    )};
    let src = unsafe {
        ::core::slice::from_raw_parts(
            xs.as_ptr().cast::<MU<u8>>(),
            xs.len(),
    )};
    dst.copy_from_slice(src);
    ptr
}

/// operates directly on the CakeML provided buffers
/// signature of this function in C syntax:
/// ```c
/// extern void ffirust_even_numbers_upto (unsigned char *c, long clen,
///   unsigned char *a, long alen)
/// ```
#[no_mangle]
pub extern "C"
fn ffirust_even_numbers_upto (
    c : *const libc::c_uchar,
    clen: *const libc::c_long,
    a : *mut libc::c_uchar,
    alen: *mut libc::c_long,
) -> () {

/*
    // turns  c  into a slice
    let c_buf : &[u8] = unsafe{
        assert!(!clen.is_null());
        assert!(!c.is_null());
        core::slice::from_raw_parts(c, clen as usize)
    };
*/

    // turns  a  into a mutuable slice
    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html
    let a_buf : &mut [u8] = unsafe{
        assert!(!alen.is_null());
        assert!(!a.is_null());
        core::slice::from_raw_parts_mut(a, alen as usize)
    };

    let from = 2; // filter from index
    let buf = a_buf;

    let buf_len = buf.len();
    let mut len : usize = from;
    for i in from..buf.len() {
        // assert!(from <= len && len <= i && i < buf_len);
        // println!("buf[{i}] = {}, len: {}", buf[i], len);
        if buf[i] % 2 == 0 {
            assert!(len < buf_len);
            buf[len] = buf[i];
            len += 1;
        }
    }
    eprintln!("rs ffirust_even_numbers_upto len: {:?}", len);

    // set length
    assert!(len < 2^8);
    buf[1] = len as u8;
}

