
pub mod take;
pub mod head;
pub mod cine;

use std::ffi;
use std::ptr;


#[no_mangle]
pub unsafe extern "C" fn get_image_size(name :*const libc::c_char) -> u64 {
    let fname = ffi::CStr::from_ptr(name);
    let filename = fname.to_str().unwrap().to_owned();
    let stepper = cine::FrameStepper::new(&filename).unwrap();
    stepper.bytes_count()
}

#[no_mangle]
pub unsafe extern "C" fn get_image_i(name :*const libc::c_char, index :u64,) -> * const u8 {
    let fname = ffi::CStr::from_ptr(name);
    let filename = fname.to_str().unwrap().to_owned();
    let mut stepper = cine::FrameStepper::new(&filename).unwrap();
    let mut thing = vec![0u8;1];
    for _i in 0..index {
        if stepper.has_next() {
            thing = stepper.next().unwrap();
        } else {
            return ptr::null();
        }
    }
    /* effectively null terminates */
    thing.push(0u8);
    /* internal pointer can be treated as byte string */
    thing.as_ptr()
}
